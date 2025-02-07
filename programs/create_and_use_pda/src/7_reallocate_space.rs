// This allows users to store and update a message on the blockchain. The key feature is account reallocation, which means the account size can change dynamically when updating the message.

use anchor_lang::prelude::*;

declare_id!("Aa1hG83TuKN9Z1T1JmbAankq2JUN7t3Nh9688biHBZ3G");

#[program]
pub mod anchor_realloc { //This is the main program module containing the initialize and update functions
    use super::*;

    // Creates a new account to store a message
    pub fn initialize(ctx: Context<Initialize>, input: String) -> Result<()> {
        ctx.accounts.message_account.message = input; // take input and saved it inside message_account
        Ok(())
    }

    // Updates the stored message
    pub fn update(ctx: Context<Update>, input: String) -> Result<()> {
        ctx.accounts.message_account.message = input; // It modifies message_account to store a new message. It dynamically resizes the account using realloc
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(input: String)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init, 
        payer = payer,  // The user who pays for creating the account
        space = Message::required_space(input.len()))] // Dynamically calculates the required storage size
    pub message_account: Account<'info, Message>, //The account that stores the message
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(input: String)]
pub struct Update<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        mut, 
        realloc = Message::required_space(input.len()), // Resize the account
        realloc::payer = payer, // User pays for additional storage
        realloc::zero = true, //  Ensures new space is initialized to zero
    )]
    pub message_account: Account<'info, Message>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Message {
    pub message: String,
}

impl Message{
    pub fn required_space(input_len: usize) -> usize { //Calculates the total space required for storing the message
        8 + // 8 bytes for the account discriminator
        4 + // 4 bytes to store the length of the string
        input_len
    }
}