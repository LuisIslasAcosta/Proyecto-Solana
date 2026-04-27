use anchor_lang::prelude::*;

declare_id!("5ZjpnkFbwAaMJonsW7MmcwGzsBwzKRuQ2NJ7SYTAL5hM");

#[program]
pub mod motocicletas {
    use super::*;

    pub fn crear_agencia(ctx: Context<NuevaAgencia>, nombre: String) -> Result<()> {
        let owner = ctx.accounts.owner.key();

        ctx.accounts.agencia.set_inner(Agencia {
            owner,
            nombre,
            motos: Vec::new(),
        });

        Ok(())
    }

    pub fn agregar_moto(
        ctx: Context<NuevaMoto>,
        modelo: String,
        marca: String,
        precio: u32,
        stock: u16,
    ) -> Result<()> {
        require!(
            ctx.accounts.agencia.owner == ctx.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let moto = Moto {
            modelo,
            marca,
            precio,
            stock,
            disponible: true,
        };

        ctx.accounts.agencia.motos.push(moto);

        Ok(())
    }

    pub fn eliminar_moto(ctx: Context<NuevaMoto>, modelo: String) -> Result<()> {
        require!(
            ctx.accounts.agencia.owner == ctx.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let motos = &mut ctx.accounts.agencia.motos;

        for i in 0..motos.len() {
            if motos[i].modelo == modelo {
                motos.remove(i);
                return Ok(());
            }
        }

        Err(Errores::MotoNoExiste.into())
    }

    pub fn actualizar_stock(
        ctx: Context<NuevaMoto>,
        modelo: String,
        nuevo_stock: u16,
    ) -> Result<()> {
        require!(
            ctx.accounts.agencia.owner == ctx.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let motos = &mut ctx.accounts.agencia.motos;

        for m in motos.iter_mut() {
            if m.modelo == modelo {
                m.stock = nuevo_stock;
                return Ok(());
            }
        }

        Err(Errores::MotoNoExiste.into())
    }

    pub fn alternar_estado(ctx: Context<NuevaMoto>, modelo: String) -> Result<()> {
        require!(
            ctx.accounts.agencia.owner == ctx.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let motos = &mut ctx.accounts.agencia.motos;

        for m in motos.iter_mut() {
            if m.modelo == modelo {
                m.disponible = !m.disponible;
                return Ok(());
            }
        }

        Err(Errores::MotoNoExiste.into())
    }

    pub fn ver_motos(ctx: Context<NuevaMoto>) -> Result<()> {
        require!(
            ctx.accounts.agencia.owner == ctx.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        msg!("Motos: {:#?}", ctx.accounts.agencia.motos);
        Ok(())
    }
}

#[error_code]
pub enum Errores {
    #[msg("No eres el propietario")]
    NoEresElOwner,

    #[msg("La motocicleta no existe")]
    MotoNoExiste,
}

#[account]
#[derive(InitSpace)]
pub struct Agencia {
    owner: Pubkey,

    #[max_len(60)]
    nombre: String,

    #[max_len(20)]
    motos: Vec<Moto>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Moto {
    #[max_len(60)]
    modelo: String,

    #[max_len(60)]
    marca: String,

    precio: u32,

    stock: u16,

    disponible: bool,
}

#[derive(Accounts)]
pub struct NuevaAgencia<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = Agencia::INIT_SPACE + 8,
        seeds = [b"agencia", owner.key().as_ref()],
        bump
    )]
    pub agencia: Account<'info, Agencia>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct NuevaMoto<'info> {
    pub owner: Signer<'info>,

    #[account(mut)]
    pub agencia: Account<'info, Agencia>,
}
