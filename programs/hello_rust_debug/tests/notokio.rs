#[cfg(test)]
mod tests {
    use anchor_lang::prelude::AccountMeta;
    use solana_program::{
        account_info::AccountInfo,
        entrypoint::ProgramResult,
        instruction::Instruction,
        program_error::{self, ProgramError},
        pubkey::Pubkey,
        rent::Rent,
    };
    use solana_sdk::account::{create_is_signer_account_infos, Account as SolanaAccount};

    use spl_token::processor::Processor;

    struct SyscallStubs {}
    impl solana_sdk::program_stubs::SyscallStubs for SyscallStubs {
        fn sol_log(&self, _message: &str) {}

        fn sol_invoke_signed(
            &self,
            _instruction: &Instruction,
            _account_infos: &[AccountInfo],
            _signers_seeds: &[&[&[u8]]],
        ) -> ProgramResult {
            Err(ProgramError::Custom(42)) // Not supported
        }

        fn sol_get_clock_sysvar(&self, _var_addr: *mut u8) -> u64 {
            program_error::UNSUPPORTED_SYSVAR
        }

        fn sol_get_epoch_schedule_sysvar(&self, _var_addr: *mut u8) -> u64 {
            program_error::UNSUPPORTED_SYSVAR
        }

        #[allow(deprecated)]
        fn sol_get_fees_sysvar(&self, _var_addr: *mut u8) -> u64 {
            program_error::UNSUPPORTED_SYSVAR
        }

        fn sol_get_rent_sysvar(&self, var_addr: *mut u8) -> u64 {
            unsafe {
                *(var_addr as *mut _ as *mut Rent) = Rent::default();
            }
            solana_program::entrypoint::SUCCESS
        }
    }

    fn do_process_instruction(
        instruction: Instruction,
        accounts: Vec<&mut SolanaAccount>,
    ) -> ProgramResult {
        {
            use std::sync::Once;
            static ONCE: Once = Once::new();

            ONCE.call_once(|| {
                solana_sdk::program_stubs::set_syscall_stubs(Box::new(SyscallStubs {}));
            });
        }

        let mut meta = instruction
            .accounts
            .iter()
            .zip(accounts)
            .map(|(account_meta, account)| (&account_meta.pubkey, account_meta.is_signer, account))
            .collect::<Vec<_>>();

        let account_infos = create_is_signer_account_infos(&mut meta);
        Processor::process(&instruction.program_id, &account_infos, &instruction.data)
    }

    #[test]
    fn test_hello() {
        let program_id = hello_rust_debug::id();
        let mut owner_account = SolanaAccount::default();
        let accounts = vec![AccountMeta::new(Pubkey::new_unique(), false)];
        let data = vec![];

        // Instruction.
        let ix = Instruction {
            program_id: program_id,
            accounts: accounts,
            data: data,
        };

        // log
        do_process_instruction(ix, vec![&mut owner_account]).unwrap();
    }
}
