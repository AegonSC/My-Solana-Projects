use anchor_lang::prelude::*;

// Instruccion que va a crear un contado de Solana
// contador: numero - iniciando en 0
declare_id!("6N3L2oAurRvcormHiWetSEyQb5S6jDyWbBC4taNvh4G2");

#[program]
pub mod contador {

    use super::*;

    pub fn crear_contador(ctx: Context<CrearContador>) -> Result<()> {
        ctx.accounts.contador.valor = 0;

        Ok(())
    }

    pub fn incrementar_contador(ctx: Context<ModificarContador>) -> Result<()> {
        let contador = &mut ctx.accounts.contador;
        contador.valor += 1;
        msg!("El contador ha sido incrementado a {} ", contador.valor);
        Ok(())
    }

    pub fn decrementar_contador(ctx: Context<ModificarContador>) -> Result<()> {
        let contador = &mut ctx.accounts.contador;
        if contador.valor > 0 {
            contador.valor -= 1;
            msg!("El contador ha sido decrementado a {}", contador.valor);
        } else {
            msg!("El contador no se pudo decrementar");
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CrearContador<'info> {
    #[account(
        init,
        payer = user,
        space = Contador::INIT_SPACE,
        seeds = [b"contador", user.key().as_ref()],
        bump
    )]
    pub contador: Account<'info, Contador>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModificarContador<'info> {
    #[account(mut)]
    pub contador: Account<'info, Contador>,
    pub user: Signer<'info>,
}

#[account]
#[derive(InitSpace)]
pub struct Contador {
    pub valor: u64,
}
