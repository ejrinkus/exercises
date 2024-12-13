const std = @import("std");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;

pub fn partOne(allocator: Allocator, input: ArrayList([]u8)) ![]const u8 {
    const height = input.items.len;
    const width = input.items[0].len;

    var sum: u32 = 0;
    for (0..height) |row| {
        for (0..width) |col| {
            if (col <= width - 4) {
                if (input.items[row][col] == 'X' and
                    input.items[row][col + 1] == 'M' and
                    input.items[row][col + 2] == 'A' and
                    input.items[row][col + 3] == 'S')
                {
                    sum += 1;
                }
                if (input.items[row][col] == 'S' and
                    input.items[row][col + 1] == 'A' and
                    input.items[row][col + 2] == 'M' and
                    input.items[row][col + 3] == 'X')
                {
                    sum += 1;
                }
            }

            // Vertical
            if (row <= height - 4) {
                if (input.items[row][col] == 'X' and
                    input.items[row + 1][col] == 'M' and
                    input.items[row + 2][col] == 'A' and
                    input.items[row + 3][col] == 'S')
                {
                    sum += 1;
                }
                if (input.items[row][col] == 'S' and
                    input.items[row + 1][col] == 'A' and
                    input.items[row + 2][col] == 'M' and
                    input.items[row + 3][col] == 'X')
                {
                    sum += 1;
                }
            }

            // Diagonal forwards
            if (col <= width - 4 and row <= height - 4) {
                if (input.items[row][col] == 'X' and
                    input.items[row + 1][col + 1] == 'M' and
                    input.items[row + 2][col + 2] == 'A' and
                    input.items[row + 3][col + 3] == 'S')
                {
                    sum += 1;
                }
                if (input.items[row][col] == 'S' and
                    input.items[row + 1][col + 1] == 'A' and
                    input.items[row + 2][col + 2] == 'M' and
                    input.items[row + 3][col + 3] == 'X')
                {
                    sum += 1;
                }
            }

            // Diagonal backwards
            if (col >= 3 and row <= height - 4) {
                if (input.items[row][col] == 'X' and
                    input.items[row + 1][col - 1] == 'M' and
                    input.items[row + 2][col - 2] == 'A' and
                    input.items[row + 3][col - 3] == 'S')
                {
                    sum += 1;
                }
                if (input.items[row][col] == 'S' and
                    input.items[row + 1][col - 1] == 'A' and
                    input.items[row + 2][col - 2] == 'M' and
                    input.items[row + 3][col - 3] == 'X')
                {
                    sum += 1;
                }
            }
        }
    }

    return std.fmt.allocPrint(allocator, "{d}", .{sum});
}

pub fn partTwo(allocator: Allocator, input: ArrayList([]u8)) ![]const u8 {
    const height = input.items.len;
    const width = input.items[0].len;

    var sum: u32 = 0;
    for (1..height - 1) |row| {
        for (1..width - 1) |col| {
            const center = input.items[row][col];
            const ul = input.items[row - 1][col - 1];
            const ll = input.items[row + 1][col - 1];
            const ur = input.items[row - 1][col + 1];
            const lr = input.items[row + 1][col + 1];

            // Center of the X-MAS must be an A.
            if (center != 'A') continue;

            // The corners all need to be M or S.
            if (ul != 'M' and ul != 'S') continue;
            if (ll != 'M' and ll != 'S') continue;
            if (ur != 'M' and ur != 'S') continue;
            if (lr != 'M' and lr != 'S') continue;

            // Match the corners.
            if (ul == 'M' and lr != 'S') continue;
            if (ul == 'S' and lr != 'M') continue;
            if (ll == 'M' and ur != 'S') continue;
            if (ll == 'S' and ur != 'M') continue;

            // If we made it here, it's valid.
            sum += 1;
        }
    }

    return std.fmt.allocPrint(allocator, "{d}", .{sum});
}
