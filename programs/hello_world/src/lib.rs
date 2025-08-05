use std::collections::HashMap;

use anchor_lang::prelude::*;
use borsh::BorshDeserialize;

declare_id!("784RuaFufDQCrsTgMgtVFEciombEWXjyaeaJa64EevR");

#[program]
pub mod hello_world {

    use anchor_lang::prelude::*;
    use anchor_lang::system_program::{transfer, Transfer};

    use super::*;
    pub fn hello(ctx: Context<SolTransfer>, string_val: String, amount: u64) -> Result<()> {
        let from_pubkey = ctx.accounts.sender.to_account_info();
        let to_pubkey = ctx.accounts.recipient.to_account_info();
        let program_id = ctx.accounts.system_program.to_account_info();

        *ctx.accounts.custom = CustomAccount {
            my_custom_data: 0,
            some_string: string_val.clone(),
            inner: InnerStruct {
                some_data: 0,
                some_other_data: string_val.clone(),
                index_map: HashMap::from([(string_val.clone(), 0)]),
            },
            a_pubkey: Pubkey::new_unique(),
        };

        let cpi_context = CpiContext::new(
            program_id,
            Transfer {
                from: from_pubkey,
                to: to_pubkey.clone(),
            },
        );
        transfer(cpi_context, amount)?;
        emit_cpi!(TransferEvent {
            amount,
            to_pubkey: *to_pubkey.key
        });
        emit_cpi!(MessageEvent {
            message: format!(
                "Hello, {} (Sent from {})",
                string_val,
                ctx.program_id.to_string()
            )
        });

        msg!(
            "{}",
            format!(
                "Goodbye, {} (Sent from {})",
                string_val,
                ctx.program_id.to_string()
            )
        );
        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Eq, PartialEq, Clone, Debug)]
pub struct DefTup(pub String);

#[event_cpi]
#[derive(Accounts)]
pub struct SolTransfer<'info> {
    #[account(mut)]
    sender: Signer<'info>,
    #[account(mut)]
    recipient: SystemAccount<'info>,
    system_program: Program<'info, System>,
    #[account(
        init_if_needed,
        payer = sender,
        seeds = [b"custom", sender.key().as_ref()],
        bump,
        space = 8 + 100,
    )]
    pub custom: Account<'info, CustomAccount>,
}

#[account]
pub struct CustomAccount {
    pub my_custom_data: u64,
    pub some_string: String,
    pub inner: InnerStruct,
    pub a_pubkey: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct InnerStruct {
    pub some_data: u64,
    pub some_other_data: String,
    pub index_map: HashMap<String, u64>,
}

#[event]
/// The transfer event.
pub struct TransferEvent {
    /// The amount transferred.
    /// Another line of comments
    pub amount: u64,
    /// The pubkey the amount was transferred to.
    pub to_pubkey: Pubkey,
}

#[event]
/// The message event.
pub struct MessageEvent {
    /// The message emitted during the transfer.
    pub message: String,
}

// #[event]
// pub struct TransferEvent {
//     pub transfer_details: TransferDetails,
//     pub message: MessageDetails,
// }

// #[derive(AnchorSerialize, AnchorDeserialize, Eq, PartialEq, Clone, Debug)]
// pub struct TransferDetails {
//     pub amount: u64,
//     pub to_pubkey: Pubkey,
// }
// #[derive(AnchorSerialize, AnchorDeserialize, Eq, PartialEq, Clone, Debug)]
// pub enum MessageDetails {
//     Success(String),
//     Error(ErrorMessage),
// }

// #[derive(AnchorSerialize, AnchorDeserialize, Eq, PartialEq, Clone, Debug)]
// pub struct ErrorMessage {
//     pub message: String,
//     pub code: u32,
// }
