const std = @import("std");
// const fmt = @import("fmt");

pub fn main() void {
    const le = [_]u8{ 1, 3 };
    // const et = [_]u8{ 3, 7 };

    // const leet = le ++ et;

    std.debug.print("LEET: {}\n", .{le});
}
