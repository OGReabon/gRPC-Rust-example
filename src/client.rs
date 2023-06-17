use payments::bitcoin_client::BitcoinClient;
use payments::BtcPaymentRequest;

pub mod payments {
    tonic::include_proto!("payments");
}

#[tokio::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = BitcoinClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(BtcPaymentRequest {
        from_addr: "12345".to_owned(),
        to_address: "67890".to_owned(),
        amount: 100,
    });

    let response = client.send_payment(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
