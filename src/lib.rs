use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    pubkey::Pubkey,
    system_instruction::transfer,
    program_error::ProgramError
};

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey, // Public key of the account the hello world program was loaded into
    accounts: &[AccountInfo], // The account to say hello to
    instruction_data: &[u8], // Ignored, all helloworld instructions are hellos
) -> ProgramResult {
    msg!("This is a solana program to transfer SOL");
	let get_tag = instruction_data
		.last()
		.map(|value| value.to_string())
		.unwrap()
		.parse::<u64>();

	let amount = match get_tag {
		Ok(amount) => amount,
		Err(_) => return Err(ProgramError::InvalidInstructionData),
	};

    let account_info_iter = &mut accounts.iter();
    let from = next_account_info(account_info_iter)?;
    let to = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;
    let transfer_ix = transfer(from.key, to.key, amount);
    invoke(
        &transfer_ix,
        &[from.clone(), to.clone(), system_program.clone()],
    )
}

// Sanity tests
#[cfg(test)]
mod test {}
