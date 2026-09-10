#[macro_use]
extern crate rocket;

use rand::Rng;
use rocket::serde::json::Json;
use rocket_cors::CorsOptions;
use serde::Serialize;

#[derive(Serialize)]
struct WeatherForecast {
    location: String,
    temperature: f32,
    description: String,
}

#[get("/weather/<location>")]
fn weather(location: String) -> Json<WeatherForecast> {
    let mut rng = rand::rng();
    let temperature = rng.random_range(-10.0..35.0);
    let descriptions = ["Sunny", "Cloudy", "Rainy", "Windy", "Snowy"];
    let description = descriptions[rng.random_range(0..descriptions.len())].to_string();

    Json(WeatherForecast {
        location,
        temperature,
        description,
    })
}

#[launch]
fn rocket() -> _ {
    let cors = CorsOptions::default()
        .to_cors()
        .expect("error creating CORS fairing");

    rocket::build().attach(cors).mount("/", routes![weather])
}
