const std = @import("std");
const grid = @import("../utils/grid.zig");
const parsing = @import("../utils/parsing.zig");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;
const AutoHashMap = std.AutoHashMap;

pub fn partOne(allocator: Allocator, input: ArrayList([]u8)) ![]const u8 {
    var sum: usize = 0;

    var space_i: usize = 0;
    for (input.items) |line| {
        if (line.len == 0) break;
        space_i += 1;
    }

    const width = input.items[0].len;
    const height = space_i;

    const g = try grid.Grid(u8).init(allocator, height, width);
    defer g.deinit();
    var bot = grid.Coord{};
    for (0..height) |row| {
        for (0..width) |col| {
            g.inner[row][col] = input.items[row][col];

            if (g.inner[row][col] == '@') {
                bot.row = row;
                bot.col = col;
            } else if (g.inner[row][col] == 'O') {
                // We calculate the current GPS sum, and we'll update it opportunistically.
                sum += 100 * row + col;
            }
        }
    }

    for (space_i + 1..input.items.len) |i| {
        const line = input.items[i];
        for (line) |d| {
            const dir = switch (d) {
                '^' => grid.Dir.Up,
                '>' => grid.Dir.Right,
                'v' => grid.Dir.Down,
                '<' => grid.Dir.Left,
                else => std.debug.panic("Unexpected direction value: {c}\n", .{d}),
            };

            const dest = try g.getDir(bot, dir);
            if (dest == '#') continue;
            if (dest == '.') {
                try g.update(bot, '.');
                bot = try bot.translateDir(dir);
                try g.update(bot, '@');
                continue;
            }
            if (dest != 'O') std.debug.panic("Unexpected grid value: {c}\n", .{dest});

            // If we get here, we're hitting a box.
            var space = grid.Coord{
                .row = bot.row,
                .col = bot.col,
            };
            while (true) {
                const space_dest = try g.get(space);
                if (space_dest == '#') break; // Hit a wall, nothing to do.
                if (space_dest != '.') {
                    space = try space.translateDir(dir);
                    continue;
                }
                // Found an empty space this line of boxes can fill.  Fill the space with a box,
                // move the bot, then overwrite the box we ran into with the bot.
                try g.update(space, 'O');
                try g.update(bot, '.');
                bot = try bot.translateDir(dir);
                try g.update(bot, '@');
                // We also need to update the GPS sum.  Remove the GPS value of the bot (since
                // there is no longer a box there), and then add the GPS value of the space we
                // just filled.
                sum -= 100 * bot.row + bot.col;
                sum += 100 * space.row + space.col;
                break;
            }
        }
    }

    const solution = try std.fmt.allocPrint(allocator, "{d}", .{sum});
    return solution;
}

pub fn partTwo(allocator: Allocator, input: ArrayList([]u8)) ![]const u8 {
    var sum: usize = 0;

    var space_i: usize = 0;
    for (input.items) |line| {
        if (line.len == 0) break;
        space_i += 1;
    }

    const width = input.items[0].len * 2;
    const height = space_i;

    const g = try grid.Grid(u8).init(allocator, height, width);
    defer g.deinit();
    var bot = grid.Coord{};
    for (0..height) |row| {
        for (0..input.items[0].len) |col| {
            switch (input.items[row][col]) {
                '#' => {
                    g.inner[row][col * 2] = '#';
                    g.inner[row][col * 2 + 1] = '#';
                },
                'O' => {
                    g.inner[row][col * 2] = '[';
                    g.inner[row][col * 2 + 1] = ']';
                    sum += (100 * row) + (col * 2);
                },
                '.' => {
                    g.inner[row][col * 2] = '.';
                    g.inner[row][col * 2 + 1] = '.';
                },
                '@' => {
                    g.inner[row][col * 2] = '@';
                    g.inner[row][col * 2 + 1] = '.';
                    bot.row = row;
                    bot.col = col * 2;
                },
                else => std.debug.panic("Unexpected input grid value: {c}\n", .{input.items[row][col]}),
            }
        }
    }

    for (space_i + 1..input.items.len) |i| {
        const line = input.items[i];
        for (line) |d| {
            const dir = switch (d) {
                '^' => grid.Dir.Up,
                '>' => grid.Dir.Right,
                'v' => grid.Dir.Down,
                '<' => grid.Dir.Left,
                else => std.debug.panic("Unexpected direction value: {c}\n", .{d}),
            };

            const dest = try g.getDir(bot, dir);
            if (dest == '#') continue;
            if (dest == '.') {
                try g.update(bot, '.');
                bot = try bot.translateDir(dir);
                try g.update(bot, '@');
                continue;
            }
            if (dest != '[' and dest != ']') std.debug.panic("Unexpected grid value: {c}\n", .{dest});

            // If we get here, we're hitting a box.
            if (dir == grid.Dir.Left or dir == grid.Dir.Right) {
                // Simpler case; just shoving boxes along a single row.
                var space = grid.Coord{
                    .row = bot.row,
                    .col = bot.col,
                };
                while (true) {
                    var space_dest = try g.get(space);
                    if (space_dest == '#') break;
                    if (space_dest != '.') {
                        space = try space.translateDir(dir);
                        continue;
                    }
                    // Found an empty space this line of boxes can fill.  Start moving boxes.
                    const reverse = grid.flipDir(dir);
                    while (space_dest != '@') {
                        const rev_cursor = try space.translateDir(reverse);
                        space_dest = try g.get(rev_cursor);
                        try g.update(space, space_dest);
                        try g.update(rev_cursor, '.');
                        if (space_dest == '[') {
                            sum -= 100 * rev_cursor.row + rev_cursor.col;
                            sum += 100 * space.row + space.col;
                        }
                        space = rev_cursor;
                    }
                    bot = try bot.translateDir(dir);
                    break;
                }
                continue;
            }

            // Less simple; need to handle possibly overlapping boxes.
            var boxes = ArrayList(grid.Coord).init(allocator);
            defer boxes.deinit();
            var boxes_set = AutoHashMap(u128, grid.Coord).init(allocator);
            defer boxes_set.deinit();

            var space_l = try bot.translateDir(dir);
            var space_r = try bot.translateDir(dir);
            if (dest == '[') {
                space_r.col += 1;
            } else {
                space_l.col -= 1;
            }

            try boxes.append(space_l);
            try boxes_set.put(space_l.toKey(), space_l);

            var j: usize = 0;
            var cancel = false;
            while (boxes.items.len > j) {
                const box_l = boxes.items[j];
                const box_r = try box_l.translateDir(grid.Dir.Right);
                space_l = try box_l.translateDir(dir);
                space_r = try box_r.translateDir(dir);
                const dest_l = try g.get(space_l);
                const dest_r = try g.get(space_r);
                j += 1;

                if (dest_l == '#' or dest_r == '#') {
                    // If any of our stack of boxes will hit a wall, we can't push.
                    cancel = true;
                    break;
                }

                if (dest_l == '.' and dest_r == '.') {
                    // Fully empty space in front of the box, nothing left to do.
                    continue;
                }

                if (dest_l == '[') {
                    // Next box is aligned with this one, so only one box to add.
                    if (!boxes_set.contains(space_l.toKey())) {
                        try boxes.append(space_l);
                        try boxes_set.put(space_l.toKey(), space_l);
                    }

                    continue;
                }

                if (dest_l == ']') {
                    // Box overlapping with the left half of this one.  Add that box, but we still
                    // need to check what's in front of the right half of this box.
                    const next_box = try space_l.translateDir(grid.Dir.Left);
                    if (!boxes_set.contains(next_box.toKey())) {
                        try boxes.append(next_box);
                        try boxes_set.put(next_box.toKey(), space_l);
                    }
                }

                if (dest_r == '[') {
                    // Box overlapping with the right half of this one.  Add that box.
                    if (!boxes_set.contains(space_r.toKey())) {
                        try boxes.append(space_r);
                        try boxes_set.put(space_r.toKey(), space_r);
                    }
                }
            }

            if (cancel) continue;

            while (boxes.items.len > 0) {
                const box_l = boxes.pop();
                const box_r = try box_l.translateDir(grid.Dir.Right);
                const moved_l = try box_l.translateDir(dir);
                const moved_r = try box_r.translateDir(dir);
                try g.update(box_l, '.');
                try g.update(box_r, '.');
                try g.update(moved_l, '[');
                try g.update(moved_r, ']');
                sum -= 100 * box_l.row + box_l.col;
                sum += 100 * moved_l.row + moved_l.col;
            }

            try g.update(bot, '.');
            bot = try bot.translateDir(dir);
            try g.update(bot, '@');
        }
    }

    const solution = try std.fmt.allocPrint(allocator, "{d}", .{sum});
    return solution;
}
