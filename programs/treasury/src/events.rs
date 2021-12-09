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
pub struct TreasurerUpdate {
    pub treasurer: Pubkey,
    pub new_treasurer: Pubkey,
    pub time: i64,
}

#[event]
pub struct TransferTokens {
    pub treasurer: Pubkey,
    pub token: Pubkey,
    pub amount: u64,
    pub time: i64,
}