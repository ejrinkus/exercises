const std = @import("std");
const grid = @import("../utils/grid.zig");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;
const AutoArrayHashMap = std.AutoArrayHashMap;
const AutoHashMap = std.AutoHashMap;

const AntennaeMap = AutoArrayHashMap(u8, ArrayList(grid.Coord));
const AntinodeMap = AutoHashMap(u128, bool);

pub fn partOne(allocator: Allocator, input: ArrayList([]u8)) ![]const u8 {
    var antennae = try parseAntennae(allocator, input);
    defer {
        for (antennae.values()) |list| {
            list.deinit();
        }
        antennae.deinit();
    }

    var antinodes = try findAntinodes(allocator, antennae, input.items.len, input.items[0].len);
    defer antinodes.deinit();

    return try std.fmt.allocPrint(allocator, "{d}", .{antinodes.count()});
}

pub fn partTwo(allocator: Allocator, input: ArrayList([]u8)) ![]const u8 {
    var antennae = try parseAntennae(allocator, input);
    defer {
        for (antennae.values()) |list| {
            list.deinit();
        }
        antennae.deinit();
    }

    var antinodes = try findResonantAntinodes(allocator, antennae, input.items.len, input.items[0].len);
    defer antinodes.deinit();

    return try std.fmt.allocPrint(allocator, "{d}", .{antinodes.count()});
}

fn parseAntennae(allocator: Allocator, input: ArrayList([]u8)) !AntennaeMap {
    const height = input.items.len;
    const width = input.items[0].len;

    var antennae = AntennaeMap.init(allocator);
    for (0..height) |row| {
        for (0..width) |col| {
            const val = input.items[row][col];
            if (val == '.') continue;

            const c = grid.Coord{
                .row = row,
                .col = col,
            };
            var entry = try antennae.getOrPut(val);
            if (!entry.found_existing) {
                entry.value_ptr.* = ArrayList(grid.Coord).init(allocator);
            }
            try entry.value_ptr.append(c);
        }
    }

    return antennae;
}

fn findAntinodes(allocator: Allocator, antennae: AntennaeMap, height: usize, width: usize) !AntinodeMap {
    var antinodes = AntinodeMap.init(allocator);

    for (antennae.values()) |locations| {
        if (locations.items.len < 2) continue;
        for (0..locations.items.len - 1) |i| {
            const a1 = locations.items[i];
            for (i + 1..locations.items.len) |j| {
                const a2 = locations.items[j];

                if ((2 * a1.row) >= a2.row and (2 * a1.col) >= a2.col) {
                    const anti = grid.Coord{
                        .row = (2 * a1.row) - a2.row,
                        .col = (2 * a1.col) - a2.col,
                    };

                    if (anti.row < height and anti.col < width) {
                        try antinodes.put(anti.toKey(), true);
                    }
                }

                if ((2 * a2.row) >= a1.row and (2 * a2.col) >= a1.col) {
                    const anti = grid.Coord{
                        .row = (2 * a2.row) - a1.row,
                        .col = (2 * a2.col) - a1.col,
                    };

                    if (anti.row < height and anti.col < width) {
                        try antinodes.put(anti.toKey(), true);
                    }
                }
            }
        }
    }

    return antinodes;
}

fn findResonantAntinodes(allocator: Allocator, antennae: AntennaeMap, height: usize, width: usize) !AntinodeMap {
    var antinodes = AntinodeMap.init(allocator);

    for (antennae.values()) |locations| {
        if (locations.items.len < 2) continue;
        for (0..locations.items.len - 1) |i| {
            const a1 = locations.items[i];
            try antinodes.put(a1.toKey(), true);

            for (i + 1..locations.items.len) |j| {
                const a2 = locations.items[j];
                try antinodes.put(a2.toKey(), true);

                var prev = grid.Coord{
                    .row = a2.row,
                    .col = a2.col,
                };
                var curr = grid.Coord{
                    .row = a1.row,
                    .col = a1.col,
                };
                while ((2 * curr.row) >= prev.row and (2 * curr.col) >= prev.col) {
                    var next = grid.Coord{
                        .row = (2 * curr.row) - prev.row,
                        .col = (2 * curr.col) - prev.col,
                    };

                    if (next.row < height and next.col < width) {
                        try antinodes.put(next.toKey(), true);
                    } else break;

                    prev = curr;
                    curr = next;
                }

                prev = grid.Coord{
                    .row = a1.row,
                    .col = a1.col,
                };
                curr = grid.Coord{
                    .row = a2.row,
                    .col = a2.col,
                };
                while ((2 * curr.row) >= prev.row and (2 * curr.col) >= prev.col) {
                    var next = grid.Coord{
                        .row = (2 * curr.row) - prev.row,
                        .col = (2 * curr.col) - prev.col,
                    };

                    if (next.row < height and next.col < width) {
                        try antinodes.put(next.toKey(), true);
                    } else break;

                    prev = curr;
                    curr = next;
                }
            }
        }
    }

    return antinodes;
}
