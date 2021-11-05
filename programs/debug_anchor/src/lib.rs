use anchor_lang::prelude::*;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod debug_anchor {
    use super::*;
    pub fn initialize(_ctx: Context<Initialize>) -> ProgramResult {
        // Log a string
        msg!("hello world");

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[cfg(test)]
mod tests {
    use solana_program_test::*;

    #[tokio::test]
    async fn test_logging() {
        use crate::*;
        use solana_program::instruction::Instruction;
        use solana_program_test::*;
        use solana_sdk::signer::Signer;
        use solana_sdk::transaction::Transaction;
        use anchor_lang::InstructionData;

        // Program
        let program_id = id();
        let program_test = ProgramTest::new("debug_anchor", program_id, processor!(entry));

        // Start
        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;
        let mut transaction = Transaction::new_with_payer(
            &[Instruction::new_with_bytes(
                program_id,
                &instruction::Initialize{}.data(),
                vec![AccountMeta::new(Pubkey::new_unique(), false)],
            )],
            Some(&payer.pubkey()),
        );

        // Sign
        transaction.sign(&[&payer], recent_blockhash);

        // Process
        banks_client.process_transaction(transaction).await.unwrap();
    }
}
