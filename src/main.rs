#[macro_use] extern crate rocket;

use rocket::serde::json::{json, Value};


#[get("/calculateDisselUsageForDistance/<distance>/<year_of_production>/<fuel_usage_per_100km>")]
fn dissel_usage(distance: u64,year_of_production: u16, fuel_usage_per_100km: u8) -> Value {
    json!({"test": "test"})
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![dissel_usage])
}