#[macro_use] extern crate rocket;

use rocket::serde::json::Json;
use serde::Serialize;
use rand::Rng;

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
    rocket::build().mount("/", routes![weather])
}
