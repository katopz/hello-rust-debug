use anchor_lang::prelude::*;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod hello_rust_debug {
    use super::*;
    pub fn initialize(_ctx: Context<Initialize>) -> ProgramResult {
        // Log a string
        msg!("hello world");

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
