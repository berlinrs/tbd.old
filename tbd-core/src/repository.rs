use futures::{Future, Stream};
use sqlite;

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

pub struct SqliteRepository {
    connection: sqlite::Connection,
}

impl Repository for SqliteRepository {
    fn query<M: 'static>(&self, q: Query) -> Box<Stream<Item = M, Error = String>>
    where
        Self: Queryable<M>,
    {
        <Self as Queryable<M>>::query(self, q)
    }
}

mod impls {
    use futures;
    use futures::Stream;
    use sqlite::Value;
    use super::{Error, Query, Queryable, SqliteRepository};

    #[derive(Debug)]
    struct User {
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
}

#[cfg(test)]
mod test {
    use super::{Error, Query, Queryable, Repository, SqliteRepository};
    use futures;
    use futures::{Future, Stream};
    use sqlite;
    use sqlite::Value;


    #[test]
    fn test() {
        let connection = sqlite::open(":memory:").unwrap();

        connection
            .execute(
                "
                CREATE TABLE users (name TEXT, age INTEGER);
                INSERT INTO users (name, age) VALUES ('Alice', 42);
                INSERT INTO users (name, age) VALUES ('Bob', 69);
                ",
            )
            .unwrap();

        let repo = SqliteRepository {
            connection: connection,
        };
        let result = repo.query::<User>(Query::All);

        result
            .for_each(|u| {
                println!("{:?}", u);
                Ok(())
            })
            .wait();
    }
}
