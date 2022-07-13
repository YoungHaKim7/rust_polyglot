// zig array

const std = @import("std");
const print = std.debug.print;

pub fn main() void {
    const le = [_]u8{ 1, 3 };
    const et = [_]u8{ 3, 7 };

    const leet = le ++ et;

    // It should result in: 1 0 0 1 1 0 0 1 1 0 0 1
    const my_array = [_]u8{ 1, 0, 0, 1 } ** 3;
    print("LEET : {any}\n", .{leet});

    print("my_array : {any}\n", .{my_array});

    // for_each
    for (leet) |n| {
        print("{}\n", .{n});
    }

    for (my_array) |n| {
        print("{}", .{n});
    }
}
