use litesvm::LiteSVM;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    message::Message,
    signature::{Keypair, Signer},
    transaction::Transaction,
    system_program
};

use crate::CounterInstruction;

#[test]
fn test_program() {
    // Create a new LiteSVM instance
    let mut svm = LiteSVM::new();
    //
    // // Create a keypair for the transaction payer
    let payer = Keypair::new();
    //
    // // Airdrop some lamports to the payer
    svm.airdrop(&payer.pubkey(), 1_000_000_000).unwrap();
    //
    // // Load our program
    let program_keypair = Keypair::new();
    let program_id = program_keypair.pubkey();
    svm.add_program_from_file(program_id, "target/deploy/solana_counter_program.so")
        .expect("Failed to load program");



    let counter_keypair = Keypair::new();
    let initial_value: u64 = 42;

    println!("Testing counter initialization...");
    let init_instruction_data =
    borsh::to_vec(&CounterInstruction::InitializeCounter { initial_value })
        .expect("Failed to serialize instruction");

    let initialize_instruction = Instruction::new_with_bytes(
        program_id,
        &init_instruction_data,
        vec![
            AccountMeta::new(counter_keypair.pubkey(), true),
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
    );

    let message = Message::new(&[initialize_instruction], Some(&payer.pubkey()));
    let transaction = Transaction::new(
        &[&payer, &counter_keypair],
        message,
        svm.latest_blockhash()
    );

    let result = svm.send_transaction(transaction);
    assert!(result.is_ok(), "Initialize transaction should succeed");

    let logs = result.unwrap().logs;
    println!("Transaction logs:\n{:#?}", logs);

    println!("Testing counter increment...");

    let increment_instruction_data =
        borsh::to_vec(&CounterInstruction::IncrementCounter)
            .expect("Failed to serialize instruction");

    let increment_instruction = Instruction::new_with_bytes(
        program_id,
        &increment_instruction_data,
        vec![AccountMeta::new(counter_keypair.pubkey(), true)],
    );

    let message = Message::new(&[increment_instruction], Some(&payer.pubkey()));
    let transaction = Transaction::new(
        &[&payer, &counter_keypair],
        message,
        svm.latest_blockhash()
    );

    let result = svm.send_transaction(transaction);
    assert!(result.is_ok(), "Increment transaction should succeed");

    let logs = result.unwrap().logs;
    println!("Transaction logs:\n{:#?}", logs);
}