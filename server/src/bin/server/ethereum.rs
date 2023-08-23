use std::error::Error;
use web3::contract::Contract;
use web3::contract::Options;
use web3::ethabi::Address;
use web3::transports::Http;
use web3::types::H256;
use web3::types::U256;
use web3::Web3;

pub struct EthereumManager {
    contract: Contract<Http>,
}

impl EthereumManager {
    pub fn create(
        node_url: &str,
        abi_json: &str,
        contract_address: &str,
    ) -> Result<Self, Box<dyn Error>> {
        let eth = Web3::new(Http::new(node_url)?).eth();
        let address = contract_address.parse::<Address>()?;

        let contract = Contract::from_json(eth, address, &abi_json.as_bytes())?;

        Ok(Self { contract: contract })
    }

    pub fn clone(&self) -> Self {
        Self {
            contract: self.contract.clone(),
        }
    }

    #[allow(dead_code)]
    pub async fn get_messages(&self, from_address: Address) -> Result<(), Box<dyn Error>> {
        let _: Vec<u8> = self
            .contract
            .query("getPosts", (from_address,), None, Options::default(), None)
            .await?;

        Ok(())
    }

    #[allow(dead_code)]
    pub async fn post_message(
        &self,
        from_address: Address,
        contents: &str,
    ) -> Result<(), Box<dyn Error>> {
        let params = (H256::from_slice(contents.as_bytes()),);

        let mut opt = Options::default();
        opt.gas = Some(U256::from(300000));

        self.contract
            .call("createPost", params, from_address, opt)
            .await?;

        Ok(())
    }
}
