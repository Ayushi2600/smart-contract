// It manages rent payments for new accounts on the Solana blockchain

use anchor_lang::prelude::*;
use anchor_lang::system_program;

declare_id!("6Yr9HEaDJpC6VGX4JFVHsJGkU1NXgavrusgwdxXzHGEb");

#[program] // main entry point of the program
pub mod pda_rent_payer { // Inside the module, we define all the instructions that can be called by users
    use super::*;

    // This function initializes a rent account and funds it with lamports 
    pub fn init_rent_account(ctx: Context<InitRentAccount>, fund_lamports: u64) -> Result<()> {
        let rent_account = &mut ctx.accounts.rent_account;
        rent_account.balance = fund_lamports;

        // Creates a cpi account struct to facilitate a Solana system program transfer
        let cpi_accounts = system_program::Transfer {
            // The lamports will be transferred from the funder account to the rent_account
            from: ctx.accounts.funder.to_account_info(),
            to: rent_account.to_account_info(),
        };

        // Executes the actual transfer of lamports.
        let cpi_ctx = CpiContext::new(ctx.accounts.system_program.to_account_info(), cpi_accounts);
        system_program::transfer(cpi_ctx, fund_lamports)?;

        Ok(())
    }

    // This function creates a new account using the funds from the rent_account
    pub fn create_new_account(ctx: Context<CreateNewAccount>) -> Result<()> {
        let rent_account = &mut ctx.accounts.rent_account; // retrieve the rent account
        let rent_amount = 1000000; // Define a fixed rent amount

        // Checks if the rent account has enough balance. If not, it returns an error code InsufficientFunds
        if rent_account.balance < rent_amount {
            return Err(ErrorCode::InsufficientFunds.into());
        }

        // Deduct rent from the account
        rent_account.balance -= rent_amount;

        // Initialize the new account
        let new_account = &mut ctx.accounts.new_account;
        new_account.owner = ctx.accounts.funder.key(); //Assigns the funder as the owner of the newly created account.

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitRentAccount<'info> {
    #[account(init, payer = funder, space = 8 + 64)]  // Define space for the rent account
    pub rent_account: Account<'info, RentAccount>,
    #[account(mut)]
    pub funder: Signer<'info>,  // The account funding the rent
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateNewAccount<'info> {
    #[account(mut)]
    pub rent_account: Account<'info, RentAccount>,  // The rent account used for rent payment
    #[account(init, payer = funder, space = 8 + 64)]  // New account to be created
    pub new_account: Account<'info, NewAccount>,
    #[account(mut)]
    pub funder: Signer<'info>,  // The account funding the new account creation
    pub system_program: Program<'info, System>,
}

#[account]
pub struct RentAccount {
    pub balance: u64,  // The amount of lamports in the rent account
}

#[account]
pub struct NewAccount {
    pub owner: Pubkey,  // The owner of the newly created account
}

#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient funds in the rent account.")]
    InsufficientFunds,
}
