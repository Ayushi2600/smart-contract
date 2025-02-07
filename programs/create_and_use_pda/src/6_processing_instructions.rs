use anchor_lang::prelude::*;

declare_id!("CT6Rmfr51ovLBZ9L4Ax5BzjHY4aEJF4b4V16LHAwY1As");

#[program]
pub mod simple_string_contract {
    use super::*;

    // The entry point of the program.
    pub fn process_instruction(ctx: Context<ProcessInstruction>, input_str: String) -> Result<()> {
        // Condition to check if the input string matches "hello"
        if input_str == "hello" {
            msg!("The string is 'hello'.");
            // Logic for the 'hello' string match (you can add custom logic here)
        } else {
            msg!("The string is not 'hello'.");
            // Logic for other strings
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct ProcessInstruction<'info> {
    #[account(init, payer = user, space = 8 + 32)]
    pub account: Account<'info, MyAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct MyAccount {
    pub input_str: String,
}
