use anchor_lang::prelude::*;

declare_id!("CCnPxPLd4GbxycDTcP12KP98rWtjKCCNcZC4hqHCB1KV");

#[error_code]
pub enum ArenaError {
    #[msg("Score overflow")]
    ScoreOverflow,
}

#[program]
pub mod arena {
    use super::*;

    pub fn log_episode(
        ctx: Context<LogEpisode>,
        episode_id: u64,
        scores: [u64; 2],
        episode_hash: [u8; 32],
    ) -> Result<()> {
        let log = &mut ctx.accounts.episode_log;
        log.episode_id = episode_id;
        log.scores = scores;
        log.episode_hash = episode_hash;
        log.timestamp = Clock::get()?.unix_timestamp;

        let rep = &mut ctx.accounts.agent_reputation;
        // Initialize agent reputation on first creation
        if rep.agent == Pubkey::default() {
            rep.agent = ctx.accounts.signer.key();
            rep.total_score = 0;
            rep.episodes_played = 0;
            rep.bump = ctx.bumps.agent_reputation;
        }

        rep.total_score = rep
            .total_score
            .checked_add(scores[0])
            .ok_or(error!(ArenaError::ScoreOverflow))?;
        rep.episodes_played += 1;

        msg!(
            "Episode {} logged - scores: [{}, {}]",
            episode_id,
            scores[0],
            scores[1]
        );
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(episode_id: u64)]
pub struct LogEpisode<'info> {
    #[account(
        init_if_needed,
        payer = signer,
        space = 8 + 8 + 16 + 32 + 8,
        seeds = [b"episode", episode_id.to_le_bytes().as_ref()],
        bump
    )]
    pub episode_log: Account<'info, EpisodeLog>,

    #[account(
        init_if_needed,
        payer = signer,
        space = 8 + 32 + 8 + 4 + 1,
        seeds = [b"reputation", signer.key().as_ref()],
        bump
    )]
    pub agent_reputation: Account<'info, AgentReputation>,

    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct EpisodeLog {
    pub episode_id: u64,
    pub scores: [u64; 2],
    pub episode_hash: [u8; 32],
    pub timestamp: i64,
}

#[account]
pub struct AgentReputation {
    pub agent: Pubkey,
    pub total_score: u64,
    pub episodes_played: u32,
    pub bump: u8,
}
