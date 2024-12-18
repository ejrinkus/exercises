const std = @import("std");
const grid = @import("../utils/grid.zig");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;
const AutoArrayHashMap = std.AutoArrayHashMap;

const Region = struct {
    id: u16,
    fill_id: u16,
    area: u32 = 0,
    perimeter: u32 = 0,
    edges: u32 = 0,
};

pub fn partOne(allocator: Allocator, input: ArrayList([]u8)) ![]const u8 {
    var sum: u32 = 0;

    var g = try grid.Grid(u16).init(allocator, input.items.len, input.items[0].len);
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

    var regions = AutoArrayHashMap(u16, Region).init(allocator);
    defer regions.deinit();

    // Input is ASCII, so start our IDs just above those values to avoid collision.
    var next_fill_id: u16 = 256;
    for (0..g.height) |row| {
        cursor.row = row;
        for (0..g.width) |col| {
            cursor.col = col;
            const id = try g.get(cursor);

            if (regions.contains(id)) {
                continue;
            }

            var region = Region{
                .id = id,
                .fill_id = next_fill_id,
            };

            next_fill_id += 1;

            try fillRegion(&g, &region, cursor);
            try regions.put(region.fill_id, region);
            sum += region.perimeter * region.area;
        }
    }

    const solution = try std.fmt.allocPrint(allocator, "{d}", .{sum});
    return solution;
}

pub fn partTwo(allocator: Allocator, input: ArrayList([]u8)) ![]const u8 {
    var sum: u32 = 0;

    var g = try grid.Grid(u16).init(allocator, input.items.len, input.items[0].len);
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

    var regions = AutoArrayHashMap(u16, Region).init(allocator);
    defer regions.deinit();

    var next_fill_id: u16 = 1;
    for (0..g.height) |row| {
        cursor.row = row;
        for (0..g.width) |col| {
            cursor.col = col;
            const id = try g.get(cursor);

            if (regions.contains(id)) {
                continue;
            }

            var region = Region{
                .id = id,
                .fill_id = next_fill_id,
            };

            next_fill_id += 1;

            try fillRegion(&g, &region, cursor);
            try regions.put(region.fill_id, region);
            sum += region.perimeter * region.area;
        }
    }

    const solution = try std.fmt.allocPrint(allocator, "{d}", .{sum});
    return solution;
}

fn fillRegion(g: *grid.Grid(u16), region: *Region, loc: grid.Coord) !void {
    region.area += 1;
    try g.update(loc, region.fill_id);

    var next_dir = grid.Dir.Right;
    for (0..4) |_| {
        const next = g.getDir(loc, next_dir) catch 0;
        if (next == region.id) {
            try fillRegion(g, region, try loc.translateDir(next_dir));
        } else if (next != region.fill_id) {
            region.perimeter += 1;
        }
        next_dir = grid.rotateDirClockwise(next_dir);
    }
}

fn traceRegion(g: *grid.Grid(u16), region: *Region, start: grid.Coord) !void {
    // Trace the perimeter of a region.  Given how the grid gets traversed, we'll always start in
    // an upper-left corner position (maybe not _the_ upper left, but it will be a space that
    // minimally has fencing on the top and left).
    //
    // If we imagine walking the trace, we will walk it keeping the current edge on our left.  If
    // there is ever no longer an edge in that direction, then we need to turn left.  Or, if we can
    // no longer keep moving in the same direction we've been moving in, we need to turn right.
    //
    // In the latter case, we increment our edges count before checking to see if we can move after
    // the turn.  This is repeated until we are able to move again (e.g. if we hit a dead end, we'll
    // turn twice and increment the edges count twice to represent the 2 new edges that the current
    // space introduced).
    //
    // We stop once we've returned to the location we started at.  Given the above, this means that
    // edge_dir should be Left.  If it isn't and we're at the starting position, then we may need
    // to rotate in-place one to three more times to ensure we count all the edges before we're
    // done.
    var loc = grid.Coord{
        .row = start.row,
        .col = start.col,
    };
    var edge_dir = grid.Dir.Up;
    while (loc.row != start.row or loc.col != start.col or edge_dir != grid.Dir.Left) {
        const left_turn = g.getDir(loc, edge_dir) catch 0;
        if (left_turn == region.fill_id) {
            // No more edge on our left, turn left.
            region.edges += 1;
            loc = try loc.translateDir(edge_dir);
            edge_dir = grid.rotateDirAnticlockwise(edge_dir);
            continue;
        }

        const forward_dir = grid.rotateDirClockwise(edge_dir);
        const forward = g.getDir(loc, forward_dir) catch 0;
        if (forward == region.fill_id) {
            // Continue forward, no new edge.
            loc = try loc.translateDir(forward_dir);
            continue;
        }

        // Don't actually move yet, in case we need to immediately turn again.
        edge_dir = forward_dir;
        region.edges += 1;
    }
}
