use solana_program::{
    //AccountInfo is a struct .
    //next_account_info returns next account info or error type.
    account_info::{next_account_info,AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    program_error::ProgramError,
    msg
};
use borsh::{BorshSerialize,BorshDeserialize};

#[derive(BorshSerialize,BorshDeserialize,Debug)]
pub struct TokenId {
    pub tokenid:u64,
}



entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: & Pubkey,//program id of this deployed contract.
    accounts: &[AccountInfo],//For now  array size is only one.
    _instruction_data:&[u8],
)-> ProgramResult {
    let account_iter=&mut accounts.iter();
    let account=next_account_info(account_iter)?;
    if account.owner!=program_id {
        msg!("Sorry you don't have ownership to call this contract");
        return Err(ProgramError::IncorrectProgramId);
    }
    Ok(())
}