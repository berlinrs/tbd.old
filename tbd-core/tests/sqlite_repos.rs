extern crate futures;
extern crate sqlite;
extern crate tbd_core;

mod sqlite_repository;

use sqlite_repository::*;

use tbd_core::repository::{Query, Repository};

use futures::{Future, Stream};

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
        .wait()
        .unwrap();
}
