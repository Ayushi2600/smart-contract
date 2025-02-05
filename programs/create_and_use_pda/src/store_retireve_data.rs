// This allows storing and retrieving a user's data (name and age) on the blockchain

use anchor_lang::prelude::*;

declare_id!("DKCXnherT6uNHXEhpFN1KTUAvoFwdNGHEaX3USyD1THn");

#[program]
pub mod account_data {
    use super::*;
    // store user's name and age
    pub fn store_data(ctx: Context<StoreData>, name : String,age: u8) -> Result<()> {
        let user_account = &mut ctx.accounts.user_account;
        user_account.name = name;
        user_account.age = age;
        msg!("Strore user's data : Name : {}, Age : {}", user_account.name, user_account.age);
        Ok(())
    }

    // This function fetched the stored data
    pub fn retrieve_data(ctx : Context<RetrieveData>) -> Result<()> {
        let user_account = &ctx.accounts.user_account;
        msg!("Retrieved data: Name: {}, Age: {}", user_account.name, user_account.age);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct StoreData<'info> {
    #[account(init, payer = signer, space = 8 + 4 + 32 + 1)]
    pub user_account: Account<'info, UserData>,
    #[account(mut)]
    pub signer: Signer<'info>, // a person who signing the transaction
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RetrieveData<'info> {
    // It takes the user_account as input and read data from it
    pub user_account: Account<'info, UserData>,
}

#[account]
pub struct UserData {
    name : String,
    age : u8,
}