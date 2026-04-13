use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct SwissSystemPointsAwardArgs {}


#[derive(Accounts)]
#[instruction(args: SwissSystemPointsAwardArgs)]
pub struct SwissSystemPointsAward<'info> {


    
}