use cosmwasm_std::{Addr, Coin, DepsMut, MessageInfo, Response, StdResult};
use cw_storage_plus::Item;
use serde::{Deserialize, Serialize};

use crate::error::ContractError;
use crate::state::{State, REVEAL, STATE};

pub fn instantiate(deps: DepsMut, _info: MessageInfo, msg: InstantiateMsg) -> StdResult<Response> {
    let state = State {
        owner: deps.api.addr_validate(&msg.owner)?,
        bid: None,
        bidder: None,
        bid_end: None,
        reveal_end: None,
        reveal: None,
        reveal_count: 0,
        reveal_total: Decimal::zero(),
        reveal_winner: None,
        reveal_winner_bid: None,
        reveal_winner_reveal: None,
        reveal_winner_count: 0,
        reveal_winner_total: Decimal::zero(),
        reveal_winner_reveal_count: 0,
        reveal_winner_reveal_total: Decimal::zero(),
    };
    STATE.save(deps.storage, &state)?;
    Ok(Response::default())
}

// state for 
