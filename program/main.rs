let rpc_url = String::from("https://api.devnet.solana.com");
let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());
