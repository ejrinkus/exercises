const std = @import("std");
const grid = @import("../utils/grid.zig");
const parsing = @import("../utils/parsing.zig");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;
const AutoHashMap = std.AutoHashMap;

pub fn partOne(allocator: Allocator, input: ArrayList([]u8)) ![]const u8 {
    const height = input.items.len;
    const width = input.items[0].len;
    var sum: u32 = 0;

    var visited = AutoHashMap(u128, bool).init(allocator);
    defer visited.deinit();
    for (0..height) |row| {
        for (0..width) |col| {
            if (input.items[row][col] != '0') continue;

            const here = grid.Coord{
                .row = row,
                .col = col,
            };

            sum += try calcScore(input, &visited, here, height, width);
            visited.clearRetainingCapacity();
        }
    }

    return try std.fmt.allocPrint(allocator, "{d}", .{sum});
}

pub fn partTwo(allocator: Allocator, input: ArrayList([]u8)) ![]const u8 {
    const height = input.items.len;
    const width = input.items[0].len;
    var sum: u32 = 0;

    for (0..height) |row| {
        for (0..width) |col| {
            if (input.items[row][col] != '0') continue;

            const here = grid.Coord{
                .row = row,
                .col = col,
            };

            sum += calcRating(input, here, height, width);
        }
    }

    return try std.fmt.allocPrint(allocator, "{d}", .{sum});
}

fn calcScore(map: ArrayList([]u8), visited: *AutoHashMap(u128, bool), start: grid.Coord, height: usize, width: usize) !u32 {
    const here = map.items[start.row][start.col];
    try visited.put(start.toKey(), true);
    if (here == '9') {
        return 1;
    }

    var result: u32 = 0;
    const next = here + 1;

    if (start.row > 0 and map.items[start.row - 1][start.col] == next) {
        // Up
        const next_coord = grid.Coord{
            .row = start.row - 1,
            .col = start.col,
        };
        if (!visited.contains(next_coord.toKey())) result += try calcScore(map, visited, next_coord, height, width);
    }
    if (start.row < height - 1 and map.items[start.row + 1][start.col] == next) {
        // Down
        const next_coord = grid.Coord{
            .row = start.row + 1,
            .col = start.col,
        };
        if (!visited.contains(next_coord.toKey())) result += try calcScore(map, visited, next_coord, height, width);
    }
    if (start.col > 0 and map.items[start.row][start.col - 1] == next) {
        const next_coord = grid.Coord{
            .row = start.row,
            .col = start.col - 1,
        };
        if (!visited.contains(next_coord.toKey())) result += try calcScore(map, visited, next_coord, height, width);
    }
    if (start.col < width - 1 and map.items[start.row][start.col + 1] == next) {
        const next_coord = grid.Coord{
            .row = start.row,
            .col = start.col + 1,
        };
        if (!visited.contains(next_coord.toKey())) result += try calcScore(map, visited, next_coord, height, width);
    }

    return result;
}

fn calcRating(map: ArrayList([]u8), start: grid.Coord, height: usize, width: usize) u32 {
    const here = map.items[start.row][start.col];
    if (here == '9') {
        return 1;
    }

    var result: u32 = 0;
    const next = here + 1;

    if (start.row > 0 and map.items[start.row - 1][start.col] == next) {
        // Up
        const next_coord = grid.Coord{
            .row = start.row - 1,
            .col = start.col,
        };
        result += calcRating(map, next_coord, height, width);
    }
    if (start.row < height - 1 and map.items[start.row + 1][start.col] == next) {
        // Down
        const next_coord = grid.Coord{
            .row = start.row + 1,
            .col = start.col,
        };
        result += calcRating(map, next_coord, height, width);
    }
    if (start.col > 0 and map.items[start.row][start.col - 1] == next) {
        const next_coord = grid.Coord{
            .row = start.row,
            .col = start.col - 1,
        };
        result += calcRating(map, next_coord, height, width);
    }
    if (start.col < width - 1 and map.items[start.row][start.col + 1] == next) {
        const next_coord = grid.Coord{
            .row = start.row,
            .col = start.col + 1,
        };
        result += calcRating(map, next_coord, height, width);
    }

    return result;
}
