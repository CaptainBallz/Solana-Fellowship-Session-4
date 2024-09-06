use anchor_lang::prelude::*;
use anchor_lang::system_program;

declare_id!("EFyPSTdcjFBiaBAZw7XhSMYHeWW9Xc7RGvntc6Yo1PaH");

#[program]
pub mod solana_withdraw {
    use super::*;
    
    pub fn initialize_account(ctx: Context<InitializeAccount>) -> Result<()> {
        let account = &mut ctx.accounts.account;
        account.balance = 0; // Initialize balance to 0
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        let account = &mut ctx.accounts.account;

        // Transfer SOL from the user to the account
        let transfer_instruction = anchor_lang::system_program::Transfer {
            from: ctx.accounts.user.to_account_info(),
            to: account.to_account_info(),
        };
        anchor_lang::system_program::transfer(
            CpiContext::new(ctx.accounts.system_program.to_account_info(), transfer_instruction),
            amount,
        )?;

        // Update the account's balance
        account.balance += amount;
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        let account = &mut ctx.accounts.account;
        
        // Calculate 10% of the current balance
        let max_withdrawable_amount = account.balance / 10;

        // Ensure the requested amount is less than 10% of the balance
        require!(
            amount < max_withdrawable_amount,
            MyError::WithdrawalExceedsLimit
        );

        // Transfer SOL from the account to the user
        let withdraw_instruction = anchor_lang::system_program::Transfer {
            from: account.to_account_info(),
            to: ctx.accounts.user.to_account_info(),
        };

        anchor_lang::system_program::transfer(
            CpiContext::new(ctx.accounts.system_program.to_account_info(), withdraw_instruction),
            amount,
        )?;

        // Update the account balance
        account.balance -= amount;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeAccount<'info> {
    #[account(init, payer = user, space = 8 + 8)]
    pub account: Account<'info, UserAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub account: Account<'info, UserAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub account: Account<'info, UserAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct UserAccount {
    pub balance: u64, // Stores the balance of SOL in lamports
}

#[error_code]
pub enum MyError {
    #[msg("Withdrawal exceeds 10% of the balance")]
    WithdrawalExceedsLimit,
}
