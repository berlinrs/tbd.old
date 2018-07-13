use futures::{Future, Stream};

pub type Error = String;

pub trait Queryable<Model> {
    fn query(&self, q: Query) -> Box<Stream<Item = Model, Error = String>>;
}

pub enum Query {
    All,
}

pub trait KVRespository {
    type Error;

    fn get<M: 'static>(pk: i64) -> Box<Future<Item = M, Error = Self::Error>>;
}

pub trait Repository {
    fn query<M: 'static>(&self, q: Query) -> Box<Stream<Item = M, Error = Error>>
    where
        Self: Queryable<M>;
}

