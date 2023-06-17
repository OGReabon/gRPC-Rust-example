use tonic::{transport::Server, Request, Response, Status};

use payments::bitcoin_server::{Bitcoin, BitcoinServer};
use payments::{BtcPaymentRequest, BtcPaymentResponse};

pub mod payments {
    tonic::include_proto!("payments");
}

#[derive(Debug, Default)]
pub struct BitcoinService {}

#[tonic::async_trait]
impl Bitcoin for BitcoinService {
    async fn send_payment(
        &self,
        request: Request<BtcPaymentRequest>,
    ) -> Result<Response<BtcPaymentResponse>, Status> {
        println!("Got a request: {:?}", request);

        let req = request.into_inner();

        let reply = BtcPaymentResponse {
            successful: true,
            message: format!("Sent {} BTC to {}", req.amount, req.to_address).into(),
        };
    }
}

#[tokio::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let btc_service = BitcoinService::default();

    Server::builder()
        .add_service(BitcoinServer::new(btc_service))
        .serve(addr)
        .await?;

    Ok(())
}
