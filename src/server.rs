use std::convert::Infallible;
use std::net::IpAddr;
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use warp::Filter;
use serde::{Deserialize, Serialize};

//this code creates a Warp server this is where the microcontrollers connect and pass messages (E.g. shoot, left, right)

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DifficultyConfig {
    pub difficulty_percentage: f32,
    pub drop_percentage: f32,
}

pub fn routes(
    tx: mpsc::Sender<PressurePadData>,
    difficulty_config: Arc<RwLock<DifficultyConfig>>,
) -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
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

    let difficulty_config_clone = difficulty_config.clone();
    let difficulty_route = warp::path("difficulty")
        .and(warp::get())
        .and_then(move || {
            let config = difficulty_config_clone.clone();
            async move {
                let difficulty_data = config.read().await.clone();
                Ok::<_, warp::Rejection>(warp::reply::json(&difficulty_data))
            }
        });

    let update_difficulty_route = warp::path("difficulty")
        .and(warp::put())
        .and(warp::body::json())
        .and(with_difficulty_config(difficulty_config.clone()))
        .and_then(
            |new_config: DifficultyConfig, config: Arc<RwLock<DifficultyConfig>>| async move {
                let mut validated_config = new_config.clone();

                validated_config.difficulty_percentage = validated_config.difficulty_percentage.clamp(0.0, 100.0);
                validated_config.drop_percentage = validated_config.drop_percentage.clamp(0.0, 100.0);

                let mut config_write = config.write().await;
                *config_write = validated_config.clone();

                println!("Updated difficulty percentage to: {}%", config_write.difficulty_percentage);
                println!("Updated drop percentage to: {}%", config_write.drop_percentage);

                Ok::<_, warp::Rejection>(warp::reply::json(&validated_config))
            },
        );

    pressure_route
        .or(health_route)
        .or(difficulty_route)
        .or(update_difficulty_route)
}

fn with_difficulty_config(
    config: Arc<RwLock<DifficultyConfig>>,
) -> impl Filter<Extract = (Arc<RwLock<DifficultyConfig>>,), Error = Infallible> + Clone {
    warp::any().map(move || config.clone())
}

pub async fn run_server(tx: mpsc::Sender<PressurePadData>) {
    println!("server running");
    let ip = IpAddr::from_str("192.168.1.113").unwrap();
    let port = 8080;
    println!("Starting Arduino server on {}:{}", ip, port);

    let difficulty_config = Arc::new(RwLock::new(DifficultyConfig {
        difficulty_percentage: 50.0,
        drop_percentage: 10.0,
    }));

    let routes = routes(tx, difficulty_config);

    warp::serve(routes)
        .run((ip, port))
        .await;
}