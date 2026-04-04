use anchor_lang::prelude::*;

mod error;
mod seeds;
mod state;
mod instructions;

use instructions::*;

declare_id!("8bDwCU1Y598BoD2BWnchBCeuvQ4rW3DUbV4c3tDvRkVX");

#[program]
pub mod competition_constructor_program {
    use super::*;

}

