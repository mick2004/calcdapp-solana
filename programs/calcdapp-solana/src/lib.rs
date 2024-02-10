use anchor_lang::prelude::*;

declare_id!("FPgUrvBWHeijJUYKkTaVoW5MSCtMeZ2g65Znnd4iq9KN");

#[program]
pub mod calcdapp_solana {
    use anchor_lang::solana_program::entrypoint::ProgramResult;

    use super::*;
    pub fn create(ctx : Context<Create> , initMessage : String) -> ProgramResult{
        let calculator = &mut ctx.accounts.calculator;
        calculator.greeting = initMessage;
        Ok(())

    }
}

#derive(Accounts)

pub struct create<'info>{
    #[account(init,payer=user,space=264)]
    pub calculator : Account<'info,Calculator>,
    #[account(mut)]
    pub user : Signer<'info>,
    pub system_program : Program<'info,System>

}

#[account]
pub struct Calculator{
    pub greeting : String ,
    pub result : i64,
    pub remainder :i64
}

