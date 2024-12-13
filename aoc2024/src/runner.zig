const std = @import("std");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;

const day1 = @import("days/day1.zig");
const day2 = @import("days/day2.zig");
const day3 = @import("days/day3.zig");
const day4 = @import("days/day4.zig");
const day5 = @import("days/day5.zig");
const day6 = @import("days/day6.zig");
const day7 = @import("days/day7.zig");
const day8 = @import("days/day8.zig");
const day9 = @import("days/day9.zig");
const day10 = @import("days/day10.zig");
const day11 = @import("days/day11.zig");
const day12 = @import("days/day12.zig");
const day13 = @import("days/day13.zig");
const day14 = @import("days/day14.zig");
const day15 = @import("days/day15.zig");
const day16 = @import("days/day16.zig");
const day17 = @import("days/day17.zig");
const day18 = @import("days/day18.zig");
const day19 = @import("days/day19.zig");
const day20 = @import("days/day20.zig");
const day21 = @import("days/day21.zig");
const day22 = @import("days/day22.zig");
const day23 = @import("days/day23.zig");
const day24 = @import("days/day24.zig");
const day25 = @import("days/day25.zig");
const daynil = @import("days/daynil.zig");

const session = @import("utils/session.zig");

const RunError = error{
    FetchError,
};

pub fn readInputFile(allocator: Allocator, subpath: []const u8) !ArrayList([]u8) {
    const file = try std.fs.cwd().openFile(subpath, .{});
    defer file.close();

    var buf_reader = std.io.bufferedReader(file.reader());
    const reader = buf_reader.reader();

    var line = ArrayList(u8).init(allocator);
    defer line.deinit();
    const writer = line.writer();

    var lines = ArrayList([]u8).init(allocator);

    while (reader.streamUntilDelimiter(writer, '\n', null)) {
        defer line.clearRetainingCapacity();

        try lines.append(try line.toOwnedSlice());
    } else |err| switch (err) {
        error.EndOfStream => {},
        else => return err,
    }

    return lines;
}

pub fn fetchInput(allocator: Allocator, day: u32, subpath: []const u8) !ArrayList([]u8) {
    var client = std.http.Client{ .allocator = allocator };
    defer client.deinit();

    var headerbuf: [4096]u8 = undefined;
    var pathbuf: [64]u8 = undefined;

    const path = try std.fmt.bufPrint(&pathbuf, "https://adventofcode.com/2024/day/{d}/input", .{day});

    const uri = try std.Uri.parse(path);
    var req = try client.open(.GET, uri, .{ .server_header_buffer = &headerbuf });
    req.extra_headers = &[1]std.http.Header{std.http.Header{ .name = "cookie", .value = session.cookie }};
    defer req.deinit();

    // Sends headers
    try req.send();
    // Finishes the body
    try req.finish();
    // Waits for a response
    try req.wait();

    const resp = req.response;
    if (resp.status != std.http.Status.ok) {
        std.debug.print("HTTP status: {any}\t HTTP Reason: {s}\n", .{ resp.status, resp.reason });
        std.debug.print("HTTP headers: {s}\n", .{resp.parser.get()});

        return RunError.FetchError;
    }

    var line = ArrayList(u8).init(allocator);
    defer line.deinit();
    const writer = line.writer();

    var reader = req.reader();
    var lines = ArrayList([]u8).init(allocator);

    const file = try std.fs.cwd().createFile(subpath, .{});
    defer file.close();

    while (reader.streamUntilDelimiter(writer, '\n', null)) {
        defer line.clearRetainingCapacity();

        const slice = try line.toOwnedSlice();
        try file.writeAll(slice);
        try file.writeAll("\n");

        try lines.append(slice);
    } else |err| switch (err) {
        error.EndOfStream => {},
        else => return err,
    }

    return lines;
}

pub fn run(allocator: Allocator, day: u8, part: u8) ![]const u8 {
    var buf: [32]u8 = undefined;
    const path = try std.fmt.bufPrint(&buf, "data/day{d}.txt", .{day});

    const input = readInputFile(allocator, path) catch try fetchInput(allocator, day, path);
    defer {
        for (input.items) |slice| {
            allocator.free(slice);
        }
        input.deinit();
    }

    switch (day) {
        1 => {
            if (part == 1) return day1.partOne(allocator, input) else if (part == 2) return day1.partTwo(allocator, input) else return "invalid part";
        },
        2 => {
            if (part == 1) return day2.partOne(allocator, input) else if (part == 2) return day2.partTwo(allocator, input) else return "invalid part";
        },
        3 => {
            if (part == 1) return day3.partOne(allocator, input) else if (part == 2) return day3.partTwo(allocator, input) else return "invalid part";
        },
        4 => {
            if (part == 1) return day4.partOne(allocator, input) else if (part == 2) return day4.partTwo(allocator, input) else return "invalid part";
        },
        5 => {
            if (part == 1) return day5.partOne(allocator, input) else if (part == 2) return day5.partTwo(allocator, input) else return "invalid part";
        },
        6 => {
            if (part == 1) return day6.partOne(allocator, input) else if (part == 2) return day6.partTwo(allocator, input) else return "invalid part";
        },
        7 => {
            if (part == 1) return day7.partOne(allocator, input) else if (part == 2) return day7.partTwo(allocator, input) else return "invalid part";
        },
        8 => {
            if (part == 1) return day8.partOne(allocator, input) else if (part == 2) return day8.partTwo(allocator, input) else return "invalid part";
        },
        9 => {
            if (part == 1) return day9.partOne(allocator, input) else if (part == 2) return day9.partTwo(allocator, input) else return "invalid part";
        },
        10 => {
            if (part == 1) return day10.partOne(allocator, input) else if (part == 2) return day10.partTwo(allocator, input) else return "invalid part";
        },
        11 => {
            if (part == 1) return day11.partOne(allocator, input) else if (part == 2) return day11.partTwo(allocator, input) else return "invalid part";
        },
        12 => {
            if (part == 1) return day12.partOne(allocator, input) else if (part == 2) return day12.partTwo(allocator, input) else return "invalid part";
        },
        13 => {
            if (part == 1) return day13.partOne(allocator, input) else if (part == 2) return day13.partTwo(allocator, input) else return "invalid part";
        },
        14 => {
            if (part == 1) return day14.partOne(allocator, input) else if (part == 2) return day14.partTwo(allocator, input) else return "invalid part";
        },
        15 => {
            if (part == 1) return day15.partOne(allocator, input) else if (part == 2) return day15.partTwo(allocator, input) else return "invalid part";
        },
        16 => {
            if (part == 1) return day16.partOne(allocator, input) else if (part == 2) return day16.partTwo(allocator, input) else return "invalid part";
        },
        17 => {
            if (part == 1) return day17.partOne(allocator, input) else if (part == 2) return day17.partTwo(allocator, input) else return "invalid part";
        },
        18 => {
            if (part == 1) return day18.partOne(allocator, input) else if (part == 2) return day18.partTwo(allocator, input) else return "invalid part";
        },
        19 => {
            if (part == 1) return day19.partOne(allocator, input) else if (part == 2) return day19.partTwo(allocator, input) else return "invalid part";
        },
        20 => {
            if (part == 1) return day20.partOne(allocator, input) else if (part == 2) return day20.partTwo(allocator, input) else return "invalid part";
        },
        21 => {
            if (part == 1) return day21.partOne(allocator, input) else if (part == 2) return day21.partTwo(allocator, input) else return "invalid part";
        },
        22 => {
            if (part == 1) return day22.partOne(allocator, input) else if (part == 2) return day22.partTwo(allocator, input) else return "invalid part";
        },
        23 => {
            if (part == 1) return day23.partOne(allocator, input) else if (part == 2) return day23.partTwo(allocator, input) else return "invalid part";
        },
        24 => {
            if (part == 1) return day24.partOne(allocator, input) else if (part == 2) return day24.partTwo(allocator, input) else return "invalid part";
        },
        25 => {
            if (part == 1) return day25.partOne(allocator, input) else if (part == 2) return day25.partTwo(allocator, input) else return "invalid part";
        },
        else => {
            if (part == 1) return daynil.partOne(allocator, input) else if (part == 2) return daynil.partTwo(allocator, input) else return "invalid part";
        },
    }
}
