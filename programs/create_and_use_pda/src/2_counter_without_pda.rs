// It allows a user to create a counter account and then user can increment the counter.
use anchor_lang::prelude::*;

declare_id!("DzCZjbDX621AhEdkGHpmEfDaCD7LF7jws6S4tpZcpgGV");

#[program]
pub mod counter_without_pda {
    use super::*;

    // This function creates a counter account and counter should be 0.
    pub fn create(ctx: Context<Create>) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        base_account.count = 0;
        Ok(())
    }
    
    // This function retrieves the counter account and increments the count.
    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        base_account.count += 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer = user, space = 8 + 8)]
    pub base_account : Account<'info, BaseAccount>,

    #[account(mut)]
    pub user : Signer<'info>,
    pub system_program : Program<'info, System>
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    pub base_account : Account<'info, BaseAccount>,

}

#[account]
pub struct BaseAccount {
    pub count : u64,
}