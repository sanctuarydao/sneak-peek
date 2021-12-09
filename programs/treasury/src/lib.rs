/*
 *  Copyright (c) 2021 The Sanctuary Authors (0xPoo, Asylum)
 *
 *  Licensed under the Apache License, Version 2.0 (the "License");
 *  you may not use this file except in compliance with the License.
 *  You may obtain a copy of the License at
 *
 *  http://www.apache.org/licenses/LICENSE-2.0, LICENSE
 *
 *  Unless required by applicable law or agreed to in writing, software
 *  distributed under the License is distributed on an "AS IS" BASIS,
 *  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *  See the License for the specific language governing permissions and
 *  limitations under the License.
 */

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

use crate::errors::ErrorCode;
use crate::events::*;

pub mod errors;
pub mod events;

// Replace with actual program_id
declare_id!("3pccTEiGB8ZRdYX7FShPYRy1ZzQjdFASKSvNNbNduTxr");

#[program]
pub mod treasury {
    use super::*;

    // Treasury Program state
    // `#[state]` is deprecated but it is the easiest way to give the program `state`
    // `#[state]` will likely be replaced by a PDA with static seeds in a future version of this code
    #[state]
    pub struct State {
        // The treasurer
        pub treasurer: Pubkey,
    }

    // State methods
    impl State {
        // constructor
        pub fn new(ctx: Context<Treasurer>) -> Result<Self, ErrorCode> {
            Ok(Self {
                treasurer: *ctx.accounts.treasurer.key,
            })
        }

        // updates the treasurer
        pub fn update_treasurer(&mut self, ctx: Context<UpdateTreasurer>, new_treasurer: Pubkey) -> Result<(), ErrorCode> {
            let clock = &ctx.accounts.clock;

            if &self.treasurer != ctx.accounts.treasurer {
                return Err(ErrorCode::Unauthorized.into());
            }

            let treasurer = self.treasurer.key;

            self.treasurer = new_treasurer;

            emit!(TreasurerUpdate {
                treasurer: treasurer,
                new_treasurer: new_treasurer,
                time: clock.unix_timestamp,
            });

            Ok(())
        }

        // transfers tokens from the treasury to the treasurer
        pub fn transfer_tokens(&mut self, ctx: Context<TransferTokens>, amount: u64, bump: u8) -> Result<(), ErrorCode> {
            let clock = &ctx.accounts.clock;

            if &self.treasurer != ctx.accounts.treasurer {
                return Err(ErrorCode::Unauthorized.into());
            }

            // Program signer
            let seeds = &[
                ctx.accounts.token_mint.to_account_info().key.as_ref(), 
                ctx.accounts.treasurer.key.as_ref(),
                &[bump]
            ];
            let signer = &[&seeds[..]];

            let cpi_accounts = Transfer {
                from: ctx.accounts.treasury_token_account.clone(),
                to: ctx.accounts.treasurer_token_account.clone(),
                authority: ctx.accounts.program_signer.clone(),
            };

            let cpi_program = ctx.accounts.token_program.to_account_info();
            let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
            token::transfer(cpi_ctx, amount)?;

            emit!(TransferTokens {
                treasurer: self.treasurer.key,
                token: *ctx.accounts.token_mint.key,
                amount: amount,
                time: clock.unix_timestamp,
            });

            Ok(())
        }
    }
}

// Constructor context
#[derive(Accounts)]
pub struct Treasurer<'info> {
    // The new treasurer
    #[account(signer)]
    pub treasurer: AccountInfo<'info>,
}

// Treasurer Update context
#[derive(Accounts)]
pub struct UpdateTreasurer<'info> {
    // The treasurer
    #[account(signer)]
    pub treasurer: AccountInfo<'info>,

    // The clock, for timestamping
    pub clock: Sysvar<'info, Clock>,
}

// Token transfer context
#[derive(Accounts)]
pub struct TransferTokens<'info> {
    // The program signer
    pub program_signer: AccountInfo<'info>,

    // The treasurer
    #[account(signer)]
    pub treasurer: AccountInfo<'info>,

    // The token mint
    #[account(mut)]
    pub token_mint: Account<'info, Mint>,

    // The treasurers token account
    #[account(mut,
    "treasurer_token_account.owner == *treasurer.key")]
    pub treasurer_token_account: Account<'info, TokenAccount>,

    // The treasury's token account
    #[account(mut,
    "treasury_token_account.owner == *program_signer.key")]
    pub treasury_token_account: Account<'info, TokenAccount>,

    // The token program
    #[account(executable, "token_program.key == &token::ID")]
    pub token_program: Program<'info, Token>,

    // The clock, for timestamping
    pub clock: Sysvar<'info, Clock>,
}