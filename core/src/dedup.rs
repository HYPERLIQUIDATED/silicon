use std::{
    collections::HashSet,
    time::{Duration, Instant},
};

use crate::update::UpdateKey;

#[derive(Debug)]
pub(crate) struct UpdateDeduper {
    capacity: usize,
    hot: HashSet<UpdateKey>,
    cold: HashSet<UpdateKey>,
    rotate_interval: Duration,
    last_rotate: Instant,
}

impl UpdateDeduper {
    pub(crate) fn new(capacity: usize, rotate_interval: Duration) -> Self {
        Self {
            capacity,
            hot: HashSet::with_capacity(capacity),
            cold: HashSet::with_capacity(capacity),
            rotate_interval,
            last_rotate: Instant::now(),
        }
    }

    pub(crate) fn insert(&mut self, update_key: UpdateKey) -> bool {
        if self.last_rotate.elapsed() >= self.rotate_interval {
            self.rotate();
        }

        if self.hot.len() >= self.capacity {
            self.rotate();
        }

        if self.hot.contains(&update_key) || self.cold.contains(&update_key) {
            return false;
        }

        self.hot.insert(update_key);

        true
    }

    fn rotate(&mut self) {
        std::mem::swap(&mut self.hot, &mut self.cold);
        self.hot.clear();
        self.last_rotate = Instant::now();
    }
}
