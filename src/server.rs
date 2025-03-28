use std::convert::Infallible;
use std::net::IpAddr;
use std::str::FromStr;
use warp::{Filter, Reply};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

#[derive(Debug, Deserialize, Serialize, Clone, Eq, Hash, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PadType {
    Right,
    Left,
    Shoot,
}


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PressurePadData {
    pub timestamp: Option<u64>,
    pub pressure: f32,
    pub pad_type: PadType,
}

pub async fn handle_pressure_data(data: PressurePadData) -> Result<impl Reply, Infallible> {
    println!("  Pressure: {}", data.pressure);
    println!("  Pad Type: {:?}", data.pad_type);

    Ok(warp::reply::json(&data))
}

pub fn routes(tx: mpsc::Sender<PressurePadData>) -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let pressure_route = warp::path("pressure")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(move |data: PressurePadData| {
            let tx = tx.clone();
            async move {
                if tx.send(data.clone()).await.is_err() {
                    eprintln!("Failed to send pressure data to the main task");
                }
                Ok::<_, warp::Rejection>(warp::reply::json(&data))
            }
        });

    let health_route = warp::path("health")
        .and(warp::get())
        .map(|| "Server is running!");

    pressure_route.or(health_route)
}

pub async fn run_server(tx: mpsc::Sender<PressurePadData>) {
    println!("server running");
    let ip = IpAddr::from_str("192.168.1.113").unwrap();
    let port = 8080;

    println!("Starting Arduino server on {}:{}", ip, port);

    let routes = routes(tx);

    warp::serve(routes)
        .run((ip, port))
        .await;
}



