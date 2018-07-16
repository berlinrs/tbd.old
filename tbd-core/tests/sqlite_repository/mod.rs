use futures;
use futures::Stream;
use sqlite;
use sqlite::Value;
use tbd_core::repository::*;

pub struct SqliteRepository {
    pub connection: sqlite::Connection,
}

impl Repository for SqliteRepository {
    fn query<M: 'static>(&self, q: Query) -> Box<Stream<Item = M, Error = String>>
    where
        Self: Queryable<M>,
    {
        <Self as Queryable<M>>::query(self, q)
    }
}

#[derive(Debug)]
pub struct User {
    name: String,
    age: i8,
}

impl Queryable<User> for SqliteRepository {
    fn query(&self, _q: Query) -> Box<Stream<Item = User, Error = Error>> {
        let mut cursor = self.connection
            .prepare("SELECT * FROM users WHERE age > ?")
            .unwrap()
            .cursor();

        cursor.bind(&[Value::Integer(50)]).unwrap();

        let mut results = Vec::new();

        while let Some(row) = cursor.next().unwrap() {
            results.push(User {
                name: row[0].as_string().unwrap().into(),
                age: row[1].as_integer().unwrap() as i8,
            });
        }

        Box::new(futures::stream::iter_ok(results.into_iter()))
    }
}
