const std = @import("std");
const Allocator = std.mem.Allocator;

pub fn parseNums(allocator: Allocator, comptime T: type, line: []const u8, delim: u8) ![]const T {
    var list = std.ArrayList(T).init(allocator);
    defer list.deinit();

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

    return list.toOwnedSlice();
}
