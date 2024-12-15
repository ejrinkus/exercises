const std = @import("std");
const ArrayList = std.ArrayList;

pub const Coord = struct {
    row: usize,
    col: usize,

    pub fn toKey(self: Coord) u128 {
        var key: u128 = self.row;
        key <<= 64; // Put the row into the upper half of the key.
        key |= self.col; // And put the col in the lower half.
        return key;
    }

    pub fn fromKey(key: u128) Coord {
        const zero: usize = 0;
        const row: usize = key >> 64;
        const col: usize = ((key << 64) >> 64) | zero;

        return Coord{
            .row = row,
            .col = col,
        };
    }
};
