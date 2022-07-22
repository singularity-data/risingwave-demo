use std::time::{UNIX_EPOCH, SystemTime, SystemTimeError, Duration};
use tonic::{Request, Response, Status};
use crate::recommender::{GetRecommendationRequest, GetRecommendationResponse, ReportActionRequest, ReportActionResponse};
use crate::recommender::recommender_server::Recommender;

pub struct Recwave {

}

#[tonic::async_trait]
impl Recommender for Recwave {
    async fn get_recommendation(&self, request: Request<GetRecommendationRequest>)
        -> Result<Response<GetRecommendationResponse>, Status> {
        let userid = request.into_inner().userid;
        Self::mock_get_recommendation(userid)
    }

    async fn report_action(&self, request: Request<ReportActionRequest>)
        -> Result<Response<ReportActionResponse>, Status> {
        let message = request.into_inner();

        Self::mock_report_action(&message)

    }
}

impl Recwave {
    fn mock_report_action(message: &ReportActionRequest) -> Result<Response<ReportActionResponse>, Status> {
        let duration = SystemTime::now().duration_since(UNIX_EPOCH);
        println!("received action from user `{}` on item `{}`", &message.userid, &message.itemid);
        match duration {
            Ok(dur) => {
                let timestamp = dur.as_micros();
                Ok(Response::new(ReportActionResponse {
                    timestamp: timestamp as u64
                }))
            }
            Err(e) => {
                Err(Status::unknown("Failed to generate timestamp".to_string()))
            }
        }
    }

    fn mock_get_recommendation(userid: String) -> Result<Response<GetRecommendationResponse>, Status> {
        println!("{} requested recommendations", userid);
        Ok(Response::new(GetRecommendationResponse {
            itemid: vec!["2333".to_string()]
        }))
    }
}