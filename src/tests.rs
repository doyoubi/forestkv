use std::str;
use super::*;

#[test]
fn test_fdb() {
    let mut db = Fdb::open("/tmp/test_fdb").unwrap();
    db.set("foo".as_bytes(), "bar".as_bytes()).unwrap();
    let value = db.get("foo".as_bytes()).unwrap().unwrap();
    assert_eq!(str::from_utf8(&value).unwrap(), "bar");
    let none = db.get("key_not_found".as_bytes()).unwrap();
    assert!(none.is_none());
}
