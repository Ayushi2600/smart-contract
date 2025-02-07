// This contract for a carnival ticketing system. The contract allows users to: - Go on a ride (if they meet height and ticket requirements). - Play a game (if they have enough tickets).- Eat food (if they have enough tickets).

use anchor_lang::prelude::*;

declare_id!("4tkKozd7G69gFG87iTgxnL3wso5t5bxKxwPjLRaK4yDM");

#[program]
pub mod carnival {
    use super::*;

    /// Function to go on a ride
    pub fn go_on_ride(
        ctx: Context<CarnivalContext>,
        name: String,
        height: u32,
        ticket_count: u32,
        ride_name: String,
    ) -> Result<()> {
        require!(height >= 120, CustomError::HeightTooShort);
        require!(ticket_count >= 2, CustomError::NotEnoughTickets);

        msg!("{} went on the ride: {}", name, ride_name);
        Ok(())
    }

    /// Function to play a game
    pub fn play_game(
        ctx: Context<CarnivalContext>,
        name: String,
        ticket_count: u32,
        game_name: String,
    ) -> Result<()> {
        require!(ticket_count >= 1, CustomError::NotEnoughTickets);

        msg!("{} played the game: {}", name, game_name);
        Ok(())
    }

    /// Function to eat food
    pub fn eat_food(
        ctx: Context<CarnivalContext>,
        name: String,
        ticket_count: u32,
        food_stand_name: String,
    ) -> Result<()> {
        require!(ticket_count >= 1, CustomError::NotEnoughTickets);

        msg!("{} ate food at: {}", name, food_stand_name);
        Ok(())
    }
}

/// Defines the accounts needed for each function call
#[derive(Accounts)]
pub struct CarnivalContext<'info> {
    #[account(mut)]
    pub payer: Signer<'info>, // The user who is paying for the transaction
}

/// Custom error handling
#[error_code]
pub enum CustomError {
    #[msg("You do not have enough tickets.")]
    NotEnoughTickets,

    #[msg("You are too short to go on this ride.")]
    HeightTooShort,
}
