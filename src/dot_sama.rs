use subxt::{
    SubstrateConfig,
    //PolkadotConfig,
    utils::{AccountId32, MultiAddress},
    OnlineClient,
};


pub struct DotSamaTransfer{
    pub config: SubstrateConfig
}


impl DotSamaTransfer {
    pub fn dts_transfer(&self, ws_endpoint: &str, dest: AccountId32) -> Result<()> {
        let connect = OnlineClient::<&self.config>::from_url(ws_endpoint).await?;

    }
}
