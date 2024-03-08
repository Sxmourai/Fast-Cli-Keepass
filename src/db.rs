use keepass::Database;

pub fn get_db() -> Database {
    let mut file = std::fs::File::open("tests/resources/test_db_with_password.kdbx").unwrap();
    let key = keepass::DatabaseKey::new().with_password("demopass");
    let db = keepass::Database::open(&mut file, key).unwrap(); //TODO Robust Error handling
    db
}