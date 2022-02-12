use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction::transfer,
};

pub fn transfer_solana(
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
