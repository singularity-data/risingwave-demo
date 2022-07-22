use serde_derive::{Deserialize, Serialize};
use std::path::Path;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::process::Command;
use rand::Rng;
use tonic::codegen::http::response;
use tonic::transport::Channel;
use crate::recommender::recommender_client::RecommenderClient;
use crate::recommender::{ActionType, GetRecommendationRequest, ReportActionRequest};


#[derive(Serialize, Deserialize)]
pub struct User {
    pub(crate) userid: String,
    pub(crate) activeness: f64,
    #[serde(skip)]
    pub(crate) context: UserContext,
    ratings: f64,
    brand: f64,
    type_: f64,
    freshness: f64
}

#[derive(Serialize, Deserialize)]
pub struct Item {
    pub(crate) itemid: String,
    #[serde(skip)]
    pub(crate) context: ItemContext,
    popularity: f64,
    brand: f64,
    type_: f64,
    ratings: f64,
    freshness: f64
}

#[derive(Serialize, Deserialize)]
pub struct ActionHistory{
    userid: String,
    itemid: String,
    action: i32,
    timestamp: u64
}

#[derive(Default)]
#[derive(Serialize, Deserialize)]
pub struct UserContext {
    userid: String,

    // sql source: recent purchased items / item -> user reversed
    recent_ratings: f64,
    recent_brand: f64,
    recent_type_: f64,
    recent_freshness: f64,

    global_ratings: f64,
    global_brand: f64,
    global_type_: f64,
    global_freshness: f64,

    conversion_count: i32
}

#[derive(Default)]
#[derive(Serialize, Deserialize)]
pub struct ItemContext {
    itemid: String,

    view_count: i32,
    click_count: i32,
    purchase_count: i32,

    recent_ratings: f64,
    recent_brand: f64,
    recent_type_: f64,
    recent_freshness: f64,

    global_ratings: f64,
    global_brand: f64,
    global_type_: f64,
    global_freshness: f64,
}

pub trait UpdatableContext{
    fn update(&self, record: &ActionHistory);
}


impl User{
    pub(crate) async fn mock_act<'a>(&'a self, client: &mut RecommenderClient<Channel>, items: &'a Vec<Item>) -> Result<ActionHistory, &str> {
        // json.insert("item", generated item)
        // json.insertion
        let selected_item = items.get(rand::thread_rng().gen_range(0, items.len() - 1)).ok_or("no item found")?;
        let selected_action = ActionType::View;
        let response = client.report_action(tonic::Request::new(
            ReportActionRequest {
                userid: self.userid.clone(),
                itemid: selected_item.itemid.clone(),  // select item
                action: selected_action as i32
            })).await.unwrap();
        let timestamp = response.into_inner().timestamp;

        Ok(ActionHistory{
            userid: self.userid.clone(),
            itemid: selected_item.itemid.clone(),
            action: selected_action as i32,
            timestamp: timestamp
        })
    }

    pub async fn mock_get_recommendations<'a>(&'a self, client: &mut RecommenderClient<Channel>) -> Vec<String> {
        let response = client.get_recommendation(GetRecommendationRequest{
            userid: self.userid.clone()
        }).await.unwrap();

        response.into_inner().itemid
    }
}


pub(crate) fn read_users_json(path: &Path) -> Result<Vec<User>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let users: Vec<User> = serde_json::from_reader(reader)?;
    Ok(users)
}


pub(crate) fn read_items_json(path: &Path) -> Result<Vec<Item>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let items: Vec<Item> = serde_json::from_reader(reader)?;
    Ok(items)
}

pub fn generate_user_metadata() -> Result<(Vec<User>, Vec<Item>), ()> {
    Command::new("python3")
        .arg("../generator")
        .arg("--num-users=150")
        .arg("--num-items=20")
        .arg("--dump-users=../generator/users.json")
        .arg("--dump-items=../generator/items.json")
        .output()
        .expect("failed to execute process");
    let users = read_users_json(Path::new("../generator/users.json")).unwrap();
    let items = read_items_json(Path::new("../generator/items.json")).unwrap();
    return Ok((users, items));
}
