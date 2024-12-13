const std = @import("std");
const runner = @import("runner.zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    // Create an unbuffered writer to stdout
    const stdout = std.io.getStdOut().writer();

    // Create an unbuffered reader from stdin
    const stdin = std.io.getStdIn().reader();

    // Prompt the user
    try stdout.print("Enter a day: ", .{});
    const day = try read_u8(stdin);
    try stdout.print("Enter a part: ", .{});
    const part = try read_u8(stdin);

    const solution = try runner.run(allocator, day, part);
    defer allocator.free(solution);

    try stdout.print("Solution: {s}\n", .{solution});
}

fn read_u8(reader: anytype) !u8 {
    var buffer: [100]u8 = undefined;
    const input = (try reader.readUntilDelimiterOrEof(&buffer, '\n')).?;
    return try std.fmt.parseInt(u8, input, 10);
}
