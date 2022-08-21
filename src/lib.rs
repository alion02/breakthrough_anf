//! An implementation of the game of Breakthrough, along with an ANF-based solver for it.
//!
//! # Examples
//!
//! ```
//! use breakthrough_anf::*;
//! let starting_position = State::<6, 8>::default();
//! let is_winning = starting_position.children().any(State::is_lost);
//! assert!(!is_winning);
//! ```
//!
//! ```compile_fail
//! use breakthrough_anf::*;
//! let excessively_large = State::<50, 50>::default();
//! ```
//!
//! ```compile_fail
//! use breakthrough_anf::*;
//! let not_tall_enough = State::<14, 3>::default();
//! ```

use bit_iter::BitIter;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct State<const WIDTH: u32, const HEIGHT: u32> {
    me: u64,
    them: u64,
}

impl<const WIDTH: u32, const HEIGHT: u32> Default for State<WIDTH, HEIGHT> {
    fn default() -> Self {
        let me = Self::ROW_MASK | Self::ROW_MASK << WIDTH;
        let them = me << Self::AREA - 2 * WIDTH;
        Self { me, them }
    }
}

impl<const WIDTH: u32, const HEIGHT: u32> State<WIDTH, HEIGHT> {
    pub const AREA: u32 = {
        let area = WIDTH * HEIGHT;
        assert!(HEIGHT >= 4);
        assert!(area <= u64::BITS);
        area
    };
    pub const ROW_MASK: u64 = {
        let bit = 1 << (WIDTH - 1);
        (bit - 1) | bit
    };

    #[inline]
    pub fn children(mut self) -> impl Iterator<Item = Self> {
        // Generate moves from flipped perspective.
        (self.them, self.me) = (
            self.me.reverse_bits() >> (u64::BITS - Self::AREA),
            self.them.reverse_bits() >> (u64::BITS - Self::AREA),
        );

        BitIter::from(self.them).flat_map(move |(i, bit)| {
            // TODO: Bit-based row mask discovery for non-power-of-two widths.
            let row_mask = Self::ROW_MASK << (i / WIDTH - 1) * WIDTH;

            let diagonals = bit >> WIDTH - 1 | bit >> WIDTH + 1;
            let forward = bit >> WIDTH & !self.me;

            let move_mask = (forward | diagonals) & !self.them & row_mask;
            let new_them = self.them ^ bit;

            BitIter::from(move_mask).map(move |(_, bit)| Self {
                me: self.me & !bit,
                them: new_them ^ bit,
            })
        })
    }

    #[inline]
    pub fn is_lost(self) -> bool {
        self.them & Self::ROW_MASK != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn perft<const WIDTH: u32, const HEIGHT: u32>(state: State<WIDTH, HEIGHT>, depth: u32) -> u64 {
        if depth == 0 {
            1
        } else {
            state.children().map(|state| perft(state, depth - 1)).sum()
        }
    }

    #[test]
    fn perft_1() {
        assert_eq!(perft(State::<4, 16>::default(), 1), 10);
    }

    #[test]
    fn perft_2() {
        assert_eq!(perft(State::<4, 16>::default(), 2), 100);
    }

    #[test]
    fn perft_3() {
        assert_eq!(perft(State::<4, 16>::default(), 3), 1100);
    }

    #[test]
    fn perft_4() {
        assert_eq!(perft(State::<4, 16>::default(), 4), 12100);
    }

    // TODO: Perft 5.
    // TODO: Combat perft.
}
