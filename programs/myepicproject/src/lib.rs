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
    pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> Result <()> {

        // &mut is a mutable reference to base_account. When you do this, it allows you to 
        // make changes to base_account. Otherwise it'll be working with a local copy of base_account:
        let base_account = &mut ctx.accounts.base_account;
        base_account.total_gifs = 0;

        Ok(())
    }

    pub fn add_gif(ctx: Context<AddGif>, gif_link: String) -> Result <()> {
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user; 

        let item = ItemStruct {
            gif_link: gif_link.to_string(), 
            user_address: *user.to_account_info().key, 
        };

        base_account.gif_list.push(item); 
        base_account.total_gifs += 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    
    // This tells Solana how to initialize BaseAccount:
    #[account(init, payer = user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,

    // Signer: Data passed into the program that proves to the program the user actually owns their wallet account:
    // Program: A program that creators of Solana deployed that other programs can talk to 
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program <'info, System>,
}

#[derive(Accounts)]
pub struct AddGif<'info> {
  #[account(mut)]
  pub base_account: Account<'info, BaseAccount>,
  #[account(mut)]
  pub user: Signer<'info>,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub gif_link: String, 
    pub user_address: Pubkey, 
}

#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
    pub gif_list: Vec<ItemStruct>,
}
