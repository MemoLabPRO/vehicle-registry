use anchor_lang::prelude::*;

declare_id!("3CgTkABSuEUC6C6U5zpuzB2SRhggThSHsVLe3NvxKYY3");

#[program]
pub mod vehicle_registry {
    use super::*;

    pub fn register_vehicle(
        ctx: Context<RegisterVehicle>,
        vin: [u8; 17],
        make: String,
        model: String,
        year: u16,
    ) -> Result<()> {
        require!(make.len() <= 32, RegistryError::StringTooLong);
        require!(model.len() <= 32, RegistryError::StringTooLong);

        let record = &mut ctx.accounts.vehicle_record;
        record.vin = vin;
        record.owner = ctx.accounts.owner.key();
        record.make = make;
        record.model = model;
        record.year = year;
        record.status = VehicleStatus::Active;
        record.registered_at = Clock::get()?.unix_timestamp;
        record.bump = ctx.bumps.vehicle_record;

        msg!("Vehicle registered. VIN: {:?}", vin);
        Ok(())
    }

    pub fn transfer_title(
        ctx: Context<TransferTitle>,
        _vin: [u8; 17],
    ) -> Result<()> {
        let record = &mut ctx.accounts.vehicle_record;

        require!(
            record.status == VehicleStatus::ForSale,
            RegistryError::NotForSale
        );

        record.owner = ctx.accounts.new_owner.key();
        record.status = VehicleStatus::Active;

        msg!("Title transferred to: {:?}", record.owner);
        Ok(())
    }

    pub fn update_status(
        ctx: Context<UpdateStatus>,
        _vin: [u8; 17],
        new_status: VehicleStatus,
    ) -> Result<()> {
        let record = &mut ctx.accounts.vehicle_record;
        record.status = new_status;

        msg!("Status updated: {:?}", record.status);
        Ok(())
    }
}

// ── Accounts ──────────────────────────────────────────────

#[derive(Accounts)]
#[instruction(vin: [u8; 17])]
pub struct RegisterVehicle<'info> {
    #[account(
        init,
        payer = owner,
        space = VehicleRecord::LEN,
        seeds = [b"vehicle", vin.as_ref()],
        bump
    )]
    pub vehicle_record: Account<'info, VehicleRecord>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(vin: [u8; 17])]
pub struct TransferTitle<'info> {
    #[account(
        mut,
        seeds = [b"vehicle", vin.as_ref()],
        bump = vehicle_record.bump,
        has_one = owner
    )]
    pub vehicle_record: Account<'info, VehicleRecord>,

    pub owner: Signer<'info>,

    /// CHECK: new owner is just a destination pubkey, no signing needed
    pub new_owner: AccountInfo<'info>,
}

#[derive(Accounts)]
#[instruction(vin: [u8; 17])]
pub struct UpdateStatus<'info> {
    #[account(
        mut,
        seeds = [b"vehicle", vin.as_ref()],
        bump = vehicle_record.bump,
        has_one = owner
    )]
    pub vehicle_record: Account<'info, VehicleRecord>,

    pub owner: Signer<'info>,
}

// ── State ─────────────────────────────────────────────────

#[account]
pub struct VehicleRecord {
    pub vin: [u8; 17],
    pub owner: Pubkey,
    pub make: String,
    pub model: String,
    pub year: u16,
    pub status: VehicleStatus,
    pub registered_at: i64,
    pub bump: u8,
}

impl VehicleRecord {
    pub const LEN: usize = 8     // discriminator
        + 17                     // vin
        + 32                     // owner pubkey
        + 4 + 32                 // make (string prefix + max bytes)
        + 4 + 32                 // model
        + 2                      // year
        + 1                      // status enum
        + 8                      // registered_at
        + 1;                     // bump
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Debug)]
pub enum VehicleStatus {
    Active,
    ForSale,
    Stolen,
}

// ── Errors ────────────────────────────────────────────────

#[error_code]
pub enum RegistryError {
    #[msg("Vehicle is not listed for sale")]
    NotForSale,
    #[msg("String field exceeds maximum length")]
    StringTooLong,
}