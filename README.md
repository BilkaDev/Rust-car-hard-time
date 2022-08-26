
# Rust Car for hard time

A short application that calculates fuel consumption and probability of vehicle damage.


## Installation

Install my-project with cargo

```bash
  cargo check
  cargo run
```
    
## 🛠 Skills
Rust.


## Tech Stack

**Server:** Rust, Rocket version = 0.5.0-rc.2.




## API Reference

#### Get consumption on specified distance.

```http
    GET /calculateDisselUsageForDistance/<distance>/<year_of_production>/<fuel_usage_per_100km>

```

| Parameter | Type     | Description                       |
| :-------- | :------- | :-------------------------------- |
| `distance`      | `integer number` | **Required**. The distance between point A and B |
| `year_of_production`      | `integer number` | **Required**. Year of production of the car |
| `fuel_usage_per_100km`      | `integer number` | **Required**. Average fuel consumption of the car per 100km |

#### Get percent chance that engine will fail.

```http
    GET /probabilityOfUnitInjectorFail/<vin>

```

| Parameter | Type     | Description                       |
| :-------- | :------- | :-------------------------------- |
| `vin`      | `string` | **Required**. Vehicle vin number |
