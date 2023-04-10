use mini_redis::{client, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    uuid: String,
    name: String,
    age: u8,
    city: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = client::connect("127.0.0.1:6379").await?;

    let user = User {
        uuid: String::from("472495dc-1d21-4c72-9bbe-9ca65ae06908"),
        name: String::from("John"),
        city: String::from("New York"),
        age: 30,
    };

    let user_serialized = serde_json::to_string(&user).unwrap();
    let user_in_bytes = user_serialized.as_bytes().to_vec();

    client.set(&user.uuid, user_in_bytes.into()).await?;

    let result = client.get(&user.uuid).await?;

    if let Some(data) = result {
        let user_deserialized: User = serde_json::from_slice(&data)?;
        println!("redis key {:?}\n {:?}", user.uuid, user_deserialized);
    } else {
        println!("Key not found in Redis");
    }

    Ok(())
}
