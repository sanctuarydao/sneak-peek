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

 // All events are subject to change
 
 #[event]
 pub struct UserCreated {
     pub authority: Pubkey,
     pub has_genesis: bool,
     pub genesis_multiplier: u8,
 }
 
 #[event]
 pub struct StakeEvent {
     pub time: i64,
     pub amount: u64,
     pub user: Pubkey,
 }
 
 #[event]
 pub struct UnstakeEvent {
     pub time: i64,
     pub amount: u64,
     pub user: Pubkey,
 }
 
 #[event]
 pub struct AuthorityUpdated {
     pub old_authority: Pubkey,
     pub new_authority: Pubkey,
     pub time: i64,
 }