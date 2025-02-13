use anchor_lang::prelude::*;

declare_id!("56MX3BTARSEVMAa8T5WPfhEGWKHiuTmaDNfviGoLt6Yy");

#[program]
pub mod user_profile{
    use super::*;

    pub fn create_profile(ctx: Context<CreateProfile>, username: String, bio: String) -> Result<()> {
        let profile = &mut ctx.accounts.profile;
        profile.authority = ctx.accounts.user.key();
        profile.username = username;
        profile.bio = bio;

        msg!("User {} created a profile!", ctx.accounts.user.key());
        Ok(())        
    }

    pub fn update_profile(ctx: Context<UpdateProfile>, new_bio: String) -> Result<()> {
        let profile = &mut ctx.accounts.profile;

        require_keys_eq!(profile.authority, ctx.accounts.user.key(), ProfileError::Unauthorized);
        profile.bio = new_bio;

        msg!("User {} update their bio!", ctx.accounts.user.key());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateProfile<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init, 
        payer = user, 
        space = 8 + 32 + 30 + 200,
        seeds = [b"profile", user.key().as_ref()],
        bump
    )]
    pub profile: Account<'info, Profile>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct UpdateProfile<'info> {
    #[account(mut, seeds = [b"profile", user.key().as_ref()], bump)]
    pub profile: Account<'info, Profile>,

    pub user: Signer<'info>
}

#[account]
pub struct Profile{
    pub authority: Pubkey,
    pub username: String,
    pub bio: String
}

#[error_code]
pub enum ProfileError{
    #[msg("Unauthorized")]
    Unauthorized
}