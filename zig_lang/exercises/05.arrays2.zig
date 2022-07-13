const std = @import("std");
const print = std.debug.print;

pub fn main() void {
    const le = [_]u8{ 1, 3 };
    const et = [_]u8{ 3, 7 };

    const leet = le ++ et;

    print("LEET: {any}\n", .{leet});
}
