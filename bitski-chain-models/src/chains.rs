use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Chain {
    pub name: String,
    pub short_name: String,
    pub chain: String,
    pub chain_id: u64,
    #[serde(default)]
    pub rpc: Vec<String>,
}

const CHAINS_JSON: &str = include_str!("../../chains/chains.json");

pub fn chains() -> Result<Vec<Chain>, anyhow::Error> {
    let chains = serde_json::from_str(CHAINS_JSON)?;
    Ok(chains)
}

pub fn chain_from_str(value: &&str) -> Result<Chain, anyhow::Error> {
    let chains = chains()?;

    let chain = chains
        .into_iter()
        .find(|chain| {
            vec![
                chain.name.to_lowercase(),
                chain.chain.to_lowercase(),
                chain.short_name.to_lowercase(),
            ]
            .contains(&value.to_lowercase())
        })
        .ok_or_else(|| anyhow::anyhow!("Chain not found"))?;
    Ok(chain)
}
