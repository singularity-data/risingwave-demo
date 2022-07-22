#![feature(rt)]

use tokio::runtime::{Handle, Runtime};

mod simulation;
mod entities;
mod recommender;

#[tokio::main]
async fn main(){
    println!("This is the recwave actor!");
    let handle = Handle::current();
    simulation::main_loop().await;
}