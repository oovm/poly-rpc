use sled_typed::{Database, DiskMap};
use std::path::PathBuf;

#[test]
fn ready() {
    println!("it works!")
}

#[tokio::test]
async fn test_files() {
    let path = PathBuf::from("sqlite");
    let db = Database::open(&path).unwrap();
    let map: DiskMap<String, String> = db.document("").unwrap();
    let key = "key".to_string();
    let value = "value".to_string();
    map.insert(key, value);
    println!("{:?}", map.get("key".to_string()));

    // file_db.test().await.unwrap()
}
