use std::rc::Rc;
use anchor_client::{Client, Cluster};
use anchor_client::solana_sdk::signer::Signer;
use solana_program_test::{processor, ProgramTest};
use test_dapps::id;
use test_dapps::{
    accounts::Initialize,
};


#[tokio::test]
async fn test() {
    let program_test = ProgramTest::new(
        "test_dapps",
        id(),
        processor!(test_dapps::entry),
    );
    println!("Worked!!!");

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;
    let s: Rc<dyn Signer> = Rc::new(payer);

    let my_client = Client::new(Cluster::Localnet, s);

    // let instructions = my_client
    //     .program(id())
    //     .request()
    //     .accounts(test_dapps::accounts::Initialize{});

    // println!("{:?}", test_dapps::accounts::Initialize{})

}