// Similar to an import statemet. It imports the tools Anchor provides to write Solana programs:
use anchor_lang::prelude::*;

// A machro that declares what the programs account id is & should be used at the root of 
// all Anchor-based programs:
declare_id!("6cFQPTmKF8vzYu1v8Gktj3JAVz6C5EnhBDztEzXPqGEM");

// An attribute machro, that attaches code to the module & lets Anchor know these are the 
// programs instruction handlres. This allows a calling of the solana program from your dApp
// via a fetch request:
#[program]
pub mod sceneportal {
    use super::*;
    pub fn start_stuff_off(_ctx: Context<StartStuffOff>) -> Result <()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct StartStuffOff {}
