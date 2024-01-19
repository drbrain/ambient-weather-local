mod ambient_weather_report;
mod report;
mod reports;

use crate::{ambient_weather_report::AmbientWeatherReport, reports::Reports};
use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use env_logger::Env;
use std::ops::Deref;
use tracing::error;

#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::NotFound()
}

#[get("/ambient-weather/upload")]
async fn ambient_weather_upload(
    reports: web::Data<Reports>,
    report: web::Query<AmbientWeatherReport>,
) -> impl Responder {
    match report.deref().try_into() {
        Ok(report) => {
            reports.add_report(report);
        }
        Err(e) => {
            error!(?e);

            return HttpResponse::BadRequest();
        }
    };

    HttpResponse::Ok()
}

#[get("/metrics")]
async fn metrics(reports: web::Data<Reports>) -> impl Responder {
    HttpResponse::Ok().body(reports.metrics())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let reports = web::Data::new(Reports::default());

    HttpServer::new(move || {
        App::new()
            .app_data(reports.clone())
            .service(root)
            .service(ambient_weather_upload)
            .service(metrics)
            .wrap(Logger::default())
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
