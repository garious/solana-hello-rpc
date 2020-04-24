use solana_client::rpc_client::RpcClient;
use solana_core::validator::TestValidator;
use solana_sdk::{
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
};
use std::fs::remove_dir_all;

fn main() {
    let validator = TestValidator::run();
    let rpc_client = RpcClient::new_socket(validator.leader_data.rpc);

    let bob = Keypair::new();
    let instruction = system_instruction::transfer(&validator.alice.pubkey(), &bob.pubkey(), 1);
    let (blockhash, _fee_calculator) = rpc_client.get_recent_blockhash().unwrap();
    let signers = vec![&validator.alice];
    let mut transaction =
        Transaction::new_signed_instructions(&signers, vec![instruction], blockhash);
    rpc_client.send_transaction(&mut transaction).unwrap();
    assert_ne!(rpc_client.get_balance(&bob.pubkey()).unwrap(), 0);

    validator.server.close().unwrap();
    remove_dir_all(validator.ledger_path).unwrap();
}
