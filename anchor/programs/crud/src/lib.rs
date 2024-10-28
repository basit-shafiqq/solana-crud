#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("AsjZ3kWAUSQRNt2pZVeJkywhZ6gpLpHZmJjduPmKZDZZ");

#[program]
pub mod crud {
    use super::*;

    pub fn create_journal_entry(
        context: Context<CreateEntry>,
        title: String,
        message: String
    ) -> Result<()> {
        let journal = &mut context.accounts.journal_entry;
        journal.author = *context.accounts.owner.key;
        journal.title = title;
        journal.message = message;
        Ok(())
    }

    pub fn update_journal_entry(
        context: Context<UpdateEntry>,
        _title: String,
        message: String
    ) -> Result<()> {
        let journal = &mut context.accounts.journal_entry;
        journal.message = message;
        Ok(())
    }

    pub fn delete_journal(_context:Context<DeleteEntry>,_title:String)->Result<()>{

      Ok(())
    }
}

#[derive(Accounts)]
#[instruction(title:String)]
pub struct CreateEntry<'info> {
    #[account(
        init,
        seeds = [title.as_bytes(), owner.key().as_ref()],
        bump,
        space = 8 + Journal::INIT_SPACE,
        payer = owner
    )]
    pub journal_entry: Account<'info, Journal>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(title:String)]
pub struct UpdateEntry<'info> {
    #[account(
        mut,
        seeds = [title.as_bytes(), owner.key().as_ref()],
        bump,
        realloc = 8 + Journal::INIT_SPACE,
        realloc::payer = owner,
        realloc::zero = true
    )]
    pub journal_entry: Account<'info, Journal>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(title:String)]
pub struct DeleteEntry<'info> {
    #[account(
        mut,
        seeds = [title.as_bytes(), owner.key().as_ref()],
        bump,
        close = owner,
    )]
    pub journal_entry: Account<'info, Journal>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct Journal {
    pub author: Pubkey,
    #[max_len(50)]
    pub title: String,
    #[max_len(500)]
    pub message: String,
}
