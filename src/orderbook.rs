// ============================================================================
// REFERENCE IMPLEMENTATION (Naive, for correctness comparison)
// ============================================================================

use crate::interfaces::{OrderBook, Price, Quantity, Side, Update};
// use std::collections::BTreeMap;

pub struct OrderBookImpl {
    // bids: BTreeMap<Price, Quantity>,
    bids: Vec<(Price, Quantity)>,
    // asks: BTreeMap<Price, Quantity>,
    asks: Vec<(Price, Quantity)>,
}

impl OrderBook for OrderBookImpl {
    fn new() -> Self {
        Self {
            bids: Vec::new(),
            asks: Vec::new(),
        }
    }

    fn apply_update(&mut self, update: Update) {
        match update {
            Update::Set { price, quantity, side } => {
                if quantity == 0 {
                    match side {
                        // Side::Bid => self.bids.remove(&price),
                        // Side::Ask => self.asks.remove(&price)
                        Side::Bid => self.bids.retain(|&(p, _)| p != price),
                        Side::Ask => self.asks.retain(|&(p, _)| p != price),
                    };
                } else {
                    match side {
                        // Side::Bid => self.bids.insert(price, quantity),
                        // Side::Ask => self.asks.insert(price, quantity)
                        Side::Bid => {
                            if let Some(pos) = self.bids.iter().position(|&(p, _)| p == price) {
                                self.bids[pos] = (price, quantity);
                            } else {
                                self.bids.push((price, quantity));
                            }
                        }
                        Side::Ask => {
                            if let Some(pos) = self.asks.iter().position(|&(p, _)| p == price) {
                                self.asks[pos] = (price, quantity);
                            } else {
                                self.asks.push((price, quantity));
                            }
                        }
                    };
                }
            },
            Update::Remove { price, side } => {
                match side {
                    // Side::Bid => self.bids.remove(&price),
                    // Side::Ask => self.asks.remove(&price)
                    Side::Bid => self.bids.retain(|&(p, _)| p != price),
                    Side::Ask => self.asks.retain(|&(p, _)| p != price),
                };
            }
        };
    }

    fn get_spread(&self) -> Option<Price> {
        Some(self.get_best_ask()? - self.get_best_bid()?)
    }

    fn get_best_bid(&self) -> Option<Price> {
        // self.bids.last_key_value().map(|(&price, _)| price)
        self.bids.iter().max_by_key(|&&(price, _)| price).map(|&(price, _)| price)
    }

    fn get_best_ask(&self) -> Option<Price> {
        // self.asks.first_key_value().map(|(&price, _)| price)
        self.asks.iter().min_by_key(|&&(price, _)| price).map(|&(price, _)| price)
    }

    fn get_quantity_at(&self, price: Price, side: Side) -> Option<Quantity> {
        match side {
            // Side::Bid => self.bids.get(&price).copied(),
            // Side::Ask => self.asks.get(&price).copied()
            Side::Bid => self.bids.iter().find(|&&(p, _)| p == price).map(|&(price, quantity)| quantity),
            Side::Ask => self.asks.iter().find(|&&(p, _)| p == price).map(|&(price, quantity)| quantity),
        }
    }

    fn get_top_levels(&self, side: Side, n: usize) -> Vec<(Price, Quantity)> {
        match side {
            // Side::Bid => self.bids.iter().rev().take(n).map(|(&price, &quantity)| (price, quantity)).collect(),
            // Side::Ask => self.asks.iter().take(n).map(|(&price, &quantity)| (price, quantity)).collect(),
            Side::Bid => {
                let mut top_bids = self.bids.clone();
                top_bids.sort_by_key(|&(price, _)| std::cmp::Reverse(price));
                top_bids.into_iter().take(n).collect()
            },
            Side::Ask => {
                let mut top_asks = self.asks.clone();
                top_asks.sort_by_key(|&(price, _)| price);
                top_asks.into_iter().take(n).collect()
            },
        }
    }

    fn get_total_quantity(&self, side: Side) -> Quantity {
        match side {
            // Side::Bid => self.bids.values().sum(),
            // Side::Ask => self.asks.values().sum(),
            Side::Bid => self.bids.iter().map(|&(_, quantity)| quantity).sum(),
            Side::Ask => self.asks.iter().map(|&(_, quantity)| quantity).sum(),
        }
    }
}
