const std = @import("std");
const math = @import("../utils/math.zig");
const parsing = @import("../utils/parsing.zig");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;

pub fn partOne(allocator: Allocator, input: ArrayList([]u8)) ![]const u8 {
    var sum: u128 = 0;
    const disk = input.items[0];

    var block: u128 = 0;
    var left: usize = 0;
    var right: usize = disk.len - 1;
    while (left <= right) {
        const length = parsing.charToDigit(disk[left]).?;

        if (left % 2 == 0) {
            // File block.
            const id = left / 2;
            for (0..length) |_| {
                sum += block * id;
                block += 1;
            }
        } else {
            // Empty block, fill in from the right side of the disk.
            var id = right / 2;
            var rem = parsing.charToDigit(disk[right]).?;
            for (0..length) |_| {
                sum += block * id;
                block += 1;
                rem -= 1;
                while (rem == 0 and right >= left) {
                    disk[right] = '0';
                    disk[right - 1] = '0';
                    right -= 2; // Make sure to skip the empty block.
                    id = right / 2;
                    rem = parsing.charToDigit(disk[right]).?;
                }
                if (right < left) {
                    break;
                }
            }
            disk[right] = rem + '0';
        }

        left += 1;
    }

    return try std.fmt.allocPrint(allocator, "{d}", .{sum});
}

pub fn partTwo(allocator: Allocator, input: ArrayList([]u8)) ![]const u8 {
    var sum: u128 = 0;
    const disk = input.items[0];
    const spaces = try allocator.alloc(u8, disk.len);
    defer allocator.free(spaces);

    // spaces contains all of the free blocks in disk.  Initially, it's sort of a copy of disk.
    // All the indices that contain a file (i.e. the even ones) are '0' since they are full, and
    // all the empty blocks (i.e. the even indices) just get copied in since they haven't been
    // filled yet.
    for (disk, 0..) |val, i| {
        if (i % 2 == 0) {
            spaces[i] = '0';
        } else {
            spaces[i] = val;
        }
    }

    // Loop over the files from the end of the disk.
    // For each one, we try and find the left-most gap it'll fit in.  If we don't find one before
    // getting back to where the file is already located, then we leave it as is.  We use the
    // spaces array to figure out where there are open spaces, and to keep track of the spaces
    // we've filled, without mangling the disk array (we need it to keep track of block counts).
    //
    // Eventually there will be no empty blocks to the left of our remaining files.  This algo will
    // still handle these files correctly, but with some unnecessary looping to look for spaces that
    // don't exist.  We could use memoization to optimize this, but probably not worth it.
    var right: usize = disk.len - 1;
    while (right >= 0) : (right -= 2) {
        const id = right / 2;
        const file_len = parsing.charToDigit(disk[right]).?;

        // Search for a suitable gap.
        var block: u128 = 0;
        var left: usize = 1;
        var found = false;
        while (left < right) : (left += 2) {
            const gap_len = parsing.charToDigit(spaces[left]).?;

            block += parsing.charToDigit(disk[left - 1]).?;
            if (gap_len < file_len) {
                block += parsing.charToDigit(disk[left]).?;
                continue;
            }

            if (spaces[left] != disk[left]) {
                // Something else has partially occupied this spot already; nudge block to be the
                // correct value.
                block += disk[left] - spaces[left];
            }

            // Found one.  Update our checksum and continue the outer loop.
            for (0..file_len) |_| {
                sum += block * id;
                block += 1;
            }
            spaces[left] -= file_len;
            found = true;

            break;
        }

        // We didn't find a gap, so update the checksum by leaving the file where it is.
        if (!found) {
            for (0..file_len) |_| {
                sum += block * id;
                block += 1;
            }
        }

        if (right == 0) break;
    }

    return try std.fmt.allocPrint(allocator, "{d}", .{sum});
}
