const std = @import("std");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;

const Equation = struct {
    result: u128,
    values: ArrayList(u32),
};

pub fn partOne(allocator: Allocator, input: ArrayList([]u8)) ![]const u8 {
    var sum: u128 = 0;

    // We only process one equation at a time, so we can pre-allocate.
    var equation = Equation{
        .result = 0,
        .values = ArrayList(u32).init(allocator),
    };
    defer equation.values.deinit();

    // Validate each equation.
    for (input.items) |line| {
        try parseEquation(line, &equation);
        defer equation.values.clearRetainingCapacity();

        if (validateEquation(equation, equation.result, equation.values.items.len - 1, false)) {
            sum += equation.result;
        }
    }

    return try std.fmt.allocPrint(allocator, "{d}", .{sum});
}

pub fn partTwo(allocator: Allocator, input: ArrayList([]u8)) ![]const u8 {
    var sum: u128 = 0;

    // We only process one equation at a time, so we can pre-allocate.
    var equation = Equation{
        .result = 0,
        .values = ArrayList(u32).init(allocator),
    };
    defer equation.values.deinit();

    // Validate each equation.
    for (input.items) |line| {
        try parseEquation(line, &equation);
        defer equation.values.clearRetainingCapacity();

        if (validateEquation(equation, equation.result, equation.values.items.len - 1, true)) {
            sum += equation.result;
        }
    }

    return try std.fmt.allocPrint(allocator, "{d}", .{sum});
}

fn parseEquation(line: []u8, equation: *Equation) !void {
    var mark: usize = 0;
    for (line, 0..) |char, i| {
        if (char == ':') {
            equation.result = try std.fmt.parseInt(u128, line[0..i], 10);
            continue;
        }

        if (char != ' ') continue;

        if (mark == 0) {
            mark = i;
            continue;
        }

        try equation.values.append(try std.fmt.parseInt(u32, line[mark + 1 .. i], 10));
        mark = i;
    }
    try equation.values.append(try std.fmt.parseInt(u32, line[mark + 1 ..], 10));
}

// validateEquation works backwards to try to avoid overflow.  At least for part one we don't
// actually care about what the list of operators was, so we can ignore that and work recursively.
fn validateEquation(equation: Equation, carry: u128, idx: usize, with_concat: bool) bool {
    if (carry == 0) return false;

    var val = equation.values.items[idx];
    if (idx == 0) return carry == val;

    // We can only try a '+' here if the remaining carry is greater than the current value
    // (otherwise we'll go negative).
    if (carry > val) {
        const new_carry = carry - val;
        if (validateEquation(equation, new_carry, idx - 1, with_concat)) return true;
    }

    // We can only try a '*' here if the remaining carry is divisible by the current value
    // (otherwise we'll end up with a floating point value, which isn't valid input).
    if (carry % val == 0) {
        const new_carry = carry / val;
        if (validateEquation(equation, new_carry, idx - 1, with_concat)) return true;
    }

    // We can only try a '||' here if concatenation is enabled and carry is greater than the current
    // value.
    if (with_concat and carry > val) {
        var new_carry = carry;
        var valid = true;
        while (val > 0) {
            if (new_carry % 10 != val % 10) {
                // Digit mismatch, concat isn't valid.
                valid = false;
                break;
            }
            new_carry /= 10;
            val /= 10;
        }
        if (valid and validateEquation(equation, new_carry, idx - 1, with_concat)) return true;
    }

    return false;
}
