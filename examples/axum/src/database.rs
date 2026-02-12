use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use anyhow::{Result, anyhow};
use uuid::Uuid;

use crate::user::schemas::User;

#[derive(Clone, Default)]
pub struct Database {
    users: Arc<Mutex<HashMap<Uuid, User>>>,
}

impl Database {
    pub fn user(&self, id: Uuid) -> Result<Option<User>> {
        let users = self.users.lock().map_err(|err| anyhow!("{err}"))?;

        Ok(users.get(&id).cloned())
    }

    pub fn insert_user(&self, user: User) -> Result<User> {
        let mut users = self.users.lock().map_err(|err| anyhow!("{err}"))?;

        users.insert(user.id, user.clone());

        Ok(user)
    }

    pub fn remove_user(&self, user: User) -> Result<()> {
        let mut users = self.users.lock().map_err(|err| anyhow!("{err}"))?;

        users.remove(&user.id);

        Ok(())
    }
}
