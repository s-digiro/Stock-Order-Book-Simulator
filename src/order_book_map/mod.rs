mod order_book;

use crate::Transaction;
use std::collections::HashMap;
use order_book::OrderBook;

#[derive(PartialEq)]
#[derive(Debug)]
pub struct OrderBookMap {
    enable_cross: bool,
    scenario: usize,
    books: HashMap<String, OrderBook>
}

impl OrderBookMap {
    pub fn new(enable_cross: bool) -> OrderBookMap {
        println!("# scenario 1");
        OrderBookMap {
            enable_cross,
            scenario: 1,
            books: HashMap::new()
        }
    }

    pub fn do_transaction(&mut self, t: Transaction) {
        match t {
            Transaction::Flush => {
                self.books.clear();
                println!("");
                self.scenario += 1;
                println!("# scenario {}", self.scenario);
            },
            Transaction::New(n) => {
                if !self.books.contains_key(&n.symbol) {
                    self.books.insert(
                        n.symbol.clone(),
                        OrderBook::new(&n.symbol, self.enable_cross)
                    );
                }

                self.books.get_mut(&n.symbol).unwrap().submit_order(n);
            },
            Transaction::Cancel(c) => {
                for book in self.books.values_mut() {
                    book.remove(c.user, c.user_order_id);
                }
            }
        }
    }
}
