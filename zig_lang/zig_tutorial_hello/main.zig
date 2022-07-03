const std = @import("std");

pub fn main() void {
    var n: i64 = 50;
    n = n + 5;

    const pi: i64 = 314159;

    const negative_eleven: i8 = -11;

    std.debug.print(" (n=50) n + 5  = {}\n pi = {}\n  negative_eleven = {}\n", .{ n, pi, negative_eleven });
}
