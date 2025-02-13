use anchor_lang::prelude::*;

// Declare the program ID
declare_id!("8E5LnZVwhGJ5aLHtspcXaV7VUE4REEpSGibNdsr17mxW");

#[program]
pub mod create_accounts {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, name: String, balance: u64) -> Result<()> {
        let new_account = &mut ctx.accounts.new_account;
        new_account.name = name;
        new_account.balance = balance;
        new_account.authority = *ctx.accounts.user.key;
        Ok(())
    }

    pub fn fetch_balance(ctx: Context<FetchBalance>) -> Result<u64> {
        let user_account = &ctx.accounts.user_account;
        Ok(user_account.balance)
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 32 + 8 + 40)]
    pub new_account: Account<'info, UserAccount>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct FetchBalance<'info> {
    #[account(has_one = authority)]
    pub user_account: Account<'info, UserAccount>,
    pub authority: Signer<'info>,
}

#[account]
pub struct UserAccount {
    pub name: String, 
    pub balance: u64,
    pub authority: Pubkey,
}
