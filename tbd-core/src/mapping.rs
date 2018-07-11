use db_tool;
use rusqlite::Row;

pub type SqlType<'row> = &'row Row<'row, 'row>;

pub trait Mapping<'row, T>
where
    T: From<SqlType<'row>>,
{
    fn from_sql(s: SqlType<'row>) -> T {
        s.into()
    }
}

/// This is an in-memory representation of a user
#[derive(Debug)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub age: i8,
}

impl<'a> From<SqlType<'a>> for User {
    fn from(s: SqlType) -> Self {
        Self {
            id: s.get(0),
            name: s.get(1),
            age: s.get(2),
        }
    }
}

/// Represent a mapping for a user
pub struct UserMapping;
impl<'row> Mapping<'row, User> for UserMapping {}

pub fn test_this() {
    let conn = db_tool::setup_db();
    db_tool::create_dummy_data(&conn);

    /* Query the database */
    let mut stmt = conn.prepare("SELECT id, name, age FROM person").unwrap();

    let users = stmt
        .query_map(&[], |row| UserMapping::from_sql(row))
        .unwrap()
        .filter_map(|u| u.ok())
        .collect::<Vec<User>>();

    println!("{:#?}", users);
}
