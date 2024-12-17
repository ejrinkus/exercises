const std = @import("std");
const parsing = @import("../utils/parsing.zig");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;
const AutoHashMap = std.AutoHashMap;

const Pair = struct {
    first: u64,
    second: u64,
};

pub fn partOne(allocator: Allocator, input: ArrayList([]u8)) ![]const u8 {
    const initial_rocks = try parsing.parseNums(allocator, u64, input.items[0], ' ');
    defer initial_rocks.deinit();

    var count: usize = 0;
    for (initial_rocks.items) |r| {
        const list = try blinkOneRock(allocator, r, 25);
        defer list.deinit();
        count += list.items.len;
    }

    return try std.fmt.allocPrint(allocator, "{d}", .{count});
}

pub fn partTwo(allocator: Allocator, input: ArrayList([]u8)) ![]const u8 {
    const initial_rocks = try parsing.parseNums(allocator, u64, input.items[0], ' ');
    defer initial_rocks.deinit();

    var memo25 = AutoHashMap(u64, u128).init(allocator);
    var memo50 = AutoHashMap(u64, u128).init(allocator);
    var memo75 = AutoHashMap(u64, u128).init(allocator);
    defer {
        memo25.deinit();
        memo50.deinit();
        memo75.deinit();
    }

    var count: u128 = 0;
    for (initial_rocks.items) |r| {
        // If we've already simulated this rock up to 75 blinks, then use
        // that memoized value.
        if (memo75.contains(r)) {
            const c = memo75.get(r).?;
            count += c;
            continue;
        }

        // Otherwise, blink it 25 times and memoize _that_ value.
        const rocks25 = try blinkOneRock(allocator, r, 25);
        defer rocks25.deinit();

        if (!memo25.contains(r)) try memo25.put(r, rocks25.items.len);

        // This count represents the result of blinking r 75 times.
        var count2: u128 = 0;
        for (rocks25.items) |r2| {
            // Same as the outer loop, except we only need to blink r2 50 times.
            // Make sure we're updating all the outer counts as well.
            if (memo50.contains(r2)) {
                const c = memo50.get(r2).?;
                count += c;
                count2 += c;
                continue;
            }

            const rocks50 = try blinkOneRock(allocator, r2, 25);
            defer rocks50.deinit();

            if (!memo25.contains(r2)) try memo25.put(r2, rocks50.items.len);

            var count3: u128 = 0;
            for (rocks50.items) |r3| {
                // Same as the outer loops, except we only need to blink r3
                // 25 times (making this the final nested loop).  Again, make
                // sure to update all the outer counts.
                if (memo25.contains(r3)) {
                    const c = memo25.get(r3).?;
                    count += c;
                    count2 += c;
                    count3 += c;
                    continue;
                }

                const rocks75 = try blinkOneRock(allocator, r3, 25);
                defer rocks75.deinit();

                if (!memo25.contains(r3)) try memo25.put(r3, rocks75.items.len);

                count += rocks75.items.len;
                count2 += rocks75.items.len;
                count3 += rocks75.items.len;
            }

            // We blinked r2 50 times, so memoize accordingly.
            try memo50.put(r2, count3);
        }

        // We blinked r 75 times, so memoize accordingly
        try memo75.put(r, count2);
    }

    return try std.fmt.allocPrint(allocator, "{d}", .{count});
}

fn splitNum(num: u64) ?Pair {
    var pair = Pair{
        .first = num,
        .second = 0,
    };

    var sentinel = num;
    var tens: u64 = 1;
    while (sentinel > 9) {
        pair.second = (pair.second) + ((pair.first % 10) * tens);
        pair.first /= 10;
        sentinel /= 100;
        tens *= 10;
    }

    if (sentinel > 0) {
        // If num had an even number of digits, then sentinel will be 0 by now (final iteration
        // would've been a 2-digit number divided by 100, which is zero).  If we get here, then
        // sentinel should've been left as a single-digit nonzero number.
        return null;
    }

    return pair;
}

fn blinkOneRock(allocator: Allocator, seed: u64, times: usize) !ArrayList(u64) {
    var rocks = try ArrayList(u64).initCapacity(allocator, 1396875);
    var next_rocks = try ArrayList(u64).initCapacity(allocator, 1396875);
    defer next_rocks.deinit();

    try rocks.append(seed);

    for (0..times) |_| {
        for (0..rocks.items.len) |i| {
            const rock = rocks.items[i];
            if (rock == 0) {
                try next_rocks.append(1);
            } else {
                const maybe_split = splitNum(rock);
                if (maybe_split == null) {
                    try next_rocks.append(rock * 2024);
                } else {
                    try next_rocks.append(maybe_split.?.first);
                    try next_rocks.append(maybe_split.?.second);
                }
            }
        }
        const temp = rocks;
        rocks = next_rocks;
        next_rocks = temp;
        next_rocks.clearRetainingCapacity();
    }

    return rocks;
}
