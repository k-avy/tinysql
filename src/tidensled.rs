// use tide::Request;
// use sled::Db;

//  pub async fn tidendb() -> tide::Result<()> {
//     let mut app = tide::new();

//     app.at("/").get(|_req: Request<()>| async move {
//         // Open the Sled database
//         let db = sled::open("my_db")?;

//         // Insert a key-value pair into the database
//         db.insert("key", "value")?;

//         // Retrieve the value
//         let value = db.get("key")?.unwrap_or_else(|| sled::IVec::from("not found"));

//         // Close the database by dropping it
//         db.flush()?;
//         drop(db);

//         // Respond with the retrieved value
//         Ok(format!("Value: {:?}", String::from_utf8_lossy(&value)))
//     });

//     app.listen("127.0.0.1:8081").await?;
//     Ok(())
// }