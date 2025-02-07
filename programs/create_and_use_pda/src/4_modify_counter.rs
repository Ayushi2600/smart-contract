// This can perform some basic arithmetic operations on a stored counter value.

use anchor_lang::prelude::*;
use serde::{Serialize, Deserialize}; 

declare_id!("DvsymM1HhU5ReVAefGz7sXW99kPvJkaYvYKzVKGovKge");

#[program]
pub mod modify_counter {
    use super::*;

    pub fn create(ctx: Context<Create>) -> Result<()> {
        // Creates a new counter account
        let base_account = &mut ctx.accounts.base_account;
        base_account.counter = 0; // sets the counter to 0
        base_account.creater = *ctx.accounts.user.key; // stores the creator's wallet address
        Ok(())
    }

    pub fn add(ctx: Context<Increment>, num: u64) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        base_account.counter += num; // Adds a number to the counter.
        Ok(())
    }

    pub fn subtract(ctx: Context<Decrement>, num: u64) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;

        if base_account.counter >= num {
            base_account.counter -= num; //Subtracts a number from the counter.
        } else {
            base_account.counter = 0; // Ensures the counter doesn’t go negative (sets it to 0).
        }
        Ok(())
    }

    pub fn multiply(ctx: Context<Multiple>, num: u64) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        base_account.counter *= num; // Multiplies the counter by number
        Ok(())
    }

    pub fn divide(ctx: Context<Divide>, num: u64) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;

        if num == 0 {
            base_account.counter = 0; // Prevents division by zero (sets counter to 0).
        } else {
            base_account.counter /= num; // Divides the counter by number
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer = user, space = 8 + 32 + 8)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>, 
}

#[derive(Accounts)]
pub struct Decrement<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>, 
}

#[derive(Accounts)]
pub struct Multiple<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>, 
}

#[derive(Accounts)]
pub struct Divide<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>, 
}

//Defines the structure of the counter account. It stores the current counter value (u64) and the wallet address of the creator (Pubkey)
// Uses serialization so data can be stored and retrieved.
#[account]
#[derive(Serialize, Deserialize)]
pub struct BaseAccount {
    pub counter: u64, 
    pub creater: Pubkey, 
}


