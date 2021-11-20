
use sum_service::sum_service_client::SumServiceClient;
use sum_service::SumRequest;

pub mod sum_service {
    tonic::include_proto!("me.viluon.sum");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = SumServiceClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(SumRequest {
        numbers: vec![1, 2, 3],
    });

    let response = client.sum(request).await?;
    println!("RESPONSE={:?}", response);

    Ok(())
}
