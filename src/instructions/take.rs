use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    program_error::ProgramError,
    pubkey::{create_program_address, find_program_address},
    ProgramResult,
};
use pinocchio_token::{instructions::Transfer, state::TokenAccount};

use crate::{helpers::*, Escrow};

///accounts
/// instruction data
/// business logic

pub struct TakeAccounts<'a> {
    pub taker: &'a AccountInfo,
    pub maker: &'a AccountInfo,
    pub escrow: &'a AccountInfo,
    pub mint_a: &'a AccountInfo,
    pub mint_b: &'a AccountInfo,
    pub vault: &'a AccountInfo,
    pub taker_ata_a: &'a AccountInfo,
    pub maker_ata_b: &'a AccountInfo,
    pub taker_ata_b: &'a AccountInfo,
    pub associated_token_program: &'a AccountInfo,
    pub token_program: &'a AccountInfo,
    pub system_program: &'a AccountInfo,
}

impl<'a> TryFrom<&'a [AccountInfo]> for TakeAccounts<'a> {
    type Error = ProgramError;
    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let [taker, maker, escrow, mint_a, mint_b, vault, taker_ata_a, maker_ata_b, taker_ata_b, associated_token_program, token_program, system_program] =
            accounts
        else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        SignerAccount::check(taker)?;
        ProgramAccount::check(escrow)?;
        MintInterface::check(mint_a)?;
        MintInterface::check(mint_b)?;
        AssociatedTokenAccount::check(taker_ata_b, taker, mint_b, token_program)?;
        AssociatedTokenAccount::check(vault, escrow, mint_a, token_program)?;

        Ok(Self {
            taker,
            maker,
            escrow,
            mint_a,
            mint_b,
            vault,
            taker_ata_a,
            maker_ata_b,
            taker_ata_b,
            associated_token_program,
            token_program,
            system_program,
        })
    }
}

pub struct Take<'a> {
    pub accounts: TakeAccounts<'a>,
}

impl<'a> TryFrom<&'a [AccountInfo]> for Take<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let accounts = TakeAccounts::try_from(accounts)?;

        //Initialize necessary accounts
        AssociatedTokenAccount::init_if_needed(
            accounts.taker_ata_a,
            accounts.mint_a,
            accounts.taker,
            accounts.taker,
            accounts.system_program,
            accounts.token_program,
        )?;

        AssociatedTokenAccount::init_if_needed(
            accounts.maker_ata_b,
            accounts.mint_b,
            accounts.taker,
            accounts.maker,
            accounts.system_program,
            accounts.token_program,
        )?;

        Ok(Self { accounts })
    }
}

// Transfer the tokens from the taker_ata_b to the maker_ata_b.
// Transfer the tokens from the vault to the taker_ata_a.
// Close the now empty vault and withdraw the rent from the account.

impl<'a> Take<'a> {
    pub const DISCRIMINATOR: &'a u8 = &1;
    pub fn process(&mut self) -> ProgramResult {
        let data = self.accounts.escrow.try_borrow_data()?;
        let escrow = Escrow::load(&data)?;

        //check if escrow is valid
        let escrow_key = create_program_address(
            &[
                b"escrow",
                self.accounts.maker.key(),
                &escrow.seed.to_le_bytes(),
                &escrow.bump,
            ],
            &crate::ID,
        )?;

        if &escrow_key != self.accounts.escrow.key() {
            return Err(ProgramError::InvalidAccountOwner);
        }

        let seed_binding = escrow.seed.to_le_bytes();
        let bump_binding = escrow.bump;
        let escrow_seeds = [
            Seed::from(b"escrow"),
            Seed::from(self.accounts.maker.key().as_ref()),
            Seed::from(&seed_binding),
            Seed::from(&bump_binding),
        ];

        let signer = Signer::from(&escrow_seeds);

        let amount = TokenAccount::amount(self.accounts.vault);
    }
}
