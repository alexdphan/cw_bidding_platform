use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::Item;
use serde::{Deserialize, Serialize};
use cosmwasm_std::Decimal;

// This is the state of the bidding contract
// It is serialized and stored under the key "State" in the storage
// It is deserialized when read from the storage
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct State {
    pub owner: Addr,
    pub bid: Option<Coin>,
    pub bidder: Option<Addr>,
    pub bid_end: Option<u64>,
    pub reveal_end: Option<u64>,
    pub reveal: Option<Reveal>,
    pub reveal_count: u64,
    pub reveal_total: Decimal,
    pub reveal_winner: Option<Addr>,
    pub reveal_winner_bid: Option<Coin>,
    pub reveal_winner_reveal: Option<Reveal>,
    pub reveal_winner_count: u64,
    pub reveal_winner_total: Decimal,
    pub reveal_winner_reveal_count: u64,
    pub reveal_winner_reveal_total: Decimal,
}

// This is the state of the reveal
// It is serialized and stored under the key "Reveal" in the storage
// It is deserialized when read from the storage
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct RevealWinner {
    pub bid: Coin, // bid amount
    pub salt: String, // string used to generate the hash
    pub reveal: Reveal, // Reveal struct, this comes from the reveal message
}


pub const STATE: Item<State> = Item::new("state");
pub const REVEAL: Item<Reveal> = Item::new("reveal");