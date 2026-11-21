use anchor_lang::prelude::*;
use anchor_lang::solana_program::hash::hashv;

// Playground fills this in automatically when you click Build.
declare_id!("11111111111111111111111111111111");

const MAX_TEXT_LEN: usize = 280;

#[program]
pub mod sealed_prediction {
    use super::*;

    /// Step 1: post only the fingerprint (hash) of the prediction. The text stays secret.
    pub fn commit(ctx: Context<Commit>, id: u64, hash: [u8; 32]) -> Result<()> {
        let prediction = &mut ctx.accounts.prediction;
        prediction.author = ctx.accounts.author.key();
        prediction.id = id;
        prediction.hash = hash;
        prediction.committed_at = Clock::get()?.unix_timestamp;
        prediction.revealed_at = 0;
        prediction.text = String::new();
        prediction.bump = ctx.bumps.prediction;
        Ok(())
    }

    /// Step 2: publish the text + salt. The program re-computes the hash and checks it matches.
    pub fn reveal(ctx: Context<Reveal>, text: String, salt: [u8; 32]) -> Result<()> {
        require!(text.len() <= MAX_TEXT_LEN, PredictionError::TextTooLong);

        let prediction = &mut ctx.accounts.prediction;
        require!(prediction.revealed_at == 0, PredictionError::AlreadyRevealed);

        let computed = hashv(&[&salt, text.as_bytes()]).to_bytes();
        require!(computed == prediction.hash, PredictionError::HashMismatch);

        prediction.text = text;
        prediction.revealed_at = Clock::get()?.unix_timestamp;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct Commit<'info> {
    #[account(
        init,
        payer = author,
        space = 8 + Prediction::INIT_SPACE,
        seeds = [b"prediction", author.key().as_ref(), &id.to_le_bytes()],
        bump
    )]
    pub prediction: Account<'info, Prediction>,
    #[account(mut)]
    pub author: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Reveal<'info> {
    #[account(
        mut,
        seeds = [b"prediction", author.key().as_ref(), &prediction.id.to_le_bytes()],
        bump = prediction.bump,
        has_one = author
    )]
    pub prediction: Account<'info, Prediction>,
    pub author: Signer<'info>,
}

#[account]
#[derive(InitSpace)]
pub struct Prediction {
    pub author: Pubkey,
    pub id: u64,
    pub hash: [u8; 32],
    pub committed_at: i64,
    pub revealed_at: i64,
    #[max_len(280)]
    pub text: String,
    pub bump: u8,
}

#[error_code]
pub enum PredictionError {
    #[msg("Prediction text is longer than 280 bytes")]
    TextTooLong,
    #[msg("This prediction has already been revealed")]
    AlreadyRevealed,
    #[msg("Text + secret don't match the sealed fingerprint")]
    HashMismatch,
}
