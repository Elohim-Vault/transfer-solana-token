use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction::transfer,
};

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey, // Public key of the account the hello world program was loaded into
    accounts: &[AccountInfo], // The account to say hello to
    instruction_data: &[u8], // Ignored, all helloworld instructions are hellos
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
            transfer_solana(program_id, accounts, instruction_data)
        },
        _ => {
            msg!("Instruction: Invalid instruction");
            Err(ProgramError::InvalidInstructionData)
        },
    }
}

fn transfer_solana(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let get_amount = instruction_data
        .last()
        .map(|value| value.to_string())
        .unwrap()
        .parse::<u64>();

    let amount = match get_amount {
        Ok(amount) => amount,
        Err(_) => return Err(ProgramError::InvalidInstructionData),
    };

    let from = next_account_info(account_info_iter)?;
    let to = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;
    let transfer_ix = transfer(from.key, to.key, amount);
    return invoke(
        &transfer_ix,
        &[from.clone(), to.clone(), system_program.clone()],
    );
}
