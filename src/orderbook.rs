// ============================================================================
// REFERENCE IMPLEMENTATION (Naive, for correctness comparison)
// ============================================================================

use crate::interfaces::{OrderBook, Price, Quantity, Side, Update};
use std::collections::BTreeMap;

pub struct OrderBookImpl {
    bids: BTreeMap<Price, Quantity>,
    asks: BTreeMap<Price, Quantity>,
}

impl OrderBook for OrderBookImpl {
    fn new() -> Self {
        Self {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
        }
    }

    fn apply_update(&mut self, update: Update) {
        match update {
            Update::Set { price, quantity, side } => {
                if quantity == 0 {
                    match side {
                        Side::Bid => self.bids.remove(&price),
                        Side::Ask => self.asks.remove(&price)
                    };
                } else {
                    match side {
                        Side::Bid => self.bids.insert(price, quantity),
                        Side::Ask => self.asks.insert(price, quantity)
                    };
                }
            },
            Update::Remove { price, side } => {
                match side {
                    Side::Bid => self.bids.remove(&price),
                    Side::Ask => self.asks.remove(&price)
                };
            }
        };
    }

    fn get_spread(&self) -> Option<Price> {
        Some(self.get_best_ask()? - self.get_best_bid()?)
    }

    fn get_best_bid(&self) -> Option<Price> {
        self.bids.last_key_value().map(|(&price, _)| price)
    }

    fn get_best_ask(&self) -> Option<Price> {
        self.asks.first_key_value().map(|(&price, _)| price)
    }

    fn get_quantity_at(&self, price: Price, side: Side) -> Option<Quantity> {
        match side {
            Side::Bid => self.bids.get(&price).copied(),
            Side::Ask => self.asks.get(&price).copied()
        }
    }

    fn get_top_levels(&self, side: Side, n: usize) -> Vec<(Price, Quantity)> {
        match side {
            Side::Bid => self.bids.iter().rev().take(n).map(|(&price, &quantity)| (price, quantity)).collect(),
            Side::Ask => self.asks.iter().take(n).map(|(&price, &quantity)| (price, quantity)).collect(),
        }
    }

    fn get_total_quantity(&self, side: Side) -> Quantity {
        match side {
            Side::Bid => self.bids.values().sum(),
            Side::Ask => self.asks.values().sum(),
        }
    }
}
