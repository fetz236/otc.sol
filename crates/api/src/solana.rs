use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;

pub async fn get_balance(pubkey_str: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let rpc_url = "https://api.mainnet-beta.solana.com";
    let client = RpcClient::new(rpc_url.to_string());

    let pubkey = Pubkey::from_str(pubkey_str)?;
    let balance = client.get_balance(&pubkey)?;

    Ok(balance)
}
