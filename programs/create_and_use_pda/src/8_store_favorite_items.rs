// This smart contract stores and retrieves a user's favorite number, color, and hobbies on the blockchain. It ensures that each user has a unique on-chain account tied to their public key.

use anchor_lang::prelude::*;

declare_id!("5R5eMMtEDaWesYZuJEJwcTSEvVXDXtsvCdbmSgeRnrm6");

pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8; // Anchor automatically adds an 8-byte discriminator to every account.

#[program] 
pub mod favorites { // It contains the function that interacts with the blockchain.
    use super::*;

    // store user's favorite number, color and hobbies in an account
    pub fn set_favorites(ctx: Context<SetFavorites>, number: u64, color: String, hobbies: Vec<String>) -> Result<()> {
        msg!("Greetings from {}", ctx.program_id); // prints the program id on the blockchain logs
        let user_public_key = ctx.accounts.user.key(); // fetch the wallet address of the user
        msg!(
            "User {user_public_key}'s favorite number is {number}, favorite color is: {color}, and their hobbies are {hobbies:?}",
        ); // Prints the user's submitted data.

        ctx.accounts.favorites.set_inner(Favorites {
            number,
            color,
            hobbies
        }); // Saves the favorite number, color, and hobbies in the user's on-chain account.
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SetFavorites<'info> { // defines the accounts required to store the user's data
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init_if_needed,  // If the account does not exist, it initializes it. If it already exists, it updates it.
        payer = user, // user’s wallet will pay the required Solana fees
        space = ANCHOR_DISCRIMINATOR_SIZE + Favorites::INIT_SPACE,  // Required metadata + Automatically calculates required space
        seeds=[b"favorites", user.key().as_ref()], // Each user gets a unique account based on their wallet address
        bump
    )]
    pub favorites: Account<'info, Favorites>,

    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct Favorites { // Defines the account structure to store user preferences
    pub number: u64, // store favorite number as an unsigned 64-bit integer

    #[max_len(50)] // maximum length is 50 characters
    pub color: String, // store favorite color as a string

    #[max_len(5, 50)] // store up to 5 hobbies, each up to 50 characters long
    pub hobbies: Vec<String> // Stores the hobbies as a list of strings
}