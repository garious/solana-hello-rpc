use solana_client::rpc_client::RpcClient;
use solana_core::validator::TestValidator;
use solana_sdk::{
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
};
use std::fs::remove_dir_all;

fn main() {
    // Why doesn't this function accept the total supply as its only argument?
    let validator = TestValidator::run();

    // Why are we creating a TCP connection to have the server send a UDP packet?
    let rpc_client = RpcClient::new_socket(validator.leader_data.rpc);

    let bob = Keypair::new();
    let instruction = system_instruction::transfer(&validator.alice.pubkey(), &bob.pubkey(), 1);

    // Why must we get a blockhash and sign the transaction when the RPC client implements the
    // same functionality to update the transaction when the blockhash expires?
    let (blockhash, _fee_calculator) = rpc_client.get_recent_blockhash().unwrap();

    // Why alice and not mint_keypair?
    let signers = [&validator.alice];

    println!("Sending 1 lamport...");

    // Why am I creating a Transaction and not handing the client an Instruction or Message,
    // as we would with the Client trait?
    let mut transaction = Transaction::new_signed_instructions(&signers, &[instruction], blockhash);

    // Why isn't this an async function? If I want to send multiple transactions, I need to either
    // wait for finality or use a different method that may behave differently (no spinner)?
    rpc_client
        .send_and_confirm_transaction_with_spinner(&mut transaction, &signers)
        .unwrap();
    assert_eq!(rpc_client.get_balance(&bob.pubkey()).unwrap(), 1);

    println!("Success!");

    // Why doesn't TestValidator implement Drop?
    validator.server.close().unwrap();
    remove_dir_all(validator.ledger_path).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_runtime::{bank::Bank, bank_client::BankClient};
    use solana_sdk::{client::SyncClient, genesis_config::create_genesis_config};

    #[test]
    fn test_transfer() {
        let (genesis_config, mint) = create_genesis_config(1);
        let bank = Bank::new(&genesis_config);
        let bank_client = BankClient::new(bank);
        let bob = Keypair::new();
        let instruction = system_instruction::transfer(&mint.pubkey(), &bob.pubkey(), 1);
        bank_client.send_instruction(&mint, instruction).unwrap();
        assert_eq!(bank_client.get_balance(&bob.pubkey()).unwrap(), 1);
    }
}
