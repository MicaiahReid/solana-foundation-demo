use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("2BNRo4FVTngRnf7pzXGwK74Seucj5WjZ8RVkDUoGFi1H");
type Thing = bool;
type MyConst<const N: usize> = [u64; N];
const MY_CONST: u64 = 1;
#[program]
pub mod hello_world {

    use super::*;
    pub fn hello(
        ctx: Context<Initialize>,
        // bytes_val: Vec<u8>,
        string_val: String,
        // pubkey: Pubkey,
        // opt: Option<u64>,
        // vec: Vec<u64>,
        // array: [u64; 4],
        // t: Thing,
        // my_struct: Def<String, u64, 9>,
        // thing: MyConst<1>,
        // my_tuple: DefTup,
    ) -> Result<()> {
        // ctx.accounts.account_name.data = u64_val;
        msg!(
            "Hello, {}! (Sent from {})",
            string_val,
            ctx.program_id.to_string()
        );
        Ok(())
    }
}

const NINE: u64 = 9;
#[derive(AnchorSerialize, AnchorDeserialize, Eq, PartialEq, Clone, Debug)]
pub struct Def<A, T, const N: usize> {
    a_field: T,
    b_field: A,
    c_field: [u64; N],
}

#[derive(AnchorSerialize, AnchorDeserialize, Eq, PartialEq, Clone, Debug)]
pub struct DefTup(pub String);

#[derive(Accounts)]
pub struct Initialize {}

// #[derive(Accounts)]
// pub struct InstructionAccounts<'info> {
//     #[account(init, payer = test, space = 8 + 8)]
//     pub account_name: Account<'info, AccountStruct>,
//     #[account(mut)]
//     pub test: Signer<'info>,
//     pub system_program: Program<'info, System>,
// }
// #[account]
// pub struct AccountStruct {
//     data: u64,
// }
