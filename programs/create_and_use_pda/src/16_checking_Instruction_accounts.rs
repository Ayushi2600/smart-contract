
use anchor_lang::prelude::*;

declare_id!("8VQnc88kMGny1BRzLmTtQXcyrGBWR1UZTytPMfbkWCYW");

#[program]
pub mod check_instruction_accounts {
    use super::*;

    pub fn initialize_account(ctx: Context<InitializeAccount>) -> Result<()> {
        let account = &mut ctx.accounts.my_account;

        if account.is_initialized {
            return Err(ErrorCode::AccountAlreadyInitialized.into());
        }

        account.is_initialized =  true;
        msg!("Account initialized successfully!");
        
        Ok(())
    }

    pub fn check_account_state(ctx: Context<CheckAccountState>) -> Result<()> {
        let account = &mut ctx.accounts.my_account;

        // Check if the account is initialized
        if !account.is_initialized {
            return Err(ErrorCode::AccountNotInitialized.into());
        }

        msg!("Account is initialized!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeAccount<'info> {
    #[account(init, payer = user, space = 8 + 1)]  // 8 bytes for discriminator, 1 byte for bool
    pub my_account: Account<'info, MyAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CheckAccountState<'info> {
    pub my_account: Account<'info, MyAccount>,
}

#[account]
pub struct MyAccount {
    pub is_initialized: bool,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Account is already initialized.")]
    AccountAlreadyInitialized,

    #[msg("Account is not initialized.")]
    AccountNotInitialized,
}
