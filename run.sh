RUST_LOG=info cargo run --bin jito-shredstream-proxy -- shredstream \
    --block-engine-url https://mainnet.block-engine.jito.wtf \
    --auth-keypair shred_key.json \
    --desired-regions amsterdam,frankfurt \
    --dest-ip-ports 127.0.0.1:8001 \
    --grpc-service-port 9999 \
    --num-threads 1
