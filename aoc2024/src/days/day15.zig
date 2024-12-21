const std = @import("std");
const grid = @import("../utils/grid.zig");
const parsing = @import("../utils/parsing.zig");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;

pub fn partOne(allocator: Allocator, input: ArrayList([]u8)) ![]const u8 {
    var sum: usize = 0;

    var space_i: usize = 0;
    for (input.items) |line| {
        if (line.len == 0) break;
        space_i += 1;
    }

    const width = input.items[0].len;
    const height = space_i;

    const g = try grid.Grid(u8).init(allocator, height, width);
    defer g.deinit();
    var bot = grid.Coord{};
    for (0..height) |row| {
        for (0..width) |col| {
            g.inner[row][col] = input.items[row][col];

            if (g.inner[row][col] == '@') {
                bot.row = row;
                bot.col = col;
            } else if (g.inner[row][col] == 'O') {
                // We calculate the current GPS sum, and we'll update it opportunistically.
                sum += 100 * row + col;
            }
        }
    }

    for (space_i + 1..input.items.len) |i| {
        const line = input.items[i];
        for (line) |d| {
            const dir = switch (d) {
                '^' => grid.Dir.Up,
                '>' => grid.Dir.Right,
                'v' => grid.Dir.Down,
                '<' => grid.Dir.Left,
                else => std.debug.panic("Unexpected direction value: {c}\n", .{d}),
            };

            const dest = try g.getDir(bot, dir);
            if (dest == '#') continue;
            if (dest == '.') {
                try g.update(bot, '.');
                bot = try bot.translateDir(dir);
                try g.update(bot, '@');
                continue;
            }
            if (dest != 'O') std.debug.panic("Unexpected grid value: {c}\n", .{dest});

            // If we get here, we're hitting a box.
            var space = grid.Coord{
                .row = bot.row,
                .col = bot.col,
            };
            while (true) {
                const space_dest = try g.get(space);
                if (space_dest == '.') {
                    // Found an empty space this line of boxes can fill.  Fill the space with a box,
                    // move the bot, then overwrite the box we ran into with the bot.
                    try g.update(space, 'O');
                    try g.update(bot, '.');
                    bot = try bot.translateDir(dir);
                    try g.update(bot, '@');
                    // We also need to update the GPS sum.  Remove the GPS value of the bot (since
                    // there is no longer a box there), and then add the GPS value of the space we
                    // just filled.
                    sum -= 100 * bot.row + bot.col;
                    sum += 100 * space.row + space.col;
                    break;
                }
                if (space_dest == '#') break; // Hit a wall, nothing to do.
                space = try space.translateDir(dir);
            }
        }
    }

    const solution = try std.fmt.allocPrint(allocator, "{d}", .{sum});
    return solution;
}

pub fn partTwo(allocator: Allocator, input: ArrayList([]u8)) ![]const u8 {
    const sum = input.items.len;

    const solution = try std.fmt.allocPrint(allocator, "{d}", .{sum});
    return solution;
}
