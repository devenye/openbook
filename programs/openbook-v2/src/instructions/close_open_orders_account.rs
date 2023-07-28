use crate::accounts_ix::*;
use crate::error::OpenBookError;
use anchor_lang::prelude::*;

pub fn close_open_orders_account(ctx: Context<CloseOpenOrdersAccount>) -> Result<()> {
    let open_orders_account = ctx.accounts.open_orders_account.load()?;

    require!(
        open_orders_account.has_no_orders(),
        OpenBookError::OpenOrdersAccountContainsOrders
    );

    let mut indexer = ctx.accounts.open_orders_indexer.load_mut()?;
    indexer.closed_counter += 1;

    Ok(())
}
