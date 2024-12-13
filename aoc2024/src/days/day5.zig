const std = @import("std");
const parsing = @import("../utils/parsing.zig");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;
const AutoHashMap = std.AutoHashMap;

const RuleMap = AutoHashMap(u32, ArrayList(u32));
const SeenMap = AutoHashMap(u32, usize);

pub fn partOne(allocator: Allocator, input: ArrayList([]u8)) ![]const u8 {
    var sum: u32 = 0;

    // Rules map: value (lhs) -> values that must not precede it (rhs).
    var rules = RuleMap.init(allocator);
    defer {
        // Since each value is an array list, we need to free them individually.
        var iterator = rules.valueIterator();
        while (iterator.next()) |value| {
            value.deinit();
        }
        rules.deinit();
    }

    // Loop over all the rules and store them (stop once we hit a line break);
    var i: usize = 0;
    while (!std.mem.allEqual(u8, input.items[i], '\n')) {
        const line = input.items[i];
        i += 1;

        try processRule(allocator, line, &rules);
    }

    // Skip the line break;
    i += 1;

    // Validate the updates.
    while (i < input.items.len) {
        const line = input.items[i];
        i += 1;

        const update = try parsing.parseNums(allocator, u32, line, ',');
        defer update.deinit();

        sum += try validateUpdate(allocator, update.items, rules);
    }

    return try std.fmt.allocPrint(allocator, "{d}", .{sum});
}

pub fn partTwo(allocator: Allocator, input: ArrayList([]u8)) ![]const u8 {
    var sum: u32 = 0;

    // Rules map: value (lhs) -> values that must not precede it (rhs).
    var rules = RuleMap.init(allocator);
    defer {
        // Since each value is an array list, we need to free them individually.
        var iterator = rules.valueIterator();
        while (iterator.next()) |value| {
            value.deinit();
        }
        rules.deinit();
    }

    // Loop over all the rules and store them (stop once we hit a line break);
    var i: usize = 0;
    while (!std.mem.allEqual(u8, input.items[i], '\n')) {
        const line = input.items[i];
        i += 1;

        try processRule(allocator, line, &rules);
    }

    // Skip the line break;
    i += 1;

    // Validate the updates.
    while (i < input.items.len) {
        const line = input.items[i];
        i += 1;

        const update = try parsing.parseNums(allocator, u32, line, ',');
        defer update.deinit();

        sum += try fixUpdate(allocator, update.items, rules);
    }

    return try std.fmt.allocPrint(allocator, "{d}", .{sum});
}

fn processRule(allocator: Allocator, line: []u8, rules: *RuleMap) !void {
    // Find the pipe that separates the two numbers.
    var pipe_idx: usize = undefined;
    for (line, 0..line.len) |val, j| {
        if (val == '|') {
            pipe_idx = j;
            break;
        }
    }

    // Extract the two numbers
    const lhs = try std.fmt.parseInt(u32, line[0..pipe_idx], 10);
    const rhs = try std.fmt.parseInt(u32, line[pipe_idx + 1 ..], 10);

    // Get list associated with lhs, or make a new one if this is the first rule for it.
    var list = rules.get(lhs) orelse ArrayList(u32).init(allocator);
    // Add the rhs to the list of followers.
    try list.append(rhs);
    // Replace the list.
    try rules.put(lhs, list);
}

fn validateUpdate(allocator: Allocator, update: []u32, rules: RuleMap) !u32 {
    var seen = SeenMap.init(allocator);
    defer seen.deinit();

    for (update, 0..) |val, i| {
        // If this val didn't appear on the lhs of a rule, we can skip it.  It doesn't matter what
        // preceded this value (but it may matter what comes after it since it could still have
        // been an rhs value).
        //
        // It's invalid for a rule to require that a value follow itself, so it should be safe to
        // add this value to the seen set now.
        try seen.put(val, i);
        if (!rules.contains(val)) continue;

        const followers = rules.get(val) orelse unreachable;
        for (followers.items) |f| {
            if (seen.contains(f)) {
                return 0;
            }
        }
    }

    const middle = (update.len - 1) / 2;
    return update[middle];
}

fn fixUpdate(allocator: Allocator, update: []u32, rules: RuleMap) !u32 {
    var seen = SeenMap.init(allocator);
    defer seen.deinit();

    // Preprocess the list to populate our seen map.
    for (update, 0..) |val, i| {
        try seen.put(val, i);
    }

    // Validate the update, fixing any errors as we go.  If nothing was fixed, we can return 0
    // since this update shouldn't be counted in the sum.
    //
    // STRATEGY:
    // Given VAL at index I and F at index J, where VAL follows F (i.e. I > J).
    // If F is in VAL's followers list, then VAL is out-of-place.
    // Swap VAL and F, updating their indices in the seen map accordingly.
    //
    // Everything prior to J should still be in the correct place, since the numbers preceding and
    // following them haven't actually changed (we've only changed the _order_ of their followers,
    // which doesn't matter).
    //
    // Additionally, VAL and F are now corrected with respect to each other.  It's possible that VAL
    // is still out-of-order relative to other values in its followers list that haven't been
    // checked yet, though, so we should resume fixing from index J so that we check VAL again.  This
    // also implies that we will later fix F when we get back to that spot in the update, in case
    // it was pushed too far forward.
    //
    // This strategy relies on there being no cycles in the rules.  However, a cyclical ruleset
    // means that we can't guarantee fixes to all broken updates, which is invalid for this problem.
    var i: usize = 0;
    var fixed = false;
    OUTER: while (i < update.len) {
        const val = update[i];

        // If this val didn't appear on the lhs of a rule, we can skip it.  It doesn't matter what
        // preceded this value (but it may matter what comes after it since it could still have
        // been an rhs value).
        if (!rules.contains(val)) {
            i += 1;
            continue;
        }

        const followers = rules.get(val) orelse unreachable;
        for (followers.items) |f| {
            if (!seen.contains(f)) continue; // This follower didn't appear in the update.

            const j = seen.get(f) orelse unreachable;
            if (i < j) continue; // f already follows val.

            // Swap the elements.
            update[i] = f;
            update[j] = val;

            // Update the seen map.
            try seen.put(f, i);
            try seen.put(val, j);
            fixed = true;

            // Resume from j.
            i = j;
            continue :OUTER;
        }

        i += 1;
    }

    if (!fixed) return 0;

    const middle = (update.len - 1) / 2;
    return update[middle];
}
