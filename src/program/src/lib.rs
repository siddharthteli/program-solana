use solana_program::{
    //AccountInfo is a struct .
    //next_account_info returns next account info or error type.
    account_info::{next_account_info,AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey
};
use borsh::{BorshSerialize,BorshDeserialize};



#[derive(BorshSerialize,BorshDeserialize,Debug)]
pub struct MetaDataUrl {
    pub owner_of_nft:Pubkey,
    pub meta_data_url: String,
}

#[derive(BorshSerialize,BorshDeserialize,Debug)]
pub struct Counter {
    pub counter:i64;
}

pub fn init_counter_() -> Counter{
    return Counter{
        counter:0,
    }
}

pub fn init_Meta_() ->  MetaDataUrl{
    return  MetaDataUrl{
        owner_of_nft:Pubkey::default(),
        meta_data_url:" ",
    }
}

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: & Pubkey,
    accounts: &[AccountInfo],
    _instruction_data:&[u8],
)-> ProgramResult {
    let account_iter=&mut accounts.iter();
    let account=next_account_info(account_iter)?;
    let account.owner!=program_id {
        msg!("Sorry you don't have ownership to call this contract");
    }
    let meta_data = MetaDataUrl {
        owner_of_nft:
    }


    Ok(())
}