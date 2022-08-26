#[macro_use] extern crate rocket;

use rocket::serde::json::{json, Value};
use regex::Regex;
use rand::Rng;
use rust_decimal::prelude::*;

#[get("/calculateDisselUsageForDistance/<distance>/<year_of_production>/<fuel_usage_per_100km>")]
fn dissel_usage(distance: u32,year_of_production: u32, fuel_usage_per_100km: u32) -> Value {
    let fuel_usage = distance as f32 / fuel_usage_per_100km as f32;
    json!({"fuelUsage": fuel_usage})
}

#[get("/probabilityOfUnitInjectorFail/<vin>")]
fn probability_of_fail(vin: &str) -> Value {
   let regex = Regex::new("^(?i)[A-HJ-NPR-Z0-9]{17}$").unwrap();
   let check_vin = regex.is_match(vin);
   let mut rng = rand::thread_rng();
    if (check_vin) {
        let fail_probability = Decimal::new(rng.gen_range(0..100), 2);
        json!({
            "failProbability": fail_probability,
        })
    } else {
        json!({
            "message": "Incorrect vin number!",
        }) 
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![dissel_usage, probability_of_fail])
}