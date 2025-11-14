use tonic::transport::Server;
use crate::{grpc::{grpc_service::ReservationGrpcService, reservation_fcm_service::ReservationFcmGrpcService}
    , reservation_proto::reservation_service_server::ReservationServiceServer as ReservationCoreServiceServer
    , reservationfcm_proto::reservation_service_server::ReservationServiceServer as ReservationFcmServiceServer};
use std::sync::Arc;
use crate::state::AppState;
use tokio::task;


pub async fn run_grpc_server(state: Arc<AppState>) -> Result<(), tonic::transport::Error> {
    let addr = format!("{}:{}", state.settings.grpc_host, state.settings.grpc_port)
        .parse()
        .unwrap();

    let service = ReservationGrpcService::new(Arc::clone(&state.reservation_service));
    let fcm_service = ReservationFcmGrpcService::new(Arc::clone(&state.reservation_service));

    println!("gRPC Server running at {}", addr);

    Server::builder()
        .add_service(ReservationCoreServiceServer::new(service))
        .add_service(ReservationFcmServiceServer::new(fcm_service))
        .serve(addr)
        .await // 에러를 반환하도록 수정
}

pub fn spawn_grpc_server(state: Arc<AppState>) -> tokio::task::JoinHandle<Result<(), tonic::transport::Error>> {
    task::spawn(async move { run_grpc_server(state).await })
}