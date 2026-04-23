use anchor_lang::prelude::*;

declare_id!("CCnPxPLd4GbxycDTcP12KP98rWtjKCCNcZC4hqHCB1KV");

#[error_code]
pub enum ArenaError {
    #[msg("Score overflow")]
    ScoreOverflow,
    #[msg("Agent name too long")]
    NameTooLong,
    #[msg("Episode already finalized")]
    AlreadyFinalized,
    #[msg("Score threshold not met")]
    ThresholdNotMet,
}

#[program]
pub mod arena {
    use super::*;

    pub fn create_agent(ctx: Context<CreateAgent>, name: String) -> Result<()> {
        require!(name.len() <= 32, ArenaError::NameTooLong);
        let agent = &mut ctx.accounts.agent_identity;
        agent.owner = ctx.accounts.signer.key();
        agent.name = name.clone();
        agent.registered_at = Clock::get()?.unix_timestamp;
        agent.bump = ctx.bumps.agent_identity;
        msg!(
            "Agent '{}' registered by {}",
            name,
            ctx.accounts.signer.key()
        );
        Ok(())
    }

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
        log.finalized = false;

        let rep = &mut ctx.accounts.agent_reputation;
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

    pub fn init_vault(ctx: Context<InitVault>, initial_funding: u64) -> Result<()> {
        // Transfer lamports from signer to vault
        let signer = &ctx.accounts.signer;
        let vault = &ctx.accounts.reward_vault;

        let from = signer.to_account_info();
        let to = vault.to_account_info();

        **from.try_borrow_mut_lamports()? -= initial_funding;
        **to.try_borrow_mut_lamports()? += initial_funding;

        msg!(
            "Vault initialized at {} with {} lamports funded",
            vault.key(),
            initial_funding
        );
        Ok(())
    }

    pub fn finalize_episode(
        ctx: Context<FinalizeEpisode>,
        episode_id: u64,
        score_threshold: u64,
    ) -> Result<()> {
        let log = &mut ctx.accounts.episode_log;

        require!(!log.finalized, ArenaError::AlreadyFinalized);

        let winner_score = log.scores[0].max(log.scores[1]);
        require!(winner_score >= score_threshold, ArenaError::ThresholdNotMet);

        log.finalized = true;

        // transfer reward from vault to winner
        let reward_lamports = 1_000_000; // 0.001 SOL
        let from = ctx.accounts.reward_vault.to_account_info();
        let to = ctx.accounts.signer.to_account_info();

        **from.try_borrow_mut_lamports()? -= reward_lamports;
        **to.try_borrow_mut_lamports()? += reward_lamports;

        msg!(
            "Episode {} finalized — winner score: {} — reward from vault: {} lamports",
            log.episode_id,
            winner_score,
            reward_lamports
        );
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct CreateAgent<'info> {
    #[account(
        init_if_needed,
        payer = signer,
        space = 8 + 32 + 4 + 32 + 8 + 1,
        seeds = [b"agent", signer.key().as_ref()],
        bump
    )]
    pub agent_identity: Account<'info, AgentIdentity>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(episode_id: u64)]
pub struct LogEpisode<'info> {
    #[account(
        init_if_needed,
        payer = signer,
        space = 8 + 8 + 16 + 32 + 8 + 1,
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

#[derive(Accounts)]
pub struct InitVault<'info> {
    #[account(
        init,
        payer = signer,
        space = 8,
        seeds = [b"vault"],
        bump
    )]
    pub reward_vault: Account<'info, RewardVault>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(episode_id: u64)]
pub struct FinalizeEpisode<'info> {
    #[account(
        mut,
        seeds = [b"episode", episode_id.to_le_bytes().as_ref()],
        bump
    )]
    pub episode_log: Account<'info, EpisodeLog>,
    #[account(
        mut,
        seeds = [b"vault"],
        bump
    )]
    pub reward_vault: Account<'info, RewardVault>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct AgentIdentity {
    pub owner: Pubkey,
    pub name: String,
    pub registered_at: i64,
    pub bump: u8,
}

#[account]
pub struct EpisodeLog {
    pub episode_id: u64,
    pub scores: [u64; 2],
    pub episode_hash: [u8; 32],
    pub timestamp: i64,
    pub finalized: bool,
}

#[account]
pub struct AgentReputation {
    pub agent: Pubkey,
    pub total_score: u64,
    pub episodes_played: u32,
    pub bump: u8,
}

#[account]
pub struct RewardVault {
    // Empty account used as a PDA to hold lamports
    // Lamports are stored directly on the account, not in a field
}
