// ============================================================================
// VEC IMPLEMENTATION
// ============================================================================

use crate::interfaces::{OrderBook, Price, Quantity, Side, Update};

pub struct OrderBookImpl {
    // Les Bids sont triés de manière décroissante (Highest first)
    bids: Vec<(Price, Quantity)>,
    // Les Asks sont triés de manière croissante (Lowest first)
    asks: Vec<(Price, Quantity)>,
}

impl OrderBook for OrderBookImpl {
    fn new() -> Self {
        Self {
            // Pré-allocation pour éviter les réallocations coûteuses
            bids: Vec::with_capacity(1024),
            asks: Vec::with_capacity(1024),
        }
    }

    fn apply_update(&mut self, update: Update) {
        match update {
            Update::Set { price, quantity, side } => {
                if quantity == 0 {
                    self.remove_level(price, side);
                } else {
                    match side {
                        Side::Bid => {
                            match self.bids.binary_search_by(|(p, _)| p.cmp(&price).reverse()) {
                                Ok(idx) => self.bids[idx].1 = quantity, // Met à jour la quantité
                                Err(idx) => self.bids.insert(idx, (price, quantity)), // Insère au bon endroit
                            }
                        }
                        Side::Ask => {
                            match self.asks.binary_search_by(|(p, _)| p.cmp(&price)) {
                                Ok(idx) => self.asks[idx].1 = quantity,
                                Err(idx) => self.asks.insert(idx, (price, quantity)),
                            }
                        }
                    }
                }
            }
            Update::Remove { price, side } => {
                self.remove_level(price, side);
            }
        };
    }

    #[inline]
    fn get_spread(&self) -> Option<Price> {
        Some(self.get_best_ask()? - self.get_best_bid()?)
    }

    #[inline]
    fn get_best_bid(&self) -> Option<Price> {
        self.bids.first().map(|(p, _)| *p)
    }

    #[inline]
    fn get_best_ask(&self) -> Option<Price> {
        self.asks.first().map(|(p, _)| *p)
    }

    fn get_quantity_at(&self, price: Price, side: Side) -> Option<Quantity> {
        match side {
            Side::Bid => self.bids
                .binary_search_by(|(p, _)| p.cmp(&price).reverse())
                .ok()
                .map(|idx| self.bids[idx].1),
            Side::Ask => self.asks
                .binary_search_by(|(p, _)| p.cmp(&price))
                .ok()
                .map(|idx| self.asks[idx].1),
        }
    }

    fn get_top_levels(&self, side: Side, n: usize) -> Vec<(Price, Quantity)> {
        match side {
            Side::Bid => self.bids.iter().take(n).copied().collect(),
            Side::Ask => self.asks.iter().take(n).copied().collect(),
        }
    }

    fn get_total_quantity(&self, side: Side) -> Quantity {
        match side {
            Side::Bid => self.bids.iter().map(|(_, q)| q).sum(),
            Side::Ask => self.asks.iter().map(|(_, q)| q).sum(),
        }
    }
}

// Méthode utilitaire pour garder apply_update propre
impl OrderBookImpl {
    #[inline]
    fn remove_level(&mut self, price: Price, side: Side) {
        match side {
            Side::Bid => {
                if let Ok(idx) = self.bids.binary_search_by(|(p, _)| p.cmp(&price).reverse()) {
                    self.bids.remove(idx);
                }
            }
            Side::Ask => {
                if let Ok(idx) = self.asks.binary_search_by(|(p, _)| p.cmp(&price)) {
                    self.asks.remove(idx);
                }
            }
        }
    }
}