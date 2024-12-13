const std = @import("std");
const parsing = @import("../utils/parsing.zig");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;

pub fn partOne(allocator: Allocator, input: ArrayList([]u8)) ![]const u8 {
    var sum: u32 = 0;
    for (input.items) |line| {
        const report = try parsing.parseNums(allocator, u32, line, ' ');
        defer report.deinit();

        if (evaluate(report.items)) sum += 1;
    }

    return std.fmt.allocPrint(allocator, "{d}", .{sum});
}

pub fn partTwo(allocator: Allocator, input: ArrayList([]u8)) ![]const u8 {
    var sum: u32 = 0;
    for (input.items) |line| {
        const report = try parsing.parseNums(allocator, u32, line, ' ');
        defer report.deinit();

        for (0..report.items.len) |i| {
            if (evaluateWithSkip(report.items, i)) {
                sum += 1;
                break;
            }
        }
    }

    return std.fmt.allocPrint(allocator, "{d}", .{sum});
}

pub fn evaluate(report: []u32) bool {
    var prev = report[0];
    var asc: bool = false;
    var desc: bool = false;
    var safe = true;
    var diff: u32 = 0;
    for (1..report.len) |i| {
        const curr = report[i];

        if (prev < curr) {
            asc = true;
            diff = curr - prev;
        }

        if (prev > curr) {
            desc = true;
            diff = prev - curr;
        }

        if ((prev == curr) or (asc and desc) or (diff > 3)) {
            safe = false;
            break;
        }

        prev = curr;
    }

    return safe;
}

pub fn evaluateWithSkip(report: []u32, skip: usize) bool {
    var prev = report[0];
    var start: usize = 1;
    if (skip == 0) {
        prev = report[1];
        start = 2;
    }
    var asc: bool = false;
    var desc: bool = false;
    var safe = true;
    var diff: u32 = 0;
    for (start..report.len) |i| {
        if (i == skip) {
            continue;
        }
        const curr = report[i];

        if (prev < curr) {
            asc = true;
            diff = curr - prev;
        }

        if (prev > curr) {
            desc = true;
            diff = prev - curr;
        }

        if ((prev == curr) or (asc and desc) or (diff > 3)) {
            safe = false;
            break;
        }

        prev = curr;
    }

    return safe;
}
