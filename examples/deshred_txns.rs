use jito_protos::shredstream::{
    shredstream_proxy_client::ShredstreamProxyClient, SubscribeVersionedTransactionsRequest,
};
use solana_sdk::transaction::VersionedTransaction;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let mut client = ShredstreamProxyClient::connect("http://127.0.0.1:9999")
        .await
        .unwrap();
    let mut stream = client
        .subscribe_versioned_transactions(SubscribeVersionedTransactionsRequest {})
        .await
        .unwrap()
        .into_inner();

    while let Some(versioned_transaction) = stream.message().await.unwrap() {
        let versioned_transactions =
            match bincode::deserialize::<Vec<VersionedTransaction>>(&versioned_transaction.transactions) {
                Ok(transactions) => transactions,
                Err(e) => {
                    println!("Deserialization failed with err: {e}");
                    continue;
                }
            };

        // Print the transaction signature
        for transaction in versioned_transactions {
            println!("transaction: {:?}", transaction.signatures[0].to_string());
        }
    }
    Ok(())
}
