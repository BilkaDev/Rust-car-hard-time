#[macro_use] extern crate rocket;

use rocket::serde::json::{json, Value};


#[get("/calculateDisselUsageForDistance/<distance>/<year_of_production>/<fuel_usage_per_100km>")]
fn dissel_usage(distance: u32,year_of_production: u32, fuel_usage_per_100km: u32) -> Value {
    let fuel_usage = distance as f32 / fuel_usage_per_100km as f32;
    json!({"fuelUsage": fuel_usage})
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![dissel_usage])
}