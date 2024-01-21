use crate::{ambient_weather_report::AmbientWeatherReport, Reports};
use actix_web::{
    get,
    middleware::Logger,
    web::{Data, Query},
    App, HttpResponse, HttpServer, Responder,
};
use std::{net::SocketAddr, ops::Deref};
use tracing::error;

pub struct Server {
    address: SocketAddr,
    reports: Data<Reports>,
}

impl Server {
    pub fn new(address: SocketAddr) -> Self {
        Self {
            address,
            reports: Data::new(Reports::new()),
        }
    }

    pub async fn start(self) -> std::io::Result<()> {
        HttpServer::new(move || {
            App::new()
                .app_data(self.reports.clone())
                .service(root)
                .service(ambient_weather_upload)
                .service(get_metrics)
                .wrap(Logger::default())
        })
        .bind(self.address)?
        .run()
        .await
    }
}

#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::NotFound()
}

#[get("/ambient-weather/upload")]
async fn ambient_weather_upload(
    reports: Data<Reports>,
    report: Query<AmbientWeatherReport>,
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
async fn get_metrics(reports: Data<Reports>) -> impl Responder {
    match reports.encode() {
        Ok(report) => HttpResponse::Ok().body(report),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}
