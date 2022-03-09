use std::rc::Rc;
use anchor_lang::solana_program::system_program;
use solana_sdk::signature::Keypair;
use solana_sdk::{signature::Signer, transaction::Transaction};
use solana_program_test::{processor, ProgramTest};
use test_dapps::{
    accounts::Create as CreateAccounts,
    instruction::Create as CreateInstruction,
    id
};
use anchor_lang::{
    InstructionData,
    ToAccountMetas
};
use solana_sdk::instruction::Instruction;


#[tokio::test]
async fn test() {
    let program_test = ProgramTest::new(
        "test_dapps",
        id(),
        processor!(test_dapps::entry),
    );
    println!("Worked!!!");

    let base_account = Keypair::new();


    let mut program_context = program_test.start_with_context().await;

    let create_ix = CreateInstruction{};

    let ix = Instruction::new_with_bytes(
        id(),
        create_ix.data().as_ref(),
        CreateAccounts {
            base_account: base_account.pubkey(),
            user: program_context.payer.pubkey(),
            system_program: system_program::id()
        }.to_account_metas(None)
    );

    let mut tx = Transaction::new_with_payer(&[ix], Some(&program_context.payer.pubkey()));
    tx.sign(
        &[&base_account, &program_context.payer],
        program_context.last_blockhash,
    );
    let res = program_context.banks_client.process_transaction(tx).await;
    println!("{:#?}", res);
    println!("Worked!!!");
}