const std = @import("std");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;
const AutoHashMap = std.AutoHashMap;

const Guard = struct {
    row: usize = 0,
    col: usize = 0,
    dir: u8,
};

pub fn partOne(allocator: Allocator, input: ArrayList([]u8)) ![]const u8 {
    var sum: u32 = 1;

    // Initialize our guard in their starting position.
    var guard: Guard = findGuard(input);
    if (guard.dir == '#') {
        std.debug.print("Couldn't find guard starting position.\n", .{});
        return "";
    }

    // Repeatedly move the guard, updating the grid every time they visit a new location (and
    // increment our sum every time they do visit a new location).
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
    var sum: u32 = 0;

    // Initialize our guard in their starting position.
    var guard: Guard = findGuard(input);
    if (guard.dir == '#') {
        std.debug.print("Couldn't find guard starting position.\n", .{});
        return "";
    }

    // To avoid repeated reallocations of simulated grids, we'll initialize one grid up front and
    // then just memcpy over it as needed.
    var sim_grid = ArrayList([]u8).init(allocator);
    defer {
        for (sim_grid.items) |slice| {
            allocator.free(slice);
        }
        sim_grid.deinit();
    }
    for (input.items) |slice| {
        const new_slice = try allocator.alloc(u8, slice.len);
        @memcpy(new_slice, slice);
        try sim_grid.append(new_slice);
    }

    // The grid is initially populated with ascii symbols, which all have a decimal value greater
    // than 32.  Additionally, our traversal bitmask is only 4 bits wide (values 0-15 decimal).
    // So if we want to see if a location was previously traversed, we can just check that it's
    // less than 16.
    input.items[guard.row][guard.col] = dirMask(guard.dir);

    var prev_guard = Guard{
        .row = guard.row,
        .col = guard.col,
        .dir = guard.dir,
    };
    var should_test: bool = undefined;
    while (moveGuard(input, &guard)) {
        if (input.items[guard.row][guard.col] >= 16) {
            // If this is a novel location, set it to the guard's current direction.
            // This also means we need to test placing an obstacle here.
            input.items[guard.row][guard.col] = dirMask(guard.dir);
            should_test = true;
        } else {
            // Otherwise, OR the guard's current direction into the existing mask.
            // Don't test an obstacle here, since we would've already done it earlier on the path
            // (i.e. placing it earlier would've already altered the path).
            input.items[guard.row][guard.col] |= dirMask(guard.dir);
            should_test = false;
        }

        if (guard.dir != prev_guard.dir) {
            // We just turned, ensure this direction is a part of the previous spot's mask.
            input.items[prev_guard.row][prev_guard.col] |= dirMask(guard.dir);
        }

        if (!should_test) {
            // Reset prev_guard to the current state of the guard for the next iteration.
            prev_guard.row = guard.row;
            prev_guard.col = guard.col;
            prev_guard.dir = guard.dir;
            continue;
        }

        // Copy the updated grid and run our simulation.
        for (input.items, sim_grid.items) |slice, sim_slice| {
            @memcpy(sim_slice, slice);
        }

        // First, set the guard's current location to an obstacle.  Then, simulate prev_guard's
        // new path with this obstacle in place to see if we find a loop.
        sim_grid.items[guard.row][guard.col] = '#';
        if (findLoop(sim_grid, &prev_guard)) {
            sum += 1;
        }

        // Reset prev_guard to the current state of the guard for the next iteration.
        prev_guard.row = guard.row;
        prev_guard.col = guard.col;
        prev_guard.dir = guard.dir;
    }

    return try std.fmt.allocPrint(allocator, "{d}", .{sum});
}

pub fn findLoop(input: ArrayList([]u8), guard: *Guard) bool {
    // If the guard is already surrounded by obstacles, we're in a loop.  Return now since
    // moveGuard could break in this edge case.
    if (guard.row > 0 and (input.items[guard.row - 1][guard.col] == '#') and
        guard.col > 0 and (input.items[guard.row][guard.col - 1] == '#') and
        guard.row < input.items.len - 1 and (input.items[guard.row + 1][guard.col] == '#') and
        guard.col < input.items[0].len - 1 and (input.items[guard.row][guard.col + 1] == '#'))
    {
        return true;
    }

    // The current location of the guard should already have the correct direction mask, since it's
    // a location that the guard had already previously visited.  So just start the simulated path.
    while (moveGuard(input, guard)) {
        const space = &input.items[guard.row][guard.col];
        const space_mask = space.*;
        const guard_mask = dirMask(guard.dir);
        if (space_mask >= 16) {
            // This is a novel location.
            space.* = guard_mask;
        } else {
            // This isn't a novel location.  See if we've already traveled through this location in
            // the same direction before.
            if ((space_mask & guard_mask) == guard_mask) {
                // We found a loop.
                return true;
            }
            // No loop, update the direction mask and move on.
            space.* |= guard_mask;
        }
    }

    return false;
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

fn dirMask(dir: u8) u8 {
    return switch (dir) {
        '^' => 0b00001000,
        '>' => 0b00000100,
        'v' => 0b00000010,
        '<' => 0b00000001,
        else => 0b00000000,
    };
}
