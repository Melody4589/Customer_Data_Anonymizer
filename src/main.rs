#[macro_use] extern crate rocket;

use rocket::serde::{Serialize, Deserialize, json::Json};
use rand::{Rng, distributions::Alphanumeric};
use rocket::fs::{FileServer, relative};

#[derive(Debug, Serialize, Deserialize)]
struct CustomerData {
    name: String,
    email: String,
    phone: String,
}

#[derive(Serialize, Deserialize)]
struct AnonymizedData {
    id: String,
    anonymized_name: String,
    anonymized_email: String,
    anonymized_phone: String,
}

#[post("/anonymize", format = "json", data = "<data>")]
fn anonymize(data: Json<CustomerData>) -> Json<AnonymizedData> {
    let mut rng = rand::thread_rng();
    let id: String = (0..10).map(|_| rng.sample(Alphanumeric) as char).collect();

    // Anonymize each field
    let anonymized_name: String = (0..5).map(|_| rng.sample(Alphanumeric) as char).collect();
    let anonymized_email: String = format!("{}@example.com", (0..5).map(|_| rng.sample(Alphanumeric) as char).collect::<String>());
    let anonymized_phone: String = (0..10).map(|_| rng.sample(Alphanumeric) as char).collect();

    println!("Received data: {:?}", data);

    Json(AnonymizedData { 
        id,
        anonymized_name,
        anonymized_email,
        anonymized_phone,
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![anonymize])
        .mount("/", FileServer::from(relative!("static")))
}
