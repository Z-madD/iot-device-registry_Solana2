use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

pub const DEVICE_SPACE: usize = 276;

#[program]
pub mod iot_device_registry {
    use super::*;

    pub fn create_device(
        ctx: Context<CreateDevice>,
        device_id: String,
        device_name: String,
        device_type: String,
        location: String,
        status: String,
    ) -> Result<()> {
        require!(device_id.len() <= 32, ErrorCode::DeviceIdTooLong);
        require!(device_name.len() <= 64, ErrorCode::DeviceNameTooLong);
        require!(device_type.len() <= 32, ErrorCode::DeviceTypeTooLong);
        require!(location.len() <= 64, ErrorCode::LocationTooLong);
        require!(status.len() <= 16, ErrorCode::StatusTooLong);

        let device = &mut ctx.accounts.device;
        device.owner = ctx.accounts.owner.key();
        device.device_id = device_id;
        device.device_name = device_name;
        device.device_type = device_type;
        device.location = location;
        device.status = status;
        device.created_at = Clock::get()?.unix_timestamp;

        msg!("Dispositivo creado: {} para owner {:?}", device.device_id, device.owner);
        Ok(())
    }

    pub fn read_device(_ctx: Context<ReadDevice>) -> Result<()> {
        msg!("Lectura exitosa. Usa el cliente para obtener los datos.");
        Ok(())
    }

    pub fn update_device(
        ctx: Context<UpdateDevice>,
        device_name: String,
        device_type: String,
        location: String,
        status: String,
    ) -> Result<()> {
        require!(device_name.len() <= 64, ErrorCode::DeviceNameTooLong);
        require!(device_type.len() <= 32, ErrorCode::DeviceTypeTooLong);
        require!(location.len() <= 64, ErrorCode::LocationTooLong);
        require!(status.len() <= 16, ErrorCode::StatusTooLong);

        let device = &mut ctx.accounts.device;
        device.device_name = device_name;
        device.device_type = device_type;
        device.location = location;
        device.status = status;
        msg!("Dispositivo {} actualizado", device.device_id);
        Ok(())
    }

    pub fn delete_device(_ctx: Context<DeleteDevice>) -> Result<()> {
        msg!("Dispositivo eliminado");
        Ok(())
    }
}

#[account]
pub struct Device {
    pub owner: Pubkey,
    pub device_id: String,
    pub device_name: String,
    pub device_type: String,
    pub location: String,
    pub status: String,
    pub created_at: i64,
}

#[derive(Accounts)]
#[instruction(device_id: String)]
pub struct CreateDevice<'info> {
    #[account(
        init,
        payer = owner,
        space = DEVICE_SPACE,
        seeds = [b"device", owner.key().as_ref(), device_id.as_bytes()],
        bump
    )]
    pub device: Account<'info, Device>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(device_id: String)]
pub struct ReadDevice<'info> {
    #[account(
        seeds = [b"device", owner.key().as_ref(), device_id.as_bytes()],
        bump
    )]
    pub device: Account<'info, Device>,
    pub owner: SystemAccount<'info>,
}

#[derive(Accounts)]
#[instruction(device_id: String)]
pub struct UpdateDevice<'info> {
    #[account(
        mut,
        seeds = [b"device", owner.key().as_ref(), device_id.as_bytes()],
        bump,
        constraint = device.owner == owner.key()
    )]
    pub device: Account<'info, Device>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(device_id: String)]
pub struct DeleteDevice<'info> {
    #[account(
        mut,
        seeds = [b"device", owner.key().as_ref(), device_id.as_bytes()],
        bump,
        constraint = device.owner == owner.key(),
        close = owner,
    )]
    pub device: Account<'info, Device>,
    pub owner: Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("El device_id excede los 32 caracteres permitidos")]
    DeviceIdTooLong,
    #[msg("El device_name excede los 64 caracteres permitidos")]
    DeviceNameTooLong,
    #[msg("El device_type excede los 32 caracteres permitidos")]
    DeviceTypeTooLong,
    #[msg("La location excede los 64 caracteres permitidos")]
    LocationTooLong,
    #[msg("El status excede los 16 caracteres permitidos")]
    StatusTooLong,
}
