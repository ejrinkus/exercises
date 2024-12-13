const std = @import("std");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;

pub fn partOne(allocator: Allocator, input: ArrayList([]u8)) ![]const u8 {
    const sum = input.items.len;

    const solution = try std.fmt.allocPrint(allocator, "{d}", .{sum});
    return solution;
}

pub fn partTwo(allocator: Allocator, input: ArrayList([]u8)) ![]const u8 {
    const sum = input.items.len;

    const solution = try std.fmt.allocPrint(allocator, "{d}", .{sum});
    return solution;
}
