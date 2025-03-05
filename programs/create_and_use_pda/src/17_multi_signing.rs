use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

// Declare the program ID
declare_id!("GUz6q3hdeZRsvHXhqjJJoYkrV2MFAfGLAfMru9EWeCnZ");

#[program]
pub mod multi_sig_wallet {
    use super::*;

    // Initialize the multi-sig wallet
    pub fn initialize(ctx: Context<Initialize>, required_signers: u8) -> Result<()> {
        require!(required_signers > 0 && required_signers <= 3, ErrorCode::InvalidRequiredSigners);

        let multisig_wallet = &mut ctx.accounts.multisig_wallet;
        multisig_wallet.required_signers = required_signers;
        multisig_wallet.signers_approved = 0;

        let mut signers = Vec::new();
        if required_signers >= 1 {
            signers.push(*ctx.accounts.signer_1.key);
        }
        if required_signers >= 2 {
            signers.push(*ctx.accounts.signer_2.key);
        }
        if required_signers == 3 {
            signers.push(*ctx.accounts.signer_3.key);
        }

        multisig_wallet.signers = signers;
        Ok(())
    }

    // Approve and execute the transaction
    pub fn approve_transaction(ctx: Context<ApproveTransaction>, amount: u64) -> Result<()> {
        let multisig_wallet = &mut ctx.accounts.multisig_wallet;

        // Ensure the signer is authorized
        require!(multisig_wallet.signers.contains(&ctx.accounts.signer.key()), ErrorCode::SignerNotApproved);

        // Increment approval count
        multisig_wallet.signers_approved += 1;
        msg!("Signer {} approved the transaction.", ctx.accounts.signer.key());

        // Execute transaction if enough approvals are met
        if multisig_wallet.signers_approved >= multisig_wallet.required_signers {
            msg!("Transaction approved by required signers. Transferring SOL...");

            let transfer_instruction = Transfer {
                from: ctx.accounts.multisig_wallet.to_account_info(),
                to: ctx.accounts.recipient.to_account_info(),
            };
            
            let cpi_ctx = CpiContext::new(ctx.accounts.system_program.to_account_info(), transfer_instruction);
            transfer(cpi_ctx, amount)?;

            msg!("Successfully transferred {} lamports to {}", amount, ctx.accounts.recipient.key());
        }

        Ok(())
    }
}

// Accounts involved in initializing the wallet
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + MultisigWallet::LEN)]
    pub multisig_wallet: Account<'info, MultisigWallet>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,

    // Authorized signers
    pub signer_1: Signer<'info>,
    pub signer_2: Signer<'info>,
    pub signer_3: Signer<'info>,
}

// Accounts involved in approving a transaction
#[derive(Accounts)]
pub struct ApproveTransaction<'info> {
    #[account(mut)]
    pub multisig_wallet: Account<'info, MultisigWallet>,
    #[account(mut)]
    pub recipient: SystemAccount<'info>, // The receiver of the SOL transfer
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// Data stored in the multisig wallet
#[account]
pub struct MultisigWallet {
    pub signers: Vec<Pubkey>,       // List of authorized signers
    pub required_signers: u8,       // The number of required approvals
    pub signers_approved: u8,       // The number of approvals received so far
}

impl MultisigWallet {
    const LEN: usize = 4 + // Required signers
                       4 + // Signers approved
                       32 * 3; // Max 3 signers (3 * 32 bytes per Pubkey)
}

// Error codes for validation
#[error_code]
pub enum ErrorCode {
    #[msg("This signer is not authorized to approve the transaction")]
    SignerNotApproved,

    #[msg("The number of required signers must be between 1 and 3")]
    InvalidRequiredSigners,
}
