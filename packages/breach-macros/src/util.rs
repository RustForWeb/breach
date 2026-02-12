pub enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<L, R> Either<L, R> {
    #[expect(unused)]
    pub fn is_left(&self) -> bool {
        match &self {
            Either::Left(_) => true,
            Either::Right(_) => false,
        }
    }

    pub fn is_right(&self) -> bool {
        match &self {
            Either::Left(_) => false,
            Either::Right(_) => true,
        }
    }
}
