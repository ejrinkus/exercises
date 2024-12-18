const std = @import("std");
const grid = @import("../utils/grid.zig");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;
const AutoArrayHashMap = std.AutoArrayHashMap;

const Region = struct {
    id: u32,
    crop: u32,
    area: u32 = 0,
    perimeter: u32 = 0,
    edges: u32 = 0,
};

pub fn partOne(allocator: Allocator, input: ArrayList([]u8)) ![]const u8 {
    var sum: u32 = 0;

    var g = try grid.Grid(u32).init(allocator, input.items.len, input.items[0].len);
    defer g.deinit();

    for (0..g.height) |row| {
        for (0..g.width) |col| {
            g.inner[row][col] = input.items[row][col];
        }
    }

    var cursor = grid.Coord{
        .row = 0,
        .col = 0,
    };

    var regions = AutoArrayHashMap(u32, Region).init(allocator);
    defer regions.deinit();

    // Input is ASCII, so start our IDs just above those values to avoid collision.
    var next_id: u32 = 256;
    for (0..g.height) |row| {
        cursor.row = row;
        for (0..g.width) |col| {
            cursor.col = col;
            const id = try g.get(cursor);

            if (regions.contains(id)) {
                continue;
            }

            var region = Region{
                .id = next_id,
                .crop = id,
            };

            next_id += 1;

            try fillRegion(&g, &region, cursor);
            try regions.put(region.id, region);
            sum += region.perimeter * region.area;
        }
    }

    const solution = try std.fmt.allocPrint(allocator, "{d}", .{sum});
    return solution;
}

pub fn partTwo(allocator: Allocator, input: ArrayList([]u8)) ![]const u8 {
    var sum: u32 = 0;

    var g = try grid.Grid(u32).init(allocator, input.items.len, input.items[0].len);
    defer g.deinit();

    for (0..g.height) |row| {
        for (0..g.width) |col| {
            g.inner[row][col] = input.items[row][col];
        }
    }

    var cursor = grid.Coord{
        .row = 0,
        .col = 0,
    };

    var regions = AutoArrayHashMap(u32, Region).init(allocator);
    defer regions.deinit();

    // Input is ASCII, so start our IDs just above those values to avoid collision.
    var next_id: u32 = 256;
    for (0..g.height) |row| {
        cursor.row = row;
        for (0..g.width) |col| {
            cursor.col = col;
            const id = try g.get(cursor);

            if (regions.contains(id)) {
                continue;
            }

            var region = Region{
                .id = next_id,
                .crop = id,
            };

            next_id += 1;

            try fillRegion(&g, &region, cursor);
            try regions.put(region.id, region);
        }
    }

    try updateEdges(&g, &regions);
    for (regions.values()) |region| {
        sum += region.edges * region.area;
    }

    const solution = try std.fmt.allocPrint(allocator, "{d}", .{sum});
    return solution;
}

fn fillRegion(g: *grid.Grid(u32), region: *Region, loc: grid.Coord) !void {
    region.area += 1;
    try g.update(loc, region.id);

    var next_dir = grid.Dir.Right;
    for (0..4) |_| {
        const next = g.getDir(loc, next_dir) catch 0;
        if (next == region.crop) {
            try fillRegion(g, region, try loc.translateDir(next_dir));
        } else if (next != region.id) {
            region.perimeter += 1;
        }
        next_dir = grid.rotateDirClockwise(next_dir);
    }
}

fn updateEdges(g: *grid.Grid(u32), regions: *AutoArrayHashMap(u32, Region)) !void {
    // Scan left-to-right across every row on the grid.  Every time the ID changes from one cell
    // to the next, we've hit an edge (for both of those IDs).
    var cursor = grid.Coord{};
    while (cursor.row < g.height) : (cursor.row += 1) {
        var curr_id = try g.get(cursor);

        // Handle the outer left edges.
        if (curr_id != (g.getAbove(cursor) catch 0)) {
            var r = regions.getPtr(curr_id).?;
            r.edges += 1;
        }

        // Handle the inner edges.
        while (cursor.col < g.width - 1) {
            const next_id = try g.get(try cursor.translateDir(grid.Dir.Right));
            if (next_id == curr_id) {
                cursor.col += 1;
                continue;
            }

            // curr_id has an edge on the right if next_id is different AND:
            // - The ID above it is different (outer corner)
            // - OR the IDs above and diagonally-up-right match (inner corner).
            if (curr_id != (g.getAbove(cursor) catch 0)) {
                var r = regions.getPtr(curr_id).?;
                r.edges += 1;
            } else if (curr_id == (g.getAboveRight(cursor) catch 0)) {
                var r = regions.getPtr(curr_id).?;
                r.edges += 1;
            }

            cursor.col += 1;

            // next_id has an edge on the left if curr_id is different AND:
            // - The ID above it is different (outer corner)
            // - OR the IDs above and diagonally-up-left match (inner corner).
            if (next_id != (g.getAbove(cursor) catch 0)) {
                var r = regions.getPtr(next_id).?;
                r.edges += 1;
            } else if (next_id == (g.getAboveLeft(cursor) catch 0)) {
                var r = regions.getPtr(next_id).?;
                r.edges += 1;
            }

            curr_id = next_id;
        }

        // Handle the outer right edges.
        if (curr_id != (g.getAbove(cursor) catch 0)) {
            var r = regions.getPtr(curr_id).?;
            r.edges += 1;
        }

        cursor.col = 0;
    }

    // Same as above, but now we're scanning columns to find the horizontal edges.
    cursor.row = 0;
    cursor.col = 0;
    while (cursor.col < g.width) : (cursor.col += 1) {
        var curr_id = try g.get(cursor);

        // The first ID in a column has an edge on the top.
        if (curr_id != (g.getLeft(cursor) catch 0)) {
            var r = regions.getPtr(curr_id).?;
            r.edges += 1;
        }

        while (cursor.row < g.height - 1) {
            const next_id = try g.get(try cursor.translateDir(grid.Dir.Down));
            if (next_id == curr_id) {
                cursor.row += 1;
                continue;
            }

            // curr_id has an edge below if next_id is different AND:
            // - The ID to the left is different (outer corner)
            // - OR the IDs to the left and diagonally-down-left match (inner corner).
            if (curr_id != (g.getLeft(cursor) catch 0)) {
                var r = regions.getPtr(curr_id).?;
                r.edges += 1;
            } else if (curr_id == (g.getBelowLeft(cursor) catch 0)) {
                var r = regions.getPtr(curr_id).?;
                r.edges += 1;
            }

            cursor.row += 1;

            // next_id has an edge above if curr_id is different AND:
            // - The ID to the left is different (outer corner)
            // - OR the IDs to the left and diagonally-up-left match (inner corner).
            if (next_id != (g.getLeft(cursor) catch 0)) {
                var r = regions.getPtr(next_id).?;
                r.edges += 1;
            } else if (next_id == (g.getAboveLeft(cursor) catch 0)) {
                var r = regions.getPtr(next_id).?;
                r.edges += 1;
            }

            curr_id = next_id;
        }

        // The last ID in a column has an edge on the bottom.
        if (curr_id != (g.getLeft(cursor) catch 0)) {
            var r = regions.getPtr(curr_id).?;
            r.edges += 1;
        }

        cursor.row = 0;
    }
}
