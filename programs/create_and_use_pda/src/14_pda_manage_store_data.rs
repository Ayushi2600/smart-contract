// This demonstrates how to initialize and manage a PDA and store data in a Solana account

use anchor_lang::prelude::*;

declare_id!("EjRJHS6UD3kzcDYDjQaQhjEfrH6fWsQEp5G1F7S9SZ2v");

#[program]
pub mod pda_account {
    use super::*;

    // Initialize the function
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> { // The Context is a collection of accounts that are passed to the function when the program is invoked
        // The pda_account stores the user's public key and a bump value (used to generate the Program Derived Address)
        let account_data = &mut ctx.accounts.pda_account;
        account_data.user = *ctx.accounts.user.key;
        account_data.bump = ctx.bumps.pda_account;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        seeds = [b"data", user.key().as_ref()], 
        bump,                                  
        payer = user,
        space = 8 + DataAccount::INIT_SPACE
    )]
    pub pda_account: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct DataAccount {
    pub user: Pubkey,
    pub bump: u8,     
}
