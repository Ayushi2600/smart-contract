use anchor_lang::prelude::*;

declare_id!("5Hsvd5MQfukMrBJY28Q2YouAfYRxM5ZyKPj3toQqsqS1");

#[program]
pub mod create_and_use_pda {
    use super::*;

    pub fn create_counter(ctx: Context<CreateCounter>) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        base_account.count = 0;
        Ok(())
    }

    pub fn increment_counter(ctx: Context<IncrementCounter>) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        base_account.count += 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateCounter<'info> {
    // "counter" is a static seed and user.key().as_ref() ensures that each user gets their own counter
    #[account(
        init, 
        payer = user, 
        space = 8 + 8,
        seeds = [b"counter", user.key().as_ref()], // PDA serived from user wallet
        bump,
    )]
    pub base_account : Account<'info, BaseAccount>,

    #[account(mut)]
    pub user : Signer<'info>,
    pub system_program : Program<'info, System>
}

#[derive(Accounts)]
pub struct IncrementCounter<'info> {
    #[account(
        mut,
        seeds = [b"counter", user.key().as_ref()], // same seed as used in CreateCounter
        bump,
    )]
    pub base_account : Account<'info, BaseAccount>,

    #[account(mut)]
    pub user : Signer<'info>,
}

#[account]
pub struct BaseAccount {
    pub count : u64,
}
