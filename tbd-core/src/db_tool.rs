use mapping::User;
use rusqlite::Connection;

pub fn setup_db() -> Connection {
    let conn = Connection::open_in_memory().unwrap();
    conn.execute(
        "CREATE TABLE person (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL,
                  age             INTEGER
                  )",
        &[],
    ).unwrap();
    conn
}

pub fn create_dummy_data(conn: &Connection) {
    let alice = User {
        id: 0,
        name: "Alice".to_string(),
        age: 25,
    };

    let bob = User {
        id: 0,
        name: "Bob".to_string(),
        age: 21,
    };

    conn.execute(
        "INSERT INTO person (name, age) VALUES (?1, ?2)",
        &[&alice.name, &alice.age],
    ).unwrap();

    conn.execute(
        "INSERT INTO person (name, age) VALUES (?1, ?2)",
        &[&bob.name, &bob.age],
    ).unwrap();
}
