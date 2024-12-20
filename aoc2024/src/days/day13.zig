const std = @import("std");
const parsing = @import("../utils/parsing.zig");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;

const Tuple3 = struct {
    first: i128 = 0,
    second: i128 = 0,
    third: i128 = 0,
};

pub fn partOne(allocator: Allocator, input: ArrayList([]u8)) ![]const u8 {
    var sum: i128 = 0;

    var i: usize = 0;
    while (i <= input.items.len - 3) {
        const a_button = try parseLine(input.items[i], '+');
        i += 1;
        const b_button = try parseLine(input.items[i], '+');
        i += 1;
        const prize = try parseLine(input.items[i], '=');
        i += 2; // skip the blank line.

        const result = cramerSolve(a_button, b_button, prize);
        if (result.third == 0) continue; // Couldn't get the prize
        sum += result.third;
    }

    const solution = try std.fmt.allocPrint(allocator, "{d}", .{sum});
    return solution;
}

pub fn partTwo(allocator: Allocator, input: ArrayList([]u8)) ![]const u8 {
    var sum: i128 = 0;

    var i: usize = 0;
    while (i <= input.items.len - 3) {
        const a_button = try parseLine(input.items[i], '+');
        i += 1;
        const b_button = try parseLine(input.items[i], '+');
        i += 1;
        var prize = try parseLine(input.items[i], '=');
        i += 2; // skip the blank line.
        prize.first += 10000000000000;
        prize.second += 10000000000000;

        const result = cramerSolve(a_button, b_button, prize);
        if (result.third == 0) continue; // Couldn't get the prize
        sum += result.third;
    }

    const solution = try std.fmt.allocPrint(allocator, "{d}", .{sum});
    return solution;
}

fn parseLine(line: []u8, delim: u8) !Tuple3 {
    var nums = Tuple3{};
    var num_start = parsing.findChar(line, delim, 0).?;
    nums.first = (try parsing.nextNum(i128, line, &num_start)).?;
    nums.second = (try parsing.nextNum(i128, line, &num_start)).?;
    return nums;
}

// Solving the system of equations below:
//
// (n * ax) + (m * bx) = px
// (n * ay) + (m * by) = py
fn cramerSolve(a: Tuple3, b: Tuple3, c: Tuple3) Tuple3 {
    const n_num = @abs((c.first * b.second) - (b.first * c.second));
    const m_num = @abs((a.first * c.second) - (c.first * a.second));
    const den = @abs((a.first * b.second) - (b.first * a.second));

    if (den == 0) return Tuple3{};
    if (n_num % den != 0) return Tuple3{};
    if (m_num % den != 0) return Tuple3{};

    const n = @as(i128, @intCast(n_num / den));
    const m = @as(i128, @intCast(m_num / den));
    const cost = (3 * n) + m;

    return Tuple3{ .first = n, .second = m, .third = cost };
}
