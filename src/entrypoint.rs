use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg,
    program_error::ProgramError, pubkey::Pubkey,
};

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = instruction_data
        .first()
        .map(|value| value.to_string())
        .unwrap()
        .parse::<u64>()
        .unwrap();

    match instruction {
        0 => {
            msg!("Instruction: transferSolana");
            crate::processor::transfer_solana(program_id, accounts, instruction_data)
        }
        _ => {
            msg!("Instruction: Invalid instruction");
            Err(ProgramError::InvalidInstructionData)
        }
    }
}
