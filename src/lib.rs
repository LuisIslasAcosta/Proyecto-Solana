use anchor_lang::prelude::*;

declare_id!("Z6eb7skqS5xQtBetsEspnJNfaVjkiVxfbE8awGegfDh");

#[program]
pub mod refaccionaria {
    use super::*;

    // Crear refaccionaria
    pub fn crear_refaccionaria(ctx: Context<NuevaRefaccionaria>, nombre: String) -> Result<()> {
        let owner = ctx.accounts.owner.key();

        ctx.accounts.refaccionaria.set_inner(Refaccionaria {
            owner,
            nombre,
            refacciones: Vec::new(),
        });

        Ok(())
    }

    // Agregar refacción
    pub fn agregar_refaccion(
        ctx: Context<NuevaRefaccion>,
        nombre: String,
        precio: u32,
        stock: u16,
    ) -> Result<()> {
        require!(
            ctx.accounts.refaccionaria.owner == ctx.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let refaccion = Refaccion {
            nombre,
            precio,
            stock,
            activo: true,
        };

        ctx.accounts.refaccionaria.refacciones.push(refaccion);

        Ok(())
    }

    // Eliminar refacción
    pub fn eliminar_refaccion(ctx: Context<NuevaRefaccion>, nombre: String) -> Result<()> {
        require!(
            ctx.accounts.refaccionaria.owner == ctx.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let refacciones = &mut ctx.accounts.refaccionaria.refacciones;

        for i in 0..refacciones.len() {
            if refacciones[i].nombre == nombre {
                refacciones.remove(i);
                return Ok(());
            }
        }

        Err(Errores::RefaccionNoExiste.into())
    }

    // Actualizar stock
    pub fn actualizar_stock(
        ctx: Context<NuevaRefaccion>,
        nombre: String,
        nuevo_stock: u16,
    ) -> Result<()> {
        require!(
            ctx.accounts.refaccionaria.owner == ctx.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let refacciones = &mut ctx.accounts.refaccionaria.refacciones;

        for r in refacciones.iter_mut() {
            if r.nombre == nombre {
                r.stock = nuevo_stock;
                return Ok(());
            }
        }

        Err(Errores::RefaccionNoExiste.into())
    }

    // Activar / desactivar refacción
    pub fn alternar_estado(ctx: Context<NuevaRefaccion>, nombre: String) -> Result<()> {
        require!(
            ctx.accounts.refaccionaria.owner == ctx.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let refacciones = &mut ctx.accounts.refaccionaria.refacciones;

        for r in refacciones.iter_mut() {
            if r.nombre == nombre {
                r.activo = !r.activo;
                return Ok(());
            }
        }

        Err(Errores::RefaccionNoExiste.into())
    }

    // Ver refacciones (log)
    pub fn ver_refacciones(ctx: Context<NuevaRefaccion>) -> Result<()> {
        require!(
            ctx.accounts.refaccionaria.owner == ctx.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        msg!("Refacciones: {:#?}", ctx.accounts.refaccionaria.refacciones);
        Ok(())
    }
}

// ERRORES
#[error_code]
pub enum Errores {
    #[msg("No eres el propietario")]
    NoEresElOwner,

    #[msg("La refaccion no existe")]
    RefaccionNoExiste,
}

// CUENTA PRINCIPAL
#[account]
#[derive(InitSpace)]
pub struct Refaccionaria {
    owner: Pubkey,

    #[max_len(60)]
    nombre: String,

    #[max_len(20)]
    refacciones: Vec<Refaccion>,
}

// STRUCT INTERNO
#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Refaccion {
    #[max_len(60)]
    nombre: String,

    precio: u32,

    stock: u16,

    activo: bool,
}

// CONTEXTO CREAR
#[derive(Accounts)]
pub struct NuevaRefaccionaria<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = Refaccionaria::INIT_SPACE + 8,
        seeds = [b"refaccionaria", owner.key().as_ref()],
        bump
    )]
    pub refaccionaria: Account<'info, Refaccionaria>,

    pub system_program: Program<'info, System>,
}

// CONTEXTO CRUD
#[derive(Accounts)]
pub struct NuevaRefaccion<'info> {
    pub owner: Signer<'info>,

    #[account(mut)]
    pub refaccionaria: Account<'info, Refaccionaria>,
}
