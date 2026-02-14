//! Utoipa utilities.

use std::collections::BTreeMap;

use http::StatusCode;
use itertools::Itertools;
use utoipa::openapi::{
    Content, ContentBuilder, OneOfBuilder, RefOr, Response, ResponseBuilder, Schema,
};

/// Merge multiple [`BTreeMap<String, RefOr<Response>>`] into a single [`BTreeMap<String, RefOr<Response>>`].
pub fn merge_responses(
    responses: impl Iterator<Item = BTreeMap<String, RefOr<Response>>>,
) -> BTreeMap<String, RefOr<Response>> {
    responses
        .flatten()
        .chunk_by(|(code, _)| code.clone())
        .into_iter()
        .map(|(code, chunk)| {
            let response = merge_response(
                StatusCode::from_bytes(code.as_bytes()).expect("valid status code"),
                chunk.map(|(_, response)| response),
            );

            (code, RefOr::T(response))
        })
        .collect()
}

/// Merge multiple [`RefOr<Response>`] into a single [`Response`].
fn merge_response(code: StatusCode, responses: impl Iterator<Item = RefOr<Response>>) -> Response {
    let responses = responses.filter_map(|response| match response {
        RefOr::Ref(_) => None,
        RefOr::T(response) => Some(response),
    });

    let mut builder = ResponseBuilder::new();

    if let Some(canonical_reason) = code.canonical_reason() {
        builder = builder.description(canonical_reason)
    }

    builder = responses
        .into_iter()
        .flat_map(|response| response.content)
        .chunk_by(|(content_type, _)| content_type.clone())
        .into_iter()
        .fold(builder, |builder, (content_type, chunk)| {
            let content = merge_content(
                chunk
                    .map(|(_, content)| content)
                    .collect::<Vec<_>>()
                    .into_iter(),
            );

            if let Some(content) = content {
                builder.content(content_type, content)
            } else {
                builder
            }
        });

    // TODO: Merge headers, extensions, links.

    builder.build()
}

fn merge_content(mut contents: impl ExactSizeIterator<Item = Content>) -> Option<Content> {
    if contents.len() <= 1 {
        return contents.next();
    }

    let mut builder = ContentBuilder::new();
    let mut one_of_builder = OneOfBuilder::new();

    for content in contents {
        if content.example.is_some() {
            // TODO: Error that this is unsupported.
        }

        builder = builder.examples_from_iter(content.examples);

        for (name, encoding) in content.encoding {
            builder = builder.encoding(name, encoding);
        }

        if let Some(schema) = content.schema {
            one_of_builder = merge_into_one_of_builder(one_of_builder, schema);
        }

        // TODO: Merge extensions.
    }

    let one_of = one_of_builder.build();
    if !one_of.items.is_empty() {
        builder = builder.schema(Some(Schema::from(one_of)));
    }

    Some(builder.build())
}

fn merge_into_one_of_builder(mut builder: OneOfBuilder, schema: RefOr<Schema>) -> OneOfBuilder {
    if let RefOr::T(schema) = schema {
        if let Schema::OneOf(one_of) = schema {
            for item in one_of.items {
                builder = merge_into_one_of_builder(builder, item);
            }

            builder
        } else {
            builder.item(RefOr::T(schema))
        }
    } else {
        builder.item(schema)
    }
}
