use anchor_lang::prelude::*;
use anchor_lang::solana_program;
use anchor_spl::token;
use std::convert::TryFrom;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod named_spl_mints {
    use super::*;
    pub fn create_new_mint(
        ctx: Context<CreateNewMint>,
        mint_bump: u8,
        _attribution_bump: u8,
        name: String,
        decimals: u8,
        mint_authority: Pubkey,
        freeze_authority: Option<Pubkey>,
    ) -> ProgramResult {
        //create the mint account
        let mint_span: u64 = 82;
        let lamports = Rent::get()?.minimum_balance(usize::try_from(mint_span).unwrap());
        solana_program::program::invoke_signed(
            &solana_program::system_instruction::create_account(
                &ctx.accounts.creator.key(),
                &ctx.accounts.mint.key(),
                lamports,
                mint_span,
                &ctx.accounts.token_program.key(),
            ),
            &[
                ctx.accounts.creator.to_account_info(),
                ctx.accounts.mint.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[&[name.clone().to_seed_format().as_bytes(), &[mint_bump]]],
        )?;

        //initialize the mint
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_accounts = anchor_spl::token::InitializeMint {
            mint: ctx.accounts.mint.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        if let Some(freeze_authority) = freeze_authority {
            anchor_spl::token::initialize_mint(
                cpi_ctx,
                decimals,
                &mint_authority,
                Some(&freeze_authority),
            )?;
        } else {
            anchor_spl::token::initialize_mint(cpi_ctx, decimals, &mint_authority, None)?;
        };

        //set the attribution
        ctx.accounts.attribution.mint = ctx.accounts.mint.key();
        ctx.accounts.attribution.name = name;

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(mint_bump: u8, attribution_bump: u8, name: String)]
pub struct CreateNewMint<'info> {
    #[account(mut)]
    creator: Signer<'info>,
    #[account(
        mut,
        seeds = [name.clone().to_seed_format().as_bytes()],
        bump = mint_bump
    )]
    mint: UncheckedAccount<'info>,
    #[account(
        init,
        seeds = [mint.key().as_ref()],
        bump = attribution_bump,
        space = 92,
        payer = creator
    )]
    attribution: Account<'info, Attribution>,
    rent: Sysvar<'info, Rent>,
    system_program: Program<'info, System>,
    token_program: Program<'info, token::Token>,
}

//seed = [mintkey]
//u can go mint -> name
//should change this to just use the metadata program
#[account]
#[derive(Default)]
pub struct Attribution {
    pub mint: Pubkey,
    pub name: String,
}

trait SeedFormat {
    fn to_seed_format(self) -> String;
}
//need to add some error handling to the front end
impl SeedFormat for String {
    fn to_seed_format(mut self) -> String {
        self.make_ascii_lowercase();
        self.retain(|c| !c.is_whitespace());
        self
    }
}

//8 + 32 + 32 = 72 byte
//string is 4 byte setup, plus 1 byte per character. we are doing 16 char max = 20 bytes for the string
//total 92 bytes
//can obvi change later if u want longer names (or shorter)

//u can go name -> mint directly from the mint

/*
    #[account(init,
        mint::decimals = DECIMALS,
        mint::authority = ido_account,
        seeds = [ido_name.as_bytes(), b"redeemable_mint".as_ref()],
        bump = bumps.redeemable_mint,
        payer = ido_authority)]
    pub redeemable_mint: Box<Account<'info, Mint>>,
*/

/*
normal process for making a new mint
1. create mint account via sysProgram.createAccount
2. init the mint account
*/
