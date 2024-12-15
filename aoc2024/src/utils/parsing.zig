const std = @import("std");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;

pub fn parseNums(allocator: Allocator, comptime T: type, line: []const u8, delim: u8) !ArrayList(T) {
    var list = ArrayList(T).init(allocator);

    var start: ?usize = null;
    for (line, 0..) |value, i| {
        if (value != delim) {
            // In a number.
            if (start == null) {
                // Record location of first digit.
                start = i;
            }
            // Nothing else to do.
            continue;
        }
        if (start == null) {
            // Consecutive delimiters; nothing to do.
            continue;
        }
        // Found first delimiter after end of a number.  Parse the number and add it to our list.
        const num = std.fmt.parseInt(T, line[start.?..i], 10) catch unreachable;
        try list.append(num);
        start = null;
    }

    if (start != null) {
        const num = std.fmt.parseInt(T, line[start.?..], 10) catch unreachable;
        try list.append(num);
    }

    return list;
}

pub fn findChar(line: []const u8, char: u8, start: usize) ?usize {
    for (start..line.len) |i| {
        if (line[i] == char) return i;
    }
    return null;
}

pub fn charToDigit(char: u8) ?u8 {
    if (char < '0' or char > '9') return null;
    return char - '0';
}

pub fn nextNum(comptime T: type, line: []const u8, idx: *usize) std.fmt.ParseIntError!?T {
    var num_start: ?usize = null;
    var num_end: ?usize = null;

    for (idx..line.len) |i| {
        if (charToDigit(line[i]) == null) {
            if (num_start == null) continue;
            num_end = i;
            break;
        }
        if (num_start == null) num_start = i;
    }

    if (num_start == null) return null;
    if (num_end == null) num_end = line.len;

    idx.* = num_end;
    return try std.fmt.parseInt(T, line[num_start.?..num_end.?], 10);
}
