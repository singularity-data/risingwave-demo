use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use crate::recommender::recommender_server::RecommenderServer;
use crate::recwave::Recwave;

mod recommender;
mod recwave;

#[tokio::main]
async fn main() {
    let server = RecommenderServer::new(Recwave{});
    tonic::transport::Server::builder().add_service(server).serve(
        SocketAddr::new(IpAddr::from(Ipv4Addr::new(127, 0, 0, 1)), 2666),
    ).await.unwrap()
}
