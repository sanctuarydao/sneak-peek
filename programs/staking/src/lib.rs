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

///
/// NOTE: Portions of this code have been redacted to stop it being copied 
///       (namely the rebasing mechanism).
///
/// NOTE: This is only a sneak peek, and is only meant to provide evidence that
///       the `fork` is being worked on.
///
/// NOTE: This code is incomplete, and does not reflect what the production
///       version will be. However, it does compile.
///
/// NOTE: Wherever code has been removed, [redacted] has been put in it's place.
///
#![allow(deprecated)]

use anchor_lang::prelude::*;
use anchor_lang::solana_program::program_option::COption;
use anchor_spl::token::{self, Mint, Token, TokenAccount, MintTo, Burn};

use crate::events::*;
use crate::errors::ErrorCode;

pub mod events;
pub mod errors;

// Replace with actual program_id
declare_id!("3pccTEiGB8ZRdYX7FShPYRy1ZzQjdFASKSvNNbNduTxr");

#[program]
pub mod staking {
    use super::*;

    // Staking Program state
    // `#[state]` is deprecated but it is the easiest way to give the program `state`
    // `#[state]` will likely be replaced by a PDA with static seeds in a future version of this code
    #[state]
    pub struct State {
        // [redacted]

        // [redacted]

        // [redacted]

        // The staking program authority
        pub authority: Pubkey,

        // The total amount staked
        // needed because we burn $SANC when it is staked (for now)
        pub total_staked: u64,
    }

    impl State {
        pub fn new(ctx: Context<Authority>) -> Result<Self, ErrorCode> {
            Ok(Self {
                authority: *ctx.accounts.authority.key,
                total_staked: 0,
            })
        }

        // Sets `authority` to `new_authority`
        pub fn update_authority(&mut self, ctx: Context<UpdateAuthority>, new_authority: Pubkey) -> Result<(), ErrorCode> {
            let clock = &ctx.accounts.clock;

            if &self.authority != ctx.accounts.authority.key {
                return Err(ErrorCode::Unauthorized.into());
            }
            
            let old_authority = self.authority;

            self.authority = new_authority;

            emit!(AuthorityUpdated {
                old_authority: old_authority,
                new_authority: new_authority,
                time: clock.unix_timestamp,
            });

            Ok(())
        }

        // Stakes `amount` $SANC
        pub fn stake(&mut self, ctx: Context<Stake>, amount: u64, bump: u8) -> ProgramResult {
            let clock = &ctx.accounts.clock;

            // Burn $SANC from the user
            let cpi_accounts = Burn {
                mint: ctx.accounts.sanc_mint.to_account_info(),
                to: ctx.accounts.user_sanc.to_account_info(),
                authority: ctx.accounts.authority.clone(),
            };

            let cpi_program = ctx.accounts.token_program.to_account_info();
            let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
            token::burn(cpi_ctx, amount)?;

            // Mint $sSANC 1:1 to the user
            let seeds = &[ctx.accounts.sanc_mint.to_account_info().key.as_ref(), &[bump]];
            let signer = &[&seeds[..]];
            let cpi_accounts = MintTo {
                mint: ctx.accounts.staked_sanc_mint.to_account_info(),
                to: ctx.accounts.user_staked_sanc.to_account_info(),
                authority: ctx.accounts.program_signer.clone(),
            };
            
            let cpi_program = ctx.accounts.token_program.to_account_info();
            let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
            token::mint_to(cpi_ctx, amount)?;

            // subject to change
            emit!(StakeEvent {
                time: clock.unix_timestamp,
                amount: amount,
                user: *ctx.accounts.authority.key,
            });

            // Increment `total_staked`
            self.total_staked = self.total_staked.checked_add(amount).unwrap();

            Ok(())
        }

        // unstakes `amount` $sSANC
        pub fn unstake(&mut self, ctx: Context<Unstake>, amount: u64, bump: u8) -> ProgramResult {
            let clock = &ctx.accounts.clock;
            // Burn $sSANC from the user
            let cpi_accounts = Burn {
                mint: ctx.accounts.staked_sanc_mint.to_account_info(),
                to: ctx.accounts.user_staked_sanc.to_account_info(),
                authority: ctx.accounts.authority.clone(),
            };

            // might change `token_program` from `Program` to `AccountInfo` and use `clone()` instead
            let cpi_program = ctx.accounts.token_program.to_account_info();
            let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
            token::burn(cpi_ctx, amount)?;

            // Mint $SANC to the user
            // [redacted]
            let seeds = &[ctx.accounts.sanc_mint.to_account_info().key.as_ref(), &[bump]];
            let signer = &[&seeds[..]];
            let cpi_accounts = MintTo {
                mint: ctx.accounts.sanc_mint.to_account_info(),
                to: ctx.accounts.user_sanc.to_account_info(),
                authority: ctx.accounts.program_signer.clone(),
            };

            let cpi_program = ctx.accounts.token_program.to_account_info();
            let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
            // [redacted]
            token::mint_to(cpi_ctx, amount)?;
            
            // [redacted]
            emit!(UnstakeEvent {
                time: clock.unix_timestamp,
                amount: amount,
                user: *ctx.accounts.authority.key,
            });

            // [redacted]
            // decrement `total_staked`
            self.total_staked = self.total_staked.checked_add(amount).unwrap();

            Ok(())
        }

        // [redacted]


        // [redacted]
    }

    // Initializes a user's account
    pub fn initialize_user(ctx: Context<InitializeUser>) -> ProgramResult {
        let user_genesis = &mut ctx.accounts.user_genesis;

        user_genesis.authority = *ctx.accounts.authority.key;
        user_genesis.has_genesis = false;
        user_genesis.genesis_multiplier = 0;

        // event may be removed as it is not needed
        emit!(UserCreated {
            authority: *ctx.accounts.authority.key,
            has_genesis: false,
            genesis_multiplier: 0,
        });

        Ok(())
    }
}

// Authority context
#[derive(Accounts)]
pub struct Authority<'info> {
    // The user/authority
    #[account(signer)]
    pub authority: AccountInfo<'info>,
}

// UpdateAuthority context
#[derive(Accounts)]
pub struct UpdateAuthority<'info> {
    // The user/authority
    #[account(signer)]
    pub authority: AccountInfo<'info>,

    // clock for timestamping
    pub clock: Sysvar<'info, Clock>,
}

// InitializeUser context
#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct InitializeUser<'info> {
    // Create a PDA that stores a user's genesis status
    #[account(
        init,
        seeds = [b"user_genesis".as_ref(), authority.key.as_ref()],
        bump = bump,
        payer = authority
    )]
    pub user_genesis: ProgramAccount<'info, UserGenesis>,

    // The user
    #[account(signer)]
    pub authority: AccountInfo<'info>,

    // The system program, required by the runtime
    pub system_program: Program<'info, System>,

    // The rent sysvar
    pub rent: Sysvar<'info, Rent>,
}

// Stake context
// TODO: do account checks
#[derive(Accounts)]
pub struct Stake<'info> {
    // The program signer
    pub program_signer: AccountInfo<'info>,

    // The user
    #[account(signer)]
    pub authority: AccountInfo<'info>,

    // The $SANC mint
    #[account(mut,
    "sanc_mint.mint_authority == COption::Some(*program_signer.key)")]
    pub sanc_mint: Account<'info, Mint>,

    // The user's $SANC account
    #[account(mut, "user_sanc.owner == *authority.key")]
    pub user_sanc: Account<'info, TokenAccount>,

    // The program's $SANC account
    //#[account(mut)]
    //pub program_sanc: Account<'info, TokenAccount>,

    // The $sSANC mint
    #[account(mut,
    "staked_sanc_mint.mint_authority == COption::Some(*program_signer.key)")]
    pub staked_sanc_mint: Account<'info, Mint>,

    // The user's $sSANC account
    #[account(mut, "user_staked_sanc.owner == *authority.key")]
    pub user_staked_sanc: Account<'info, TokenAccount>,

    // We know it's address and that it's executable
    #[account(executable, "token_program.key == &token::ID")]
    pub token_program: Program<'info, Token>,

    // clock for timestamping
    pub clock: Sysvar<'info, Clock>,
}

// Unstake context
// TODO: do account checks
#[derive(Accounts)]
pub struct Unstake<'info> {
    // The program signer
    pub program_signer: AccountInfo<'info>,

    // The user
    #[account(signer)]
    pub authority: AccountInfo<'info>,

    // The $SANC mint
    #[account(mut,
    "sanc_mint.mint_authority == COption::Some(*program_signer.key)")]
    pub sanc_mint: Account<'info, Mint>,

    // The user's $SANC account
    #[account(mut, "user_sanc.owner == *authority.key")]
    pub user_sanc: Account<'info, TokenAccount>,

    // The Program's $SANC account
    //#[account(mut)]
    //pub program_sanc: Account<'info, TokenAccount>,

    // The $sSANC mint
    #[account(mut,
    "staked_sanc_mint.mint_authority == COption::Some(*program_signer.key)")]
    pub staked_sanc_mint: Account<'info, Mint>,

    // The user's $sSANC account
    #[account(mut, "user_staked_sanc.owner == *authority.key")]
    pub user_staked_sanc: Account<'info, TokenAccount>,

    // We know it's address and that it's executable
    #[account(executable, "token_program.key == &token::ID")]
    pub token_program: Program<'info, Token>,

    // The clock sysvar, to provide the unix timestamp for events
    pub clock: Sysvar<'info, Clock>,
}

// Account that stores a user's genesis status
// Genesis mechanism subject to change
#[account]
#[derive(Default)]
pub struct UserGenesis {
    // The account this account stores info about
    pub authority: Pubkey,

    // If the user has genesis
    pub has_genesis: bool,

    // The user's genesis multiplier
    pub genesis_multiplier: u8,
}