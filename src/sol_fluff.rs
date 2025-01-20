use solana_client::{
    pubsub_client::PubsubClient as solPubsubClient,
    rpc_client::RpcClient as solRpcClient,
    rpc_config::{RpcTransactionLogsConfig as solrpctxconf, RpcTransactionLogsFilter},
};
use solana_sdk::instruction::Instruction;
use anyhow::Result;
use solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey};


#[derive(Debug)]
pub enum SolChains {
    MainNet,
    DevNet,
}

pub struct sol_builder {

}


impl sol_builder {
    // raydium v4 swap on solana
    fn sol_swap_tokens(
        &prodrpc: solRpcClient, //client: &RpcClient,
         &amm_program_key: PubKey,
         &state_coin_pc: Poolstate,
         &wallet_pubkey: Pubkey,
         slippage_bps: u64,
         in_amount_specific: u64,
         out_amount_specific: u64,
    ) ->  Result<Instruction, Err> {



    }
}
