// It is designed to calculate rent and create a new system account dynamically. It ensures that the account has enough SOL (lamports) to remain rent-exempt.

use anchor_lang::prelude::*;
use anchor_lang::system_program;

declare_id!("CRiVtWUGg6cg6Ru46EwPQTCnh1fZK4PaVXLr2ZyA2k4s");

#[program]
pub mod rent_example { //main module containing the program logic
    use super::*;

    // This function is called when a user wants to create a system account
    pub fn create_system_account(ctx: Context<CreateSystemAccount>, data: AddressData) -> Result<()> {
        msg!("Creating a new system account....");

        let account_size = data.try_to_vec()?.len(); //Converts AddressData into a byte array and gets its length (size in bytes)
        let min_rent = Rent::get()?.minimum_balance(account_size); // Queries the Solana runtime to get the minimum required balance for the account.

        msg!("Required account size: {}", account_size);
        msg!("Minimum rent (lamports): {}", min_rent);

        // Uses the Solana system program to create the account
        system_program::create_account(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                system_program::CreateAccount {
                    from: ctx.accounts.signer.to_account_info(), // Pays for the account creation.
                    to: ctx.accounts.new_account.to_account_info(), // The account that will be created
                },
            ),
            min_rent, //minimum balance required for rent exemption
            account_size as u64, // calculated account size
            &ctx.accounts.system_program.key(),
        )?;

        msg!("Account successfully created.");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateSystemAccount<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub new_account: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug)]
pub struct AddressData {
    name: String,
    address: String,
}