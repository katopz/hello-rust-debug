mod tests {
    use anchor_lang::prelude::{AccountMeta, Pubkey};
    use hello_rust_debug::{entry, id};
    use solana_program::instruction::Instruction;
    use solana_program_test::*;
    use solana_sdk::signer::Signer;
    use solana_sdk::transaction::Transaction;

    #[tokio::test]
    async fn test_logging() {
        // Program
        let program_id = id();
        let program_test = ProgramTest::new("hello_rust_debug", program_id, processor!(entry));

        // Start
        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;
        let mut transaction = Transaction::new_with_payer(
            &[Instruction::new_with_bincode(
                program_id,
                &(),
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
