use anchor_lang::{system_program, InstructionData, ToAccountMetas};
use anchor_spl::token_2022;
use litesvm::LiteSVM;
use rwa_token_standard::{
    constants::*, states::*
};
use solana_keypair::Keypair;
use solana_message::Message;
use solana_pubkey::Pubkey;
use solana_signer::Signer;
// use solana_system_interface::instruction::transfer;
use solana_transaction::Transaction;
use solana_program::instruction::Instruction;


#[test]
fn litesvm_test() {
    let from_keypair = Keypair::new();
    let from = from_keypair.pubkey();
    // let to = Pubkey::new_unique();

    let mut svm = LiteSVM::new();
    svm.airdrop(&from, 10_000).unwrap();

    let program_id = rwa_token_standard::ID;
    let program_bytes = include_bytes!("../../.././target/deploy/rwa_token_standard.so");

    svm.add_program(program_id, program_bytes);

    //Asset Parameters
    let asset_params =  CreateAssetParams {
        name: "Ubadineke".to_string(),
        symbol: "Prince".to_string(),
        uri: "ubadineke.netlify.app".to_string(),
        delegate: None
    };

    //Create Account for Mint
    //Initialize Account as Mint
    let mint = Keypair::new();

    //Derive Asset PDA
    let asset_pda = Pubkey::find_program_address(
        &[ASSET.as_bytes(), mint.pubkey().as_ref()],
        &rwa_token_standard::ID,
    ).0;

    let init_ix = Instruction{
        program_id: rwa_token_standard::ID,
        accounts: rwa_token_standard::accounts::CreateAsset{
            authority: from,
            mint: mint.pubkey(),
            asset: asset_pda,
            system_program: system_program::ID,
            token_program: token_2022::ID,
        }.to_account_metas(None),
        data: rwa_token_standard::instruction::CreateAsset {params: asset_params}.data()
    };

    let tx = Transaction::new(
        &[&from_keypair],
        Message::new(&[init_ix], Some(&from)),
        svm.latest_blockhash(),
    );
    let tx_res = svm.send_transaction(tx);

    match tx_res {
        Ok(res) => {
            dbg!(res.logs);
        }
        Err(e) => {
            dbg!(e.meta.logs);
        }
    }

    // let to_account = svm.get_account(&to);
    let asset = svm.get_account(&asset_pda).unwrap();
    dbg!(asset);


    // let from_account = svm.get_account(&from);
    // // let to_account = svm.get_account(&to);
    // assert_eq!(from_account.unwrap().lamports, 4936);
    // assert_eq!(to_account.unwrap().lamports, 64);
}
