use anchor_lang::prelude::*;

declare_id!("J69R1WJjHAdW3Dv6ionquKsXLdzmzEbeuNSLmWn5VhYz");

#[program]
pub mod voting {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        crate::instructions::initialize::handle_initialize(ctx)
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        crate::instructions::increment::handle_increment(ctx)
    }
}
