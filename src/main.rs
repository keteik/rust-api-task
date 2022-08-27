use actix_web::{get, App, HttpServer, web, Responder, Result};
use serde::{Deserialize, Serialize};
use rand::{Rng, rngs::ThreadRng};

#[derive(Deserialize)]
struct FuelInfo {
    distance: u32,
    yearOfProduction: u32,
    fuelUsagePer100KM: u32
}

#[derive(Deserialize)]
struct CarInfo {
    VIN: String
}

#[derive(Serialize)]
struct FuelUsage {
    fuelUsage: f32
}

#[derive(Serialize)]
struct FailProbability {
    failProbability: String
}

#[get("/calculateDisselUsageForDistance")]
async fn fuel_consuption(req_params: web::Query<FuelInfo>) -> Result<impl Responder> {
    let fuel_usage: f32 = req_params.distance as f32 * req_params.fuelUsagePer100KM as f32 / 100.0;
    let response: FuelUsage = FuelUsage {
        fuelUsage: fuel_usage
    };
    Ok(web::Json(response))
}

#[get("/probabilityOfUnitInjectorFail")]
async fn test(_req_params: web::Query<CarInfo>) -> Result<impl Responder> {
    let mut rng: ThreadRng = rand::thread_rng();
    let fail_probability: f32 = rng.gen_range(0..100) as f32 / 100.0;
    let response: FailProbability = FailProbability {
        failProbability: fail_probability.to_string()
    };
    Ok(web::Json(response))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        App::new()
            .service(fuel_consuption)
            .service(test)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
