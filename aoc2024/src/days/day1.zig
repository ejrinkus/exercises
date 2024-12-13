const std = @import("std");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;
const AutoHashMap = std.AutoHashMap;

const ids = struct {
    first: u32,
    second: u32,
};

pub fn partOne(allocator: Allocator, input: ArrayList([]u8)) ![]const u8 {
    // Containers for our input lists.
    var a_list = ArrayList(u32).init(allocator);
    defer a_list.deinit();

    var b_list = ArrayList(u32).init(allocator);
    defer b_list.deinit();

    // Read lines and process them.
    for (input.items) |line| {
        const parsed = get_parsed(line);
        try a_list.append(parsed.first);
        try b_list.append(parsed.second);
    }

    // We've parsed the input into our lists.  Sort them both so they're matched up.
    std.mem.sort(u32, a_list.items, {}, comptime std.sort.asc(u32));
    std.mem.sort(u32, b_list.items, {}, comptime std.sort.asc(u32));

    // Now calculate the sum of distances.
    var sum: u32 = 0;
    for (a_list.items, b_list.items) |a, b| {
        if (a < b) {
            sum += (b - a);
        } else {
            sum += (a - b);
        }
    }

    const solution = try std.fmt.allocPrint(allocator, "{d}", .{sum});
    return solution;
}

pub fn partTwo(allocator: Allocator, input: ArrayList([]u8)) ![]const u8 {
    // Containers for our input lists.
    var a_list = ArrayList(u32).init(allocator);
    defer a_list.deinit();

    var b_map = AutoHashMap(u32, u32).init(allocator);
    defer b_map.deinit();

    // Read lines and process them.
    for (input.items) |line| {
        // Just keep the first list as a list.
        const parsed = get_parsed(line);
        try a_list.append(parsed.first);

        // But for the second list, store it as a map (id -> # occurrences of that id).
        const prev = b_map.get(parsed.second) orelse 0;
        try b_map.put(parsed.second, prev + 1);
    }

    // We've parsed the input into our lists.  Sort them both so they're matched up.
    std.mem.sort(u32, a_list.items, {}, comptime std.sort.asc(u32));

    // Now calculate the sum of similarities.
    var sum: u32 = 0;
    for (a_list.items) |a| {
        const b = b_map.get(a) orelse 0;
        sum += a * b;
    }

    return std.fmt.allocPrint(allocator, "{d}", .{sum});
}

fn get_parsed(line: []const u8) ids {
    var end1: usize = 0;
    var start2: usize = 0;
    for (line, 0..) |value, i| {
        if (value == ' ') {
            if (end1 == 0) {
                end1 = i;
            }
        } else if (end1 != 0) {
            // If we're not on a space and end1 has been set, we've found the start of the second
            // id.
            start2 = i;
            break;
        }
    }
    return ids{
        .first = std.fmt.parseInt(u32, line[0..end1], 10) catch unreachable,
        .second = std.fmt.parseInt(u32, line[start2..], 10) catch unreachable,
    };
}
