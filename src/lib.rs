use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
};



#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub enum CounterInstruction{
    InitializeCounter { initial_value: u64},
    IncrementCounter
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct CounterAccount {
    pub count: u64,
}

pub fn process_instruction (
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Hello Solana!!!");
    msg!("Program ID {:?}",program_id);
    msg!("instruction_data {:?}",instruction_data);

    let instruction = CounterInstruction::try_from_slice(instruction_data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    msg!("Instruction {:?}",instruction);

    // Route to the appropriate handler based on the instruction
    match instruction {
        CounterInstruction::InitializeCounter { initial_value } => {
            process_initialize_counter(program_id, accounts, initial_value)?
        }
        CounterInstruction::IncrementCounter => {
            process_increment_counter(program_id, accounts)?
        }
    };


    Ok(())
}

// Handler function for initializing a counter
fn process_initialize_counter(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    initial_value: u64,
) -> ProgramResult {
    // Create an iterator over the accounts
    let accounts_iter = &mut accounts.iter();

    // Extract the required accounts in order
    let counter_account = next_account_info(accounts_iter)?;  // The new counter account
    let payer_account = next_account_info(accounts_iter)?;    // Who pays for the account
    let system_program = next_account_info(accounts_iter)?;   // System Program for CPI

    // Calculate the space needed for our counter data
    let account_space = 8; // 8 bytes for a u64

    // Get the minimum balance required for rent exemption
    let rent = Rent::get()?;
    let required_lamports = rent.minimum_balance(account_space);

    // Create the counter account via CPI to the System Program
    invoke(
        &system_instruction::create_account(
            payer_account.key,    // Account paying for creation
            counter_account.key,  // New account being created
            required_lamports,    // Lamports to transfer
            account_space as u64, // Space to allocate in bytes
            program_id,          // Program that will own this account (our program)
        ),
        &[
            payer_account.clone(),
            counter_account.clone(),
            system_program.clone(),
        ],
    )?;

    // Initialize the counter data
    let counter_data = CounterAccount {
        count: initial_value,
    };

    // Serialize and write the data to the account
    let mut account_data = &mut counter_account.data.borrow_mut()[..];
    counter_data.serialize(&mut account_data)?;

    msg!("Counter initialized with value: {}", initial_value);

    Ok(())
}

// Handler function for incrementing a counter
fn process_increment_counter(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    // Get the counter account to increment
    let counter_account = next_account_info(accounts_iter)?;

    // Security check: Verify this program owns the account
    if counter_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Get a mutable reference to the account's data
    let mut data = counter_account.data.borrow_mut();

    // Deserialize the current counter value
    let mut counter_data: CounterAccount = CounterAccount::try_from_slice(&data)?;

    // Increment the counter value
    counter_data.count = counter_data
        .count
        .checked_add(1)
        .ok_or(ProgramError::InvalidAccountData)?;

    // Serialize the updated data back to the account
    counter_data.serialize(&mut &mut data[..])?;

    msg!("Counter incremented to: {}", counter_data.count);

    Ok(())
}

entrypoint!(process_instruction);


#[cfg(test)]
mod tests;