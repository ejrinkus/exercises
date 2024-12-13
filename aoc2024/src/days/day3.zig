const std = @import("std");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;

const Result = struct {
    value: u32,
    enabled: bool,
};

pub fn partOne(allocator: Allocator, input: ArrayList([]u8)) ![]const u8 {
    var sum: u32 = 0;
    for (input.items) |line| {
        const result = try evaluate(line, false, true);
        sum += result.value;
    }

    return std.fmt.allocPrint(allocator, "{d}", .{sum});
}

pub fn partTwo(allocator: Allocator, input: ArrayList([]u8)) ![]const u8 {
    var sum: u32 = 0;
    var enabled = true;
    for (input.items) |line| {
        const result = try evaluate(line, true, enabled);
        sum += result.value;
        enabled = result.enabled;
    }

    return std.fmt.allocPrint(allocator, "{d}", .{sum});
}

fn evaluate(line: []const u8, cond: bool, initial_enabled: bool) !Result {
    var sum: u32 = 0;
    var in_mul = false;
    var first_start: ?usize = null;
    var first_end: ?usize = null;
    var second_start: ?usize = null;
    var second_end: ?usize = null;
    var enabled = initial_enabled;
    for (3..line.len) |i| {
        if (cond) {
            // Enable multiplication if we find a do().  But reset everything else.
            if (std.mem.eql(u8, line[i - 3 .. i + 1], "do()")) {
                enabled = true;
                in_mul = false;
                first_start = null;
                first_end = null;
                second_start = null;
                second_end = null;
                continue;
            }

            // Disable multiplication if we find a don't().  Still reset everything else.
            if (i >= 6 and std.mem.eql(u8, line[i - 6 .. i + 1], "don't()")) {
                enabled = false;
                in_mul = false;
                first_start = null;
                first_end = null;
                second_start = null;
                second_end = null;
                continue;
            }

            // If multiplication is disabled, there's nothing left to do.
            if (!enabled) continue;
        }

        // Found the start of a mul() instruction.
        if (std.mem.eql(u8, line[i - 3 .. i + 1], "mul(")) {
            in_mul = true;
            first_start = i + 1;
            first_end = null;
            second_start = null;
            second_end = null;
            continue;
        }

        // Not the start of a mul() instruction, and we aren't already in one.  Move on.
        if (!in_mul) {
            continue;
        }

        // In a mul instruction.  Make sure we have either a digit or a comma.
        switch (line[i]) {
            '0', '1', '2', '3', '4', '5', '6', '7', '8', '9' => {
                continue;
            },
            ',' => {
                if (second_start != null) {
                    // Found an extra comma; bail.
                    in_mul = false;
                    first_start = null;
                    first_end = null;
                    second_start = null;
                    second_end = null;
                    continue;
                }
                first_end = i;
                second_start = i + 1;
                continue;
            },
            ')' => {
                if (first_start == null or first_end == null or second_start == null) {
                    // Found the closing parenthesis too early; bail.
                    in_mul = false;
                    first_start = null;
                    first_end = null;
                    second_start = null;
                    second_end = null;
                    continue;
                }
                second_end = i;
                const first = try std.fmt.parseInt(u32, line[first_start.?..first_end.?], 10);
                const second = try std.fmt.parseInt(u32, line[second_start.?..second_end.?], 10);
                sum += first * second;
                in_mul = false;
                first_start = null;
                first_end = null;
                second_start = null;
                second_end = null;
            },
            else => {
                // Bogus character inside mul instruction; bail.
                in_mul = false;
                first_start = null;
                first_end = null;
                second_start = null;
                second_end = null;
            },
        }
    }
    return .{
        .value = sum,
        .enabled = enabled,
    };
}
