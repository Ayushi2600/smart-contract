// This manages salary submissions and updates on the blockchain. It ensures that only the owner of a salary record can update their salary.

use anchor_lang::prelude::*;

declare_id!("GLdv7wz3ypkC9f73qKZ3UyksDDjRnTrCDp6KvmCpfcwn");

#[program]
pub mod salary_submit {
    use super::*;

    // Creates a new salary record and assigns it to a user.
    pub fn initialize(ctx: Context<Initialize>, salary: u64) -> Result<()> {
        let account = &mut ctx.accounts.salary_account;
        account.salary = salary; // saves the salary
        account.authority = *ctx.accounts.user.key; // store user's public key
        Ok(())
    }

    // Allows only the original owner to update their salary.
    pub fn update_salary(ctx: Context<UpdateSalary>, new_salary: u64) -> Result<()> {
        let account = &mut ctx.accounts.salary_account;

        // Ensure only the owner can update salary
        require!(account.authority == *ctx.accounts.authority.key, ErrorCode::Unauthorized);
        account.salary = new_salary; // updates the salary 
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8 + 32)]
    pub salary_account: Account<'info, SalaryAccount>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateSalary<'info> {
    #[account(mut, has_one = authority)] 
    pub salary_account: Account<'info, SalaryAccount>,

    pub authority: Signer<'info>,
}

#[account]
pub struct SalaryAccount {
    pub salary: u64,
    pub authority: Pubkey, 
}

// Defines an error when someone other than the owner tries to update the salary.
#[error_code]
pub enum ErrorCode {
    #[msg("You are not authorized to update this salary.")]
    Unauthorized,
}
