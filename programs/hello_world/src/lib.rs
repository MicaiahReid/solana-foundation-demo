use anchor_lang::prelude::*;
use borsh::BorshDeserialize;

declare_id!("DNPFtc5GRME1BNVV1caSgnxK28GvKduRG31BrpwAgA3u");

#[program]
pub mod hello_world {

    use super::*;
    pub fn hello(ctx: Context<Initialize>, string_val: String) -> Result<()> {
        msg!(
            "Hello there, {}! (Sent from {})",
            string_val,
            ctx.program_id.to_string()
        );
        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Eq, PartialEq, Clone, Debug)]
pub struct DefTup(pub String);

#[derive(Accounts)]
pub struct Initialize {}
