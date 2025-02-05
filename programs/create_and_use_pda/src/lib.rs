// This contract allows users to create their own counter stored in a PDA and also then increment their counter.

// Import anchor 
use anchor_lang::prelude::*;

// This is a unique address that helps clients to identify and interact with the program. 
declare_id!("5Hsvd5MQfukMrBJY28Q2YouAfYRxM5ZyKPj3toQqsqS1"); 

// Define the program
#[program]
pub mod create_and_use_pda {
    use super::*;

    // This function creates a PDA account for the user and counter start at 0.
    pub fn create_counter(ctx: Context<CreateCounter>) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        base_account.count = 0;
        Ok(())
    }

    // This function retrieve the user's PDA and increment the counter.
    pub fn increment_counter(ctx: Context<IncrementCounter>) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        base_account.count += 1;
        Ok(())
    }
}

// Account structure
#[derive(Accounts)]
pub struct CreateCounter<'info> {

    #[account(
        init, 
        payer = user, 
        space = 8 + 8,
        seeds = [b"counter", user.key().as_ref()], // PDA serived from user wallet, "counter" is a static seed and user.key().as_ref() ensures that each user gets their own counter
        bump,
    )]
    pub base_account : Account<'info, BaseAccount>,

    #[account(mut)]
    pub user : Signer<'info>,
    pub system_program : Program<'info, System>
}

#[derive(Accounts)]
pub struct IncrementCounter<'info> {
    #[account(
        mut,
        seeds = [b"counter", user.key().as_ref()], // find the existing PDA using the same seed as used in CreateCounter
        bump,
    )]
    pub base_account : Account<'info, BaseAccount>,

    #[account(mut)]
    pub user : Signer<'info>,
}
// BaseAccount structure
#[account]
pub struct BaseAccount {
    pub count : u64,
}
