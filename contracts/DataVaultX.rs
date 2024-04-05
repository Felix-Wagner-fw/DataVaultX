use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    borsh::{BorshDeserialize, BorshSerialize},
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct DataTransaction {
    seller: Pubkey,
    data: String,
    price: u64,
    is_sold: bool,
}

#[derive(BorshSerialize, BorshDeserialize)]
struct DataTransactions(Vec<DataTransaction>);

impl DataTransactions {
    pub fn new() -> Self {
        DataTransactions(vec![])
    }

    pub fn push(&mut self, data_transaction: DataTransaction) {
        self.0.push(data_transaction);
    }
}

#[entrypoint]
fn create_data_transaction(
    ctx: Context,
    data: String,
    price: u64,
) -> ProgramResult {
    let mut data_transactions = DataTransactions::try_from_slice(&ctx.accounts.data_transactions.data)?;
    let seller = *ctx.accounts.seller.key;

    let data_transaction = DataTransaction {
        seller,
        data,
        price,
        is_sold: false,
    };

    data_transactions.push(data_transaction);
    data_transactions.serialize(&mut &mut ctx.accounts.data_transactions.data.borrow_mut()[..])?;

    msg!("Data transaction created successfully");
    Ok(())
}

#[entrypoint]
fn buy_data_transaction(
    ctx: Context,
    transaction_id: u64,
) -> ProgramResult {
    let mut data_transactions = DataTransactions::try_from_slice(&ctx.accounts.data_transactions.data)?;
    let buyer = *ctx.accounts.buyer.key;

    if let Some(transaction) = data_transactions.0.get_mut(transaction_id as usize) {
        if transaction.is_sold {
            return Err(ProgramError::InvalidInstructionData);
        }

        if ctx.accounts.buyer_lamports.available < transaction.price {
            return Err(ProgramError::InsufficientFunds);
        }

        ctx.accounts.buyer_lamports.transfer(transaction.price)?;
        transaction.is_sold = true;

        data_transactions.serialize(&mut &mut ctx.accounts.data_transactions.data.borrow_mut()[..])?;
        
        msg!("Data transaction purchased successfully");
        Ok(())
    } else {
        Err(ProgramError::InvalidArgument)
    }
}

#[derive(Accounts)]
pub struct Context<'info> {
    #[account(mut)]
    data_transactions: Account<'info, DataTransactions>,
    #[account(signer)]
    seller: AccountInfo<'info>,
    #[account(signer)]
    buyer: AccountInfo<'info>,
    #[account(mut, has_one = buyer)]
    buyer_lamports: Account<'info, sysvar::Sysvar>,
}
