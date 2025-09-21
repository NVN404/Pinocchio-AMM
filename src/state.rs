use core::mem::size_of;
use pinocchio::{pubkey::Pubkey, account_info::AccountInfo, program_error::ProgramError};

#[repr(c)]
pub struct Config{
    state : u8,
    seed : [u8 ; 8],
    authority : Pubkey , 
    mint_x : Pubkey ,
    mint_y : Pubkey ,
    fee : [u8 ;2],
    config_bump : [u8 ; 1],
}


#[repr(u8)]
pub enum AmmState{
    Uninitialised = 0u8,
    Initialized = 1u8,
    Disabled = 2u8,
    WithdrawOnly = 3u8,
}

impl Config {
    pub const LEN : usize = size_of::<Config>();

    /////////////////////////////////// READING HELPERS 
 #[inline(always)]
 pub fn load(account_info : &AccountInfo)-> Result<Ref<Self>, ProgramError>{
    if account_info.data_len() != Self::LEN{
        return Err(ProgramErr::InvalidAccountData);
    }
    if account_info.owner().ne(&crate::ID){
        return Err(ProgramError::InvalidAccountOwner);
    }
    Ok(Ref::map(account_info.try_borrow_data()?, |data| unsafe {
        Self::from_bytes_unchecked(data)
    }))
    
 #[inline(always)]

    pub unsafe fn load_unchecked(account_info : &AccountInfo)-> Result<&Self, ProgramError>{
    if account_info.data_len() != Self::LEN{
        return Err(ProgramErr::InvalidAccountData);
    }
    if account_info.owner().ne(&crate::ID){
        return Err(ProgramError::InvalidAccountOwner);
    }
    Ok(Self::from_bytes_unchecked(
        account_info.borrow_data_unchecked(),
    ))

}
