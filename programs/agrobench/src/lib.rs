use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

declare_id!("EytN8UaXrfTQc6Pq4AdQbQyJwUX37ddXsV7URayBBLrN");

/// Circle USDC (Solana Devnet). Passe este mint em `initialize`.
pub const USDC_MINT_DEVNET: Pubkey =
    pubkey!("4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU");

/// 10 USDC em unidades base (6 decimais).
pub const STAKE_10_USDC: u64 = 10_000_000;

pub const POOL_SEED: &[u8] = b"pool";
pub const STAKE_SEED: &[u8] = b"stake";

#[program]
pub mod agrobench {
    use super::*;

    /// Cria o PDA `pool` e a ATA USDC do pool. Só uma vez (init do PDA).
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let pool = &mut ctx.accounts.pool;
        pool.authority = ctx.accounts.authority.key();
        pool.usdc_mint = ctx.accounts.usdc_mint.key();
        pool.bump = ctx.bumps.pool;
        Ok(())
    }

    /// Trava USDC do produtor na ATA do PDA `stake`. Soma em `Stake.amount`.
    /// `amount` em unidades base (10 USDC = 10_000_000).
    pub fn lock_stake(ctx: Context<LockStake>, amount: u64) -> Result<()> {
        require!(amount > 0, AgrobenchError::InsufficientStake);
        require!(
            ctx.accounts.producer_ata.amount >= amount,
            AgrobenchError::InsufficientStake
        );

        let stake = &mut ctx.accounts.stake;
        if stake.producer == Pubkey::default() {
            stake.producer = ctx.accounts.producer.key();
            stake.amount = 0;
            stake.bump = ctx.bumps.stake;
        } else {
            require_keys_eq!(
                stake.producer,
                ctx.accounts.producer.key(),
                AgrobenchError::Unauthorized
            );
        }

        stake.amount = stake
            .amount
            .checked_add(amount)
            .ok_or(AgrobenchError::Overflow)?;

        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.producer_ata.to_account_info(),
                    to: ctx.accounts.stake_ata.to_account_info(),
                    authority: ctx.accounts.producer.to_account_info(),
                },
            ),
            amount,
        )?;
        Ok(())
    }

    /// Devolve `Stake.amount` ao produtor e zera o stake.
    /// `authority` deve co-assinar (treasury / protocolo após o ciclo).
    pub fn release_stake(ctx: Context<ReleaseStake>) -> Result<()> {
        let amount = ctx.accounts.stake.amount;
        require!(amount > 0, AgrobenchError::InsufficientStake);
        require_keys_eq!(
            ctx.accounts.producer_ata.mint,
            ctx.accounts.stake_ata.mint,
            AgrobenchError::Unauthorized
        );
        require_keys_eq!(
            ctx.accounts.stake_ata.owner,
            ctx.accounts.stake.key(),
            AgrobenchError::Unauthorized
        );
        require_keys_eq!(
            ctx.accounts.producer_ata.owner,
            ctx.accounts.producer.key(),
            AgrobenchError::Unauthorized
        );

        let producer_key = ctx.accounts.producer.key();
        let bump = ctx.accounts.stake.bump;
        let seeds: &[&[u8]] = &[STAKE_SEED, producer_key.as_ref(), &[bump]];
        let signer = &[seeds];

        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.stake_ata.to_account_info(),
                    to: ctx.accounts.producer_ata.to_account_info(),
                    authority: ctx.accounts.stake.to_account_info(),
                },
                signer,
            ),
            amount,
        )?;

        ctx.accounts.stake.amount = 0;
        Ok(())
    }

    /// Crédito da assinatura: USDC da treasury → ATA do pool.
    pub fn credit_pool(ctx: Context<CreditPool>, amount: u64) -> Result<()> {
        require!(amount > 0, AgrobenchError::InsufficientStake);
        require!(
            ctx.accounts.from_ata.amount >= amount,
            AgrobenchError::InsufficientStake
        );

        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.from_ata.to_account_info(),
                    to: ctx.accounts.pool_ata.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                },
            ),
            amount,
        )?;
        Ok(())
    }

    /// Split mensal on-chain: `amounts[i]` → `remaining_accounts[i]` (dest ATA).
    pub fn distribute<'info>(
        ctx: Context<'_, '_, 'info, 'info, Distribute<'info>>,
        amounts: Vec<u64>,
    ) -> Result<()> {
        require!(
            ctx.remaining_accounts.len() == amounts.len(),
            AgrobenchError::LengthMismatch
        );

        let mut total: u64 = 0;
        for amount in &amounts {
            total = total
                .checked_add(*amount)
                .ok_or(AgrobenchError::Overflow)?;
        }
        require!(
            ctx.accounts.pool_ata.amount >= total,
            AgrobenchError::InsufficientStake
        );

        let bump = ctx.accounts.pool.bump;
        let seeds: &[&[u8]] = &[POOL_SEED, &[bump]];
        let signer = &[seeds];

        for (i, amount) in amounts.iter().enumerate() {
            if *amount == 0 {
                continue;
            }
            let dest = ctx.remaining_accounts[i].clone();
            token::transfer(
                CpiContext::new_with_signer(
                    ctx.accounts.token_program.to_account_info(),
                    Transfer {
                        from: ctx.accounts.pool_ata.to_account_info(),
                        to: dest,
                        authority: ctx.accounts.pool.to_account_info(),
                    },
                    signer,
                ),
                *amount,
            )?;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    /// Treasury / admin do protocolo. Payer do `init`.
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = 8 + Pool::INIT_SPACE,
        seeds = [POOL_SEED],
        bump
    )]
    pub pool: Account<'info, Pool>,

    pub usdc_mint: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = usdc_mint,
        associated_token::authority = pool
    )]
    pub pool_ata: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct LockStake<'info> {
    pub producer: Signer<'info>,

    /// Fee payer (treasury). Deve co-assinar; paga o `init` do PDA/ATA.
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init_if_needed,
        payer = authority,
        space = 8 + Stake::INIT_SPACE,
        seeds = [STAKE_SEED, producer.key().as_ref()],
        bump
    )]
    pub stake: Account<'info, Stake>,

    #[account(
        mut,
        associated_token::mint = usdc_mint,
        associated_token::authority = producer
    )]
    pub producer_ata: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = usdc_mint,
        associated_token::authority = stake
    )]
    pub stake_ata: Account<'info, TokenAccount>,

    pub usdc_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ReleaseStake<'info> {
    pub producer: Signer<'info>,

    /// Treasury / protocolo: co-assinatura obrigatória para destravar.
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [STAKE_SEED, producer.key().as_ref()],
        bump = stake.bump,
        has_one = producer @ AgrobenchError::Unauthorized
    )]
    pub stake: Account<'info, Stake>,

    #[account(mut)]
    pub producer_ata: Account<'info, TokenAccount>,

    #[account(mut)]
    pub stake_ata: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct CreditPool<'info> {
    pub authority: Signer<'info>,

    #[account(
        seeds = [POOL_SEED],
        bump = pool.bump,
        has_one = authority @ AgrobenchError::Unauthorized
    )]
    pub pool: Account<'info, Pool>,

    /// ATA USDC da treasury.
    #[account(
        mut,
        constraint = from_ata.mint == pool.usdc_mint @ AgrobenchError::Unauthorized,
        constraint = from_ata.owner == authority.key() @ AgrobenchError::Unauthorized
    )]
    pub from_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = pool_ata.mint == pool.usdc_mint @ AgrobenchError::Unauthorized,
        constraint = pool_ata.owner == pool.key() @ AgrobenchError::Unauthorized
    )]
    pub pool_ata: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Distribute<'info> {
    pub authority: Signer<'info>,

    #[account(
        seeds = [POOL_SEED],
        bump = pool.bump,
        has_one = authority @ AgrobenchError::Unauthorized
    )]
    pub pool: Account<'info, Pool>,

    #[account(
        mut,
        constraint = pool_ata.mint == pool.usdc_mint @ AgrobenchError::Unauthorized,
        constraint = pool_ata.owner == pool.key() @ AgrobenchError::Unauthorized
    )]
    pub pool_ata: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

#[account]
#[derive(InitSpace)]
pub struct Pool {
    pub authority: Pubkey,
    pub usdc_mint: Pubkey,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct Stake {
    pub producer: Pubkey,
    pub amount: u64,
    pub bump: u8,
}

#[error_code]
pub enum AgrobenchError {
    #[msg("Insufficient stake")]
    InsufficientStake,
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Length mismatch")]
    LengthMismatch,
    #[msg("Overflow")]
    Overflow,
}
