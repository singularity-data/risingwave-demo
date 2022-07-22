use rand;
use std::collections::HashMap;
use std::ops::DerefMut;
use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;
use rand::Rng;
use tokio::time::Instant;
use tokio::{
    sync::{mpsc, oneshot},
    task,
};
use futures::future::join_all;
use tokio::runtime::{Handle, Runtime};
use tokio::sync::{Mutex};
use tokio::task::JoinHandle;
use crate::entities::User;
use crate::{entities};
use crate::recommender::GetRecommendationRequest;
use crate::recommender::recommender_client::RecommenderClient;

fn get_delay_mills(delay_val: f64) -> u64 {
    let turbulence = rand::thread_rng().gen_range((delay_val * 0.6) as f64, (delay_val * 1.1) as f64) as f64;
    (turbulence * 1000.0) as u64
}

pub async fn main_loop() {
    let (users, items) = entities::generate_user_metadata().unwrap();
    let items = Arc::new(items);

    let mut client = Arc::new(Mutex::new(
        RecommenderClient::connect("https://127.0.0.1:2666")
            .await
            .expect("failed to connect to recommender server")));
    println!("Connected to server");

    let mut threads = vec![];
    for user in users {
        let mut client_mutex = client.clone();
        let items = items.clone();
        let handle = tokio::spawn(async move {
            loop {
                let history = user.mock_act(client_mutex.lock().await.deref_mut(), &items)
                    .await
                    .unwrap();
                println!("fire action success: {}", serde_json::to_string(&history).unwrap());

                let recommendations = user.mock_get_recommendations(client_mutex.lock().await.deref_mut())
                    .await;
                sleep(Duration::from_millis(get_delay_mills(1.0 / user.activeness)));
            }
        });
        threads.push(handle);
    }
    join_all(threads).await;
}
