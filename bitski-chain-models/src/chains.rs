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

pub fn chain_from_str(value: &str) -> Result<Chain, anyhow::Error> {
    let chains = chains()?;

    let chain = chains
        .into_iter()
        .find(|chain| {
            vec![
                chain.name.to_lowercase(),
                chain.chain.to_lowercase(),
                chain.short_name.to_lowercase(),
                chain.chain_id.to_string(),
            ]
            .contains(&value.to_lowercase())
        })
        .ok_or_else(|| anyhow::anyhow!("Chain not found"))?;
    Ok(chain)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_chain_name() {
        use super::chain_from_str;
        chain_from_str("eth").expect("could not get eth chain");
        chain_from_str("matic").expect("could not get matic chain");
        assert!(chain_from_str("goerli").is_err());
        chain_from_str("gor").expect("could not get goerli chain");
    }

    #[test]
    fn test_chain_number() {
        use super::chain_from_str;
        chain_from_str("1").expect("could not get chain 1");
        chain_from_str("137").expect("could not get chain 137");
    }
}
