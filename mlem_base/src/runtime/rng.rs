use std::{ops::Range, time::{SystemTime, UNIX_EPOCH}};

pub trait Rng<T> {
    fn new() -> T;
    fn from_u64(seed: u64) -> T;
    fn next_u32(&mut self) -> u32;
    
    fn next_f32(&mut self) -> f32 {
        return self.next_u32() as f32 / u32::MAX as f32;
    }

    fn range_f32(&mut self, range: Range<f32>) -> f32 {
        return range.start + (range.end - range.start) * self.next_f32();
    }

    fn range_usize(&mut self, range: Range<usize>) -> usize {
        return f32::floor(range.start as f32 + (range.end as f32 - range.start as f32) * self.next_f32()) as usize;
    }

    fn now() -> u64 {
        let time = SystemTime::now();
        let since = time
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        return since.as_secs();
    }
}

// Copyright 2018 Developers of the Rand project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
/// A splitmix64 random number generator.
///
/// The splitmix algorithm is not suitable for cryptographic purposes, but is
/// very fast and has a 64 bit state.
///
/// The algorithm used here is translated from [the `splitmix64.c`
/// reference source code](http://xoshiro.di.unimi.it/splitmix64.c) by
/// Sebastiano Vigna. For `next_u32`, a more efficient mixing function taken
/// from [`dsiutils`](http://dsiutils.di.unimi.it/) is used.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RngSplitMix64 {
    x: u64,
}

const PHI: u64 = 0x9e3779b97f4a7c15;

#[allow(refining_impl_trait)]
impl Rng<RngSplitMix64> for RngSplitMix64 {
    fn new() -> RngSplitMix64 {
        return Self::from_u64(Self::now());
    }

    fn from_u64(seed: u64) -> RngSplitMix64 {
        return Self {
            x: seed
        }
    }
    
    fn next_u32(&mut self) -> u32 {
        self.x = self.x.wrapping_add(PHI);
        let mut z = self.x;
        // David Stafford's
        // (http://zimbry.blogspot.com/2011/09/better-bit-mixing-improving-on.html)
        // "Mix4" variant of the 64-bit finalizer in Austin Appleby's
        // MurmurHash3 algorithm.
        z = (z ^ (z >> 33)).wrapping_mul(0x62A9D9ED799705F5);
        z = (z ^ (z >> 28)).wrapping_mul(0xCB24D0A5C88C35B3);
        return (z >> 32) as u32;
    }
}