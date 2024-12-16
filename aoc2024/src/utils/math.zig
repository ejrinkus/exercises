const std = @import("std");

// Calculates the sum from n to m (inclusive);
pub fn sumRange(comptime T: type, n: T, m: T) T {
    const upper_sum = (m * (m + 1)) / 2;
    const lower_sum = ((n - 1) * n) / 2;
    return upper_sum - lower_sum;
}
