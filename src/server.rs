
use tonic::{transport::Server, Request, Response, Status};

use sum_service::sum_service_server::{SumService, SumServiceServer};
use sum_service::{SumRequest, SumResponse};

pub mod sum_service {
    tonic::include_proto!("me.viluon.sum");
}

#[derive(Debug, Default)]
pub struct SumServiceImpl {}

#[tonic::async_trait]
impl SumService for SumServiceImpl {
    async fn sum(
        &self,
        request: Request<SumRequest>,
    ) -> Result<Response<SumResponse>, Status> {
        println!("SumServiceImpl::sum()");
        println!("SumRequest: {:?}", request);
        let sum_request = request.into_inner();
        let sum_response = SumResponse {
            sum: sum_request.numbers.iter().sum(),
        };
        println!("SumResponse: {:?}", sum_response);
        Ok(Response::new(sum_response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let sum_service = SumServiceImpl::default();

    println!("SumService listening on {}", addr);
    Server::builder()
        .add_service(SumServiceServer::new(sum_service))
        .serve(addr)
        .await?;

    Ok(())
}
