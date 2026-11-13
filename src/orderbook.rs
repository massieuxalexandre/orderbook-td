use crate::interfaces::{OrderBook, Price, Quantity, Side, Update};

const MAX_PRICE: usize = 1000000;

pub struct OrderBookImpl {
    bids_qty: Vec<Quantity>,
    asks_qty: Vec<Quantity>,
    
    best_bid: Price,
    best_ask: Price,
}

impl OrderBook for OrderBookImpl {
    fn new() -> Self {
        Self {
            bids_qty: vec![0; MAX_PRICE],
            asks_qty: vec![0; MAX_PRICE],
            best_bid: 0,
            best_ask: MAX_PRICE as Price, 
        }
    }

    #[inline]
    fn apply_update(&mut self, update: Update) {
        match update {
            Update::Set { price, quantity, side } => {
                let p = price as usize;
                if quantity == 0 {
                    self.remove_level(price, side);
                } else {
                    match side {
                        Side::Bid => {
                            self.bids_qty[p] = quantity;
                            if price > self.best_bid {
                                self.best_bid = price;
                            }
                        }
                        Side::Ask => {
                            self.asks_qty[p] = quantity;
                            if price < self.best_ask {
                                self.best_ask = price;
                            }
                        }
                    }
                }
            }
            Update::Remove { price, side } => {
                self.remove_level(price, side);
            }
        }
    }

    #[inline]
    fn get_best_bid(&self) -> Option<Price> {
        if self.best_bid == 0 { None } else { Some(self.best_bid) }
    }

    #[inline]
    fn get_best_ask(&self) -> Option<Price> {
        if self.best_ask == MAX_PRICE as Price { None } else { Some(self.best_ask) }
    }

    #[inline]
    fn get_quantity_at(&self, price: Price, side: Side) -> Option<Quantity> {
        let qty = match side {
            Side::Bid => self.bids_qty[price as usize],
            Side::Ask => self.asks_qty[price as usize],
        };
        if qty > 0 { Some(qty) } else { None }
    }
    
    // ... (autres implémentations basiques)
    fn get_spread(&self) -> Option<Price> { 
        Some(self.get_best_ask()? - self.get_best_bid()?)
    }

    fn get_top_levels(&self, side: Side, n: usize) -> Vec<(Price, Quantity)> { vec![] }
    fn get_total_quantity(&self, side: Side) -> Quantity { 0 }
}

impl OrderBookImpl {
    #[inline]
    fn remove_level(&mut self, price: Price, side: Side) {
        let p = price as usize;
        match side {
            Side::Bid => {
                self.bids_qty[p] = 0;
                if price == self.best_bid {
                    while self.best_bid > 0 && self.bids_qty[self.best_bid as usize] == 0 {
                        self.best_bid -= 1;
                    }
                }
            }
            Side::Ask => {
                self.asks_qty[p] = 0;
                if price == self.best_ask {
                    while self.best_ask < MAX_PRICE as Price && self.asks_qty[self.best_ask as usize] == 0 {
                        self.best_ask += 1;
                    }
                }
            }
        }
    }
}