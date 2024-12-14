const std = @import("std");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;

const Guard = struct {
    row: usize = 0,
    col: usize = 0,
    dir: u8,
};

pub fn partOne(allocator: Allocator, input: ArrayList([]u8)) ![]const u8 {
    var sum: u32 = 1;

    var guard: Guard = findGuard(input);
    if (guard.dir == '#') {
        std.debug.print("Couldn't find guard starting position.\n", .{});
        return "";
    }
    input.items[guard.row][guard.col] = 'X';

    while (moveGuard(input, &guard)) {
        if (input.items[guard.row][guard.col] == 'X') {
            continue;
        }
        sum += 1;
        input.items[guard.row][guard.col] = 'X';
    }

    return try std.fmt.allocPrint(allocator, "{d}", .{sum});
}

pub fn partTwo(allocator: Allocator, input: ArrayList([]u8)) ![]const u8 {
    var sum: usize = 0;
    sum += input.items.len;

    return try std.fmt.allocPrint(allocator, "{d}", .{sum});
}

fn findGuard(input: ArrayList([]u8)) Guard {
    const height = input.items.len;
    const width = input.items[0].len;

    for (0..height) |row| {
        for (0..width) |col| {
            const val = input.items[row][col];
            if (val == '^' or val == '>' or val == 'v' or val == '<') {
                return Guard{
                    .row = row,
                    .col = col,
                    .dir = val,
                };
            }
        }
    }

    return Guard{
        .dir = '#',
    };
}

// Returns true if the guard can move again.  Returns false if they moved off the grid.
fn moveGuard(input: ArrayList([]u8), guard: *Guard) bool {
    const height = input.items.len;
    const width = input.items[0].len;

    var nextRow = guard.row;
    var nextCol = guard.col;

    // Figure out where the guard is supposed to go next.
    switch (guard.dir) {
        '^' => {
            if (nextRow <= 0) return false; // guard will move off the top of the grid.
            nextRow -= 1;
        },
        '>' => {
            if (nextCol >= width - 1) return false; // guard will move off the right of the grid.
            nextCol += 1;
        },
        'v' => {
            if (nextRow >= height - 1) return false; // guard will move off the bottom of the grid.
            nextRow += 1;
        },
        '<' => {
            if (nextCol <= 0) return false; // guard will move off the left of the grid.
            nextCol -= 1;
        },
        else => {
            std.debug.print("Uh oh, guard has an invalid direction somehow: {}\n", .{guard.dir});
            return false;
        },
    }

    // Turn the guard, if necessary.  This requires repeating the exit logic above, in case they
    // leave the grid immediately after turning.
    while (input.items[nextRow][nextCol] == '#') {
        switch (guard.dir) {
            '^' => {
                nextRow += 1;
                guard.dir = '>';
                if (nextCol >= width - 1) return false; // guard will move off the right of the grid.
                nextCol += 1;
            },
            '>' => {
                nextCol -= 1;
                guard.dir = 'v';
                if (nextRow >= height - 1) return false; // guard will move off the bottom of the grid.
                nextRow += 1;
            },
            'v' => {
                nextRow -= 1;
                guard.dir = '<';
                if (nextCol <= 0) return false; // guard will move off the left of the grid.
                nextCol -= 1;
            },
            '<' => {
                nextCol += 1;
                guard.dir = '^';
                if (nextRow <= 0) return false; // guard will move off the top of the grid.
                nextRow -= 1;
            },
            else => {
                std.debug.print("Uh oh, guard has an invalid direction somehow: {}\n", .{guard.dir});
                return false;
            },
        }
    }

    guard.row = nextRow;
    guard.col = nextCol;
    return true;
}

fn printState(input: ArrayList([]u8), guard: Guard) void {
    const height = input.items.len;
    const width = input.items[0].len;

    for (0..height) |row| {
        for (0..width) |col| {
            if (row == guard.row and col == guard.col) {
                std.debug.print("{c}", .{guard.dir});
            } else if (input.items[row][col] == '#') {
                std.debug.print("#", .{});
            } else {
                std.debug.print(".", .{});
            }
        }
        std.debug.print("\n", .{});
    }
    std.debug.print("\n", .{});
}
