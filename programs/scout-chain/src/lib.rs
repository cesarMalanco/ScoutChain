use anchor_lang::prelude::*;

declare_id!("CXYaPQ451V8DD43pWCepNyddkf2Pnjk3mqvLgchJR5Fb");

// ===================
// PROGRAM
// ===================
#[program]
pub mod scout_chain {
    use super::*;

    // ==== PLAYER =====
    // Create player
    pub fn create_player(
        ctx: Context<CreatePlayer>,
        name: String,
        position: String,
        age: u8,
        stats: String,
        video_url: String,
    ) -> Result<()> {
        let player = &mut ctx.accounts.player;

        // Saving data
        player.authority = ctx.accounts.authority.key();
        player.name = name;
        player.position = position;
        player.age = age;
        player.stats = stats;
        player.video_url = video_url;

        // Initializing metrics
        player.review_count = 0;
        player.rating_sum = 0;
        player.is_active = true;

        Ok(())
    }

    //Read player
    pub fn read_player(ctx: Context<UpdatePlayer>) -> Result<()> {
        let player = &ctx.accounts.player;
        msg!("Player info: {:#?}", player);
        Ok(())
    }

    //Update player
    pub fn update_player(
        ctx: Context<UpdatePlayer>,
        name: String,
        position: String,
        age: u8,
        stats: String,
        video_url: String,
    ) -> Result<()> {
        let player = &mut ctx.accounts.player;

        require!(
            player.authority == ctx.accounts.authority.key(),
            ErrorCode::Unauthorized
        );

        player.name = name;
        player.position = position;
        player.age = age;
        player.stats = stats;
        player.video_url = video_url;

        Ok(())
    }

    // Deactive player
    pub fn deactivate_player(ctx: Context<ModifyPlayerStatus>) -> Result<()> {
        let player = &mut ctx.accounts.player;

        require!(
            player.authority == ctx.accounts.authority.key(),
            ErrorCode::Unauthorized
        );

        player.is_active = false;

        Ok(())
    }

    // ==== SCOUT =====
    // Create scout
    pub fn create_scout(
        ctx: Context<CreateScout>,
        name: String,
        organization: String,
        experience_years: u8,
    ) -> Result<()> {
        let scout = &mut ctx.accounts.scout;

        // Saving data
        scout.authority = ctx.accounts.authority.key();
        scout.name = name;
        scout.organization = organization;
        scout.experience_years = experience_years;

        // Initializing metrics
        scout.reviews_given = 0;
        scout.is_active = true;

        Ok(())
    }
    
    // Read scout
    pub fn read_scout(ctx: Context<UpdateScout>) -> Result<()> {
        let scout = &ctx.accounts.scout;
        msg!("Scout info: {:#?}", scout);
        Ok(())
    }

    // Update scout
    pub fn update_scout(
        ctx: Context<UpdateScout>,
        name: String,
        organization: String,
        experience_years: u8,
    ) -> Result<()> {
        let scout = &mut ctx.accounts.scout;

        require!(
            scout.authority == ctx.accounts.authority.key(),
            ErrorCode::Unauthorized
        );

        scout.name = name;
        scout.organization = organization;
        scout.experience_years = experience_years;

        Ok(())
    }

    // Deactive scout
    pub fn deactivate_scout(ctx: Context<ModifyScoutStatus>) -> Result<()> {
        let scout = &mut ctx.accounts.scout;

        require!(
            scout.authority == ctx.accounts.authority.key(),
            ErrorCode::Unauthorized
        );

        scout.is_active = false;

        Ok(())
    }

    // ==== REVIEW =====
    // Create review
    pub fn create_review(ctx: Context<CreateReview>, rating: u8, comment: String) -> Result<()> {
        let review = &mut ctx.accounts.review;
        let player = &mut ctx.accounts.player;
        let scout = &mut ctx.accounts.scout;

        require!(rating >= 1 && rating <= 5, ErrorCode::InvalidRating);
        require!(player.is_active, ErrorCode::InactiveProfile);
        require!(scout.is_active, ErrorCode::InactiveProfile);
        require!(
            ctx.accounts.authority.key() == scout.authority,
            ErrorCode::Unauthorized
        );

        // Save review
        review.player = player.key();
        review.scout = scout.key();
        review.rating = rating;
        review.comment = comment;
        review.timestamp = Clock::get()?.unix_timestamp;

        // Update metrics
        player.review_count += 1;
        player.rating_sum += rating as u32;
        scout.reviews_given += 1;

        Ok(())
    }

    // Read review
    pub fn read_review(ctx: Context<UpdateReview>) -> Result<()> {
        let review = &ctx.accounts.review;
        msg!("Review info: {:#?}", review);
        Ok(())
    }

    // Update review
    pub fn update_review(ctx: Context<UpdateReview>, new_comment: String) -> Result<()> {
        let review = &mut ctx.accounts.review;
        let scout = &ctx.accounts.scout;

        require!(
            scout.authority == ctx.accounts.authority.key(),
            ErrorCode::Unauthorized
        );

        review.comment = new_comment;

        Ok(())
    }

    // Delete review
    pub fn delete_review(_ctx: Context<DeleteReview>) -> Result<()> {
        Ok(())
    }
}

// ===================
// ACCOUNTS
// ===================
// Player
#[account]
#[derive(InitSpace, Debug)]
pub struct Player {
    pub authority: Pubkey,

    #[max_len(32)]
    pub name: String,

    #[max_len(20)]
    pub position: String,

    pub age: u8,

    #[max_len(100)]
    pub stats: String,

    #[max_len(100)]
    pub video_url: String,

    pub review_count: u32,

    pub rating_sum: u32,

    pub is_active: bool,
}

// Scout
#[account]
#[derive(InitSpace, Debug)]
pub struct Scout {
    pub authority: Pubkey, 

    #[max_len(32)]
    pub name: String,

    #[max_len(60)]
    pub organization: String,

    pub experience_years: u8,

    pub reviews_given: u32,

    pub is_active: bool,
}

// Review
#[account]
#[derive(InitSpace, Debug)]
pub struct Review {
    pub player: Pubkey,

    pub scout: Pubkey,

    pub rating: u8,

    #[max_len(100)]
    pub comment: String,

    pub timestamp: i64,
}

// ===================
// CONTEXTS
// ===================
// Create player
#[derive(Accounts)]
pub struct CreatePlayer<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + Player::INIT_SPACE,
        seeds = [b"player", authority.key().as_ref()],
        bump
    )]
    pub player: Account<'info, Player>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

// Update player
#[derive(Accounts)]
pub struct UpdatePlayer<'info> {
    #[account(mut)]
    pub player: Account<'info, Player>,
    pub authority: Signer<'info>,
}

// Modify player status
#[derive(Accounts)]
pub struct ModifyPlayerStatus<'info> {
    #[account(mut)]
    pub player: Account<'info, Player>,
    pub authority: Signer<'info>,
}

// Create scout
#[derive(Accounts)]
pub struct CreateScout<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + Scout::INIT_SPACE,
        seeds = [b"scout", authority.key().as_ref()],
        bump
    )]
    pub scout: Account<'info, Scout>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

// Update scout
#[derive(Accounts)]
pub struct UpdateScout<'info> {
    #[account(mut)]
    pub scout: Account<'info, Scout>,
    pub authority: Signer<'info>,
}

// Modify scout status
#[derive(Accounts)]
pub struct ModifyScoutStatus<'info> {
    #[account(mut)]
    pub scout: Account<'info, Scout>,
    pub authority: Signer<'info>,
}

// Create review
#[derive(Accounts)]
pub struct CreateReview<'info> {
    #[account(
        mut,
        seeds = [b"player", player.authority.as_ref()],
        bump
    )]
    pub player: Account<'info, Player>,

    #[account(
        mut,
        seeds = [b"scout", authority.key().as_ref()],
        bump
    )]
    pub scout: Account<'info, Scout>,

    #[account(
        init,
        payer = authority,
        space = 8 + Review::INIT_SPACE,
        seeds = [
            b"review",
            player.key().as_ref(),
            scout.key().as_ref(),
            &player.review_count.to_le_bytes()
        ],
        bump
    )]
    pub review: Account<'info, Review>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

// Update review
#[derive(Accounts)]
pub struct UpdateReview<'info> {
    #[account(mut)]
    pub review: Account<'info, Review>,
    pub scout: Account<'info, Scout>,
    pub authority: Signer<'info>,
}

// DeleteReview
#[derive(Accounts)]
pub struct DeleteReview<'info> {
    #[account(mut, close = authority)]
    pub review: Account<'info, Review>,
    #[account(mut)]
    pub authority: Signer<'info>,
}

// ==== ERRORS ====
#[error_code]
pub enum ErrorCode {
    #[msg("Rating must be between 1 & 5")]
    InvalidRating,

    #[msg("Unauthorized action")]
    Unauthorized,

    #[msg("Profile is inactive")]
    InactiveProfile,
}