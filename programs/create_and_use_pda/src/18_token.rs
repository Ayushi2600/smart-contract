use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, MintTo, Burn, Transfer};

declare_id!("2n3xXsBxrmMSrpMARm2xTYXH6vAQEbfiCVwDqQfjgeMa");

#[program]
pub mod spl_token_mint {
    use super::*;

    pub fn initialize_mint(ctx: Context<InitializeMint>, decimals: u8) -> Result<()> {
        msg!("Initializing Mint with {} decimals...", decimals);
        Ok(())
    }

    pub fn mint_tokens(ctx: Context<MintTokens>, amount: u64) -> Result<()> {
        let mint_decimals = ctx.accounts.mint.decimals;
        let scaled_amount = amount.checked_mul(10u64.pow(mint_decimals as u32)).unwrap();
        
        msg!("Minting {} tokens ({} raw units)...", amount, scaled_amount);
        
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.mint_authority.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        token::mint_to(cpi_ctx, scaled_amount)?;
        
        msg!("Successfully minted {} tokens!", amount);
        Ok(())
    }

    pub fn burn_tokens(ctx: Context<BurnTokens>, amount: u64) -> Result<()> {
        let mint_decimals = ctx.accounts.mint.decimals;
        let scaled_amount = amount.checked_mul(10u64.pow(mint_decimals as u32)).unwrap();
        
        msg!("Burning {} tokens ({} raw units)...", amount, scaled_amount);
        
        let cpi_accounts = Burn {
            mint: ctx.accounts.mint.to_account_info(),
            from: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.owner.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        token::burn(cpi_ctx, scaled_amount)?;
        
        msg!("Successfully burned {} tokens!", amount);
        Ok(())
    }

    pub fn transfer_tokens(ctx: Context<TransferTokens>, amount: u64) -> Result<()> {
        let mint_decimals = ctx.accounts.mint.decimals;

        msg!("Transferring {} tokens with {} decimals...", amount, mint_decimals);      

        let cpi_accounts = Transfer {
            from: ctx.accounts.sender_token_account.to_account_info(),
            to: ctx.accounts.receiver_token_account.to_account_info(),
            authority: ctx.accounts.owner.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        token::transfer(cpi_ctx, amount * 10u64.pow(mint_decimals as u32))?; // Scale amount

        msg!("Successfully transferred {} tokens!", amount);
        Ok(())
    }

}

#[derive(Accounts)]
pub struct InitializeMint<'info> {
    #[account(init, payer = user, mint::decimals = 9, mint::authority = mint_authority)]
    pub mint: Account<'info, Mint>, //A new mint account to be created 
    #[account(mut)]
    pub user: Signer<'info>, //The payer who covers the rent and transaction fees
    pub mint_authority: Signer<'info>, //The signer that will have control over minting new tokens
    pub system_program: Program<'info, System>, //Required for initializing new accounts on Solana.
    pub token_program: Program<'info, Token>, //Used to interact with the SPL token program
}

#[derive(Accounts)]
pub struct MintTokens<'info> {
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
    pub mint_authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct BurnTokens<'info> {
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
    pub owner: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct TransferTokens<'info> {
    #[account(mut)] 
    pub sender_token_account: Account<'info, TokenAccount>,  // Sender's token account

    #[account(mut)]
    pub receiver_token_account: Account<'info, TokenAccount>,

    #[account()]
    pub mint: Account<'info, Mint>,
    pub owner: Signer<'info>, // The wallet address that signs the transfer

    pub token_program: Program<'info, Token>,
}