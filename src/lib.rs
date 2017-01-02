// Copyright (c) 2017 Jeremy Rubin
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate rand;

pub use std::time::{Duration, Instant};
pub struct Timeout {
    created_at: Instant,
    expires_after: Duration,
}
impl Timeout {
    pub fn new(t: Duration) -> Self {
        Timeout {
            created_at: Instant::now(),
            expires_after: t,
        }
    }
    pub fn new_rand(min: Duration, max: Duration) -> Self {
        use rand::distributions::{IndependentSample, Range};
        let mut rng = rand::thread_rng();
        let secs = Range::new(min.as_secs(), max.as_secs()).ind_sample(&mut rng);
        let subsecs = Range::new(min.subsec_nanos(), max.subsec_nanos()).ind_sample(&mut rng);
        Timeout {
            created_at: Instant::now(),
            expires_after: Duration::new(secs, subsecs),
        }


    }
    pub fn refresh(&mut self) {
        self.created_at = Instant::now();
    }
    pub fn expired(&self) -> bool {
        self.created_at.duration_since(Instant::now()) > self.expires_after
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
