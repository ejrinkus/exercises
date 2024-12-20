const std = @import("std");
const parsing = @import("../utils/parsing.zig");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;

const Bot = struct {
    px: i32 = 0,
    py: i32 = 0,
    vx: i32 = 0,
    vy: i32 = 0,
};

pub fn partOne(allocator: Allocator, input: ArrayList([]u8)) ![]const u8 {
    var quad1: i32 = 0;
    var quad2: i32 = 0;
    var quad3: i32 = 0;
    var quad4: i32 = 0;
    for (input.items) |line| {
        var bot = try parseBot(line);

        bot.px = @mod(bot.vx * 100 + bot.px, 101);
        bot.py = @mod(bot.vy * 100 + bot.py, 103);

        if (bot.px < 50 and bot.py < 51) quad1 += 1;
        if (bot.px > 50 and bot.py < 51) quad2 += 1;
        if (bot.px < 50 and bot.py > 51) quad3 += 1;
        if (bot.px > 50 and bot.py > 51) quad4 += 1;
    }

    const solution = try std.fmt.allocPrint(allocator, "{d}", .{quad1 * quad2 * quad3 * quad4});
    return solution;
}

pub fn partTwo(allocator: Allocator, input: ArrayList([]u8)) ![]const u8 {
    var grid: [103][101]u8 = [_][101]u8{[_]u8{'.'} ** 101} ** 103;
    var bots = ArrayList(Bot).init(allocator);
    defer bots.deinit();

    for (input.items) |line| {
        const bot = try parseBot(line);
        try bots.append(bot);

        const row = @as(usize, @intCast(bot.py));
        const col = @as(usize, @intCast(bot.px));
        grid[row][col] = 'X';
    }

    for (1..10001) |s| {
        for (0..bots.items.len) |i| {
            var bot: *Bot = &bots.items[i];

            var row = @as(usize, @intCast(bot.py));
            var col = @as(usize, @intCast(bot.px));

            grid[row][col] = '.';

            bot.px = @mod(bot.vx + bot.px, 101);
            bot.py = @mod(bot.vy + bot.py, 103);

            row = @as(usize, @intCast(bot.py));
            col = @as(usize, @intCast(bot.px));

            grid[row][col] = 'X';
        }

        if (findHeurSeq(grid)) {
            std.debug.print("\nAfter {d} seconds:\n", .{s});
            printGrid(grid);
            const solution = try std.fmt.allocPrint(allocator, "{d}", .{s});
            return solution;
        }
    }

    const solution = try std.fmt.allocPrint(allocator, "", .{});
    return solution;
}

fn parseBot(line: []u8) !Bot {
    var cursor: usize = 0;
    const px = (try parsing.nextNum(i32, line, &cursor)).?;
    const py = (try parsing.nextNum(i32, line, &cursor)).?;
    const vx = (try parsing.nextNum(i32, line, &cursor)).?;
    const vy = (try parsing.nextNum(i32, line, &cursor)).?;
    return Bot{
        .px = px,
        .py = py,
        .vx = vx,
        .vy = vy,
    };
}

fn printGrid(grid: [103][101]u8) void {
    for (0..103) |row| {
        for (0..101) |col| {
            std.debug.print("{c}", .{grid[row][col]});
        }
        std.debug.print("\n", .{});
    }
}

fn findHeurSeq(grid: [103][101]u8) bool {
    for (0..103) |row| {
        for (0..71) |col| {
            if (std.mem.eql(u8, grid[row][col .. col + 30], "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXX")) {
                return true;
            }
        }
    }
    return false;
}
