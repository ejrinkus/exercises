const std = @import("std");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;

pub const GridError = error{
    OutOfBounds,
};

pub const Dir = enum { Up, Down, Left, Right };

pub fn rotateDirClockwise(dir: Dir) Dir {
    return switch (dir) {
        Dir.Up => Dir.Right,
        Dir.Right => Dir.Down,
        Dir.Down => Dir.Left,
        Dir.Left => Dir.Up,
    };
}

pub fn rotateDirAnticlockwise(dir: Dir) Dir {
    return switch (dir) {
        Dir.Up => Dir.Left,
        Dir.Left => Dir.Down,
        Dir.Down => Dir.Right,
        Dir.Right => Dir.Up,
    };
}

pub fn Grid(comptime T: type) type {
    return struct {
        allocator: Allocator,
        inner: [][]T,
        height: usize,
        width: usize,

        const Self = @This();

        pub fn init(allocator: Allocator, height: usize, width: usize) !Self {
            var inner = try allocator.alloc([]T, height);
            for (0..height) |row| {
                inner[row] = try allocator.alloc(T, width);
                @memset(inner[row], 0);
            }
            return Self{
                .allocator = allocator,
                .inner = inner,
                .height = height,
                .width = width,
            };
        }

        pub fn deinit(self: Self) void {
            for (0..self.height) |row| {
                self.allocator.free(self.inner[row]);
            }
            self.allocator.free(self.inner);
        }

        pub fn get(self: Self, coord: Coord) !T {
            if (coord.row < 0 or coord.row >= self.height) return GridError.OutOfBounds;
            if (coord.col < 0 or coord.col >= self.width) return GridError.OutOfBounds;
            return self.inner[coord.row][coord.col];
        }

        pub fn getDir(self: Self, coord: Coord, dir: Dir) !T {
            return switch (dir) {
                Dir.Up => self.getAbove(coord),
                Dir.Down => self.getBelow(coord),
                Dir.Left => self.getLeft(coord),
                Dir.Right => self.getRight(coord),
            };
        }

        pub fn getAbove(self: Self, coord: Coord) !T {
            if (coord.row < 1 or coord.row > self.height) return GridError.OutOfBounds;
            if (coord.col < 0 or coord.col >= self.width) return GridError.OutOfBounds;
            return self.inner[coord.row - 1][coord.col];
        }

        pub fn getAboveRight(self: Self, coord: Coord) !T {
            if (coord.row < 1 or coord.row > self.height) return GridError.OutOfBounds;
            if (coord.col >= self.width - 1) return GridError.OutOfBounds;
            return self.inner[coord.row - 1][coord.col + 1];
        }

        pub fn getRight(self: Self, coord: Coord) !T {
            if (coord.row < 0 or coord.row >= self.height) return GridError.OutOfBounds;
            if (coord.col >= self.width - 1) return GridError.OutOfBounds;
            return self.inner[coord.row][coord.col + 1];
        }

        pub fn getBelowRight(self: Self, coord: Coord) !T {
            if (coord.row >= self.height - 1) return GridError.OutOfBounds;
            if (coord.col >= self.width - 1) return GridError.OutOfBounds;
            return self.inner[coord.row + 1][coord.col + 1];
        }

        pub fn getBelow(self: Self, coord: Coord) !T {
            if (coord.row >= self.height - 1) return GridError.OutOfBounds;
            if (coord.col < 0 or coord.col >= self.width) return GridError.OutOfBounds;
            return self.inner[coord.row + 1][coord.col];
        }

        pub fn getBelowLeft(self: Self, coord: Coord) !T {
            if (coord.row >= self.height - 1) return GridError.OutOfBounds;
            if (coord.col < 1 or coord.col > self.width) return GridError.OutOfBounds;
            return self.inner[coord.row + 1][coord.col - 1];
        }

        pub fn getLeft(self: Self, coord: Coord) !T {
            if (coord.row < 0 or coord.row >= self.height) return GridError.OutOfBounds;
            if (coord.col < 1 or coord.col > self.width) return GridError.OutOfBounds;
            return self.inner[coord.row][coord.col - 1];
        }

        pub fn getAboveLeft(self: Self, coord: Coord) !T {
            if (coord.row < 1 or coord.row > self.height) return GridError.OutOfBounds;
            if (coord.col < 1 or coord.col > self.width) return GridError.OutOfBounds;
            return self.inner[coord.row - 1][coord.col - 1];
        }

        pub fn update(self: Self, coord: Coord, val: T) !void {
            if (coord.row < 0 or coord.row >= self.height) return GridError.OutOfBounds;
            if (coord.col < 0 or coord.col >= self.width) return GridError.OutOfBounds;
            self.inner[coord.row][coord.col] = val;
        }
    };
}

pub const Coord = struct {
    row: usize = 0,
    col: usize = 0,

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

    pub fn translateDir(self: Coord, dir: Dir) !Coord {
        if (dir == Dir.Up and self.row == 0) return GridError.OutOfBounds;
        if (dir == Dir.Left and self.col == 0) return GridError.OutOfBounds;

        return switch (dir) {
            Dir.Up => Coord{
                .row = self.row - 1,
                .col = self.col,
            },
            Dir.Down => Coord{
                .row = self.row + 1,
                .col = self.col,
            },
            Dir.Left => Coord{
                .row = self.row,
                .col = self.col - 1,
            },
            Dir.Right => Coord{
                .row = self.row,
                .col = self.col + 1,
            },
        };
    }
};
