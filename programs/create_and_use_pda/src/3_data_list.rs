// This can store the string in the data and data_list field and also update it. 

use anchor_lang::prelude::*;

declare_id!("CdUPNf2bcsix2vNNRpEdDfHpMBVAPmWmXDDXkBhKS3xC");

#[program]
pub mod data_list{
    use super::*;

    // This function initialize the BaseAccount for the user and store a string in both data and data_list field. 
    pub fn initialize(ctx : Context<Initialize>, data : String) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        let copy_of_string = data.clone(); // store a clone of the string
        base_account.data = data; // store a piece of data
        base_account.data_list.push(copy_of_string); // store multiple strings added to the account and push the clone of string into the data_list
        Ok(())
    }

    // This function allows users to update the data field and add the new string to the data_list 
    pub fn update(ctx : Context<Update>, data : String) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account; // retireve the existing BaseAccount
        let copy_of_string = data.clone();
        base_account.data = data;
        base_account.data_list.push(copy_of_string);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 64 + 64)]
    pub base_account : Account<'info, BaseAccount>,

    #[account(mut)]
    pub user : Signer<'info>,
    pub system_program : Program<'info, System>
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub base_account : Account<'info, BaseAccount>,
}

#[account]
pub struct BaseAccount{ // BaseAccount hold two fields data and data_list
    pub data : String,
    pub data_list : Vec<String> 
}