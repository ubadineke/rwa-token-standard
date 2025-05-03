use anchor_lang::system_program;
use litesvm::LiteSVM;
use rwa_token_standard::states::CreateAssetParams;
use solana_keypair::Keypair;
use solana_message::Message;
use solana_pubkey::{Pubkey};
use solana_signer::Signer;
use solana_system_interface::instruction::transfer;
use solana_transaction::Transaction;
use solana_program::instruction::Instruction;

#[test]
fn litesvm_test() {
    let from_keypair = Keypair::new();
    let from = from_keypair.pubkey();
    let to = Pubkey::new_unique();

    let mut svm = LiteSVM::new();
    svm.airdrop(&from, 10_000).unwrap();

    //Asset Parameters
    let asset_params =  CreateAssetParams {
        name: "Ubadineke".to_string(),
        symbol: "Prince".to_string(),
        uri: "ubadineke.netlify.app".to_string(),
        delegate: None
    };

    let init_instr = Instruction{
        program_id: rwa_token_standard::ID,
        accounts: rwa_token_standard::accounts::CreateAsset{
            authority: from,
            mint: //create mint
            asset:
             metadata:
            system_program: system_program::ID,
            token_program
            associated_token_program
            token_metadata_program
        }.to_account_metas(None),
        data: rwa_token_standard::instruction::CreateAsset {params: asset_params}.data()
    };
    

    let instruction = transfer(&from, &to, 64);
    let tx = Transaction::new(
        &[&from_keypair],
        Message::new(&[instruction], Some(&from)),
        svm.latest_blockhash(),
    );
    let tx_res = svm.send_transaction(tx).unwrap();

    let from_account = svm.get_account(&from);
    let to_account = svm.get_account(&to);
    assert_eq!(from_account.unwrap().lamports, 4936);
    assert_eq!(to_account.unwrap().lamports, 64);
}
