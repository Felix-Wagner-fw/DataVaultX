use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    borsh::{BorshDeserialize, BorshSerialize},
};

const INITIAL_SUPPLY: u64 = 1000000;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct Token {
    supply: u64,
    name: String,
    symbol: String,
}

impl Token {
    pub fn new(name: String, symbol: String) -> Self {
        Token {
            supply: INITIAL_SUPPLY,
            name,
            symbol,
        }
    }
}

#[entrypoint]
fn initialize_token(
    ctx: Context,
    name: String,
    symbol: String,
) -> ProgramResult {
    let mut token = Token::new(name, symbol);
    token.serialize(&mut &mut ctx.accounts.token.data.borrow_mut()[..])?;

    msg!("Token initialized successfully");
    Ok(())
}

#[derive(Accounts)]
pub struct Context<'info> {
    #[account(mut)]
    token: Account<'info, TokenAccount>,
    #[account(signer)]
    authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct TokenAccount<'info> {
    #[account(init, payer = authority, space = 256)]
    pub account: Account<'info, Token>,
    pub system_program: AccountInfo<'info>,
}
