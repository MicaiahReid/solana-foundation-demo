use anchor_lang::prelude::*;
use borsh::BorshDeserialize;

declare_id!("784RuaFufDQCrsTgMgtVFEciombEWXjyaeaJa64EevR");

#[program]
pub mod goodbye_cruel_world {

    use anchor_lang::prelude::*;
    use anchor_lang::system_program::{transfer, Transfer};

    use super::*;
    pub fn hello(ctx: Context<SolTransfer>, string_val: String, amount: u64) -> Result<()> {
        let from_pubkey = ctx.accounts.sender.to_account_info();
        let to_pubkey = ctx.accounts.recipient.to_account_info();
        let program_id = ctx.accounts.system_program.to_account_info();

        let cpi_context = CpiContext::new(
            program_id,
            Transfer {
                from: from_pubkey,
                to: to_pubkey.clone(),
            },
        );
        transfer(cpi_context, amount)?;
        emit!(TransferEvent {
            amount,
            to_pubkey: to_pubkey.key()
        });

        emit_cpi!(TransferEvent {
            amount,
            to_pubkey: to_pubkey.key()
        });

        let string_val = format!(
            "Goodbye, {}. (Sent from {})",
            string_val,
            ctx.program_id.to_string()
        );
        emit!(MessageEvent {
            message: string_val.clone()
        });
        emit_cpi!(MessageEvent {
            message: string_val.clone()
        });
        msg!("{}", string_val);
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
}

#[event]
pub struct TransferEvent {
    pub amount: u64,
    pub to_pubkey: Pubkey,
}

#[event]
pub struct MessageEvent {
    pub message: String,
}
