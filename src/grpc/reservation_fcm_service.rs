use tonic::{Request, Response, Status};
use std::sync::Arc;
use async_trait::async_trait;

// 기존 서비스 포트 import
use crate::{application::port::r#in::reservation_usecase::ReservationUseCase, reservationfcm_proto::{ContentScheduleRequest, UserList, reservation_service_server::ReservationService}};

pub struct ReservationFcmGrpcService {
     reservation_service: Arc<dyn ReservationUseCase + Send + Sync>,  
}
#[async_trait]
impl ReservationService for ReservationFcmGrpcService {
    async fn get_users_by_content_schedule_id(
        &self,
        request: Request<ContentScheduleRequest>,
    ) -> Result<Response<UserList>, Status> {
        // 1) 요청 파싱 (proto가 string이면 trim + parse)
        let req = request.into_inner();
        let schedule_id = req
            .content_schedule_id
            .trim()
            .parse::<u64>()
            .map_err(|_| Status::invalid_argument("Invalid content_schedule_id"))?;

        // 2) 서비스 호출
        let result = self
            .reservation_service
            .find_users_by_schedule_id(schedule_id)
            .await;

        // 3) 결과 매핑 (Vec<i64>/u64 → Vec<String> 방어적으로 변환)
        match result {
            Ok(user_ids) => {
                let user_ids: Vec<String> = user_ids
                    .into_iter()
                    .map(|id| id.to_string())
                    .collect();
                Ok(Response::new(UserList { user_ids }))
            }
            Err(e) => Err(Status::internal(format!("Failed to fetch users: {}", e))),
        }
    }
}

impl ReservationFcmGrpcService {
     pub fn new(reservation_service: Arc<dyn ReservationUseCase + Send + Sync>) -> Self {
        ReservationFcmGrpcService { reservation_service }
    }
}