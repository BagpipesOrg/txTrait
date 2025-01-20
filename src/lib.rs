pub mod sol_fluff;
pub mod dot_sama;
pub mod consts;
pub mod icp_mod;

pub trait TxBuilder {
    fn supported_ecosystems(&self) -> supported_chains {}
}


impl sol_fluff::sol_builder for TxBuilder {}

impl dot_sama::DotSamaTransfer for TxBuilder {}
