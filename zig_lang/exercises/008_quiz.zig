const std = @import("std");
const print = std.debug.print;

pub fn main() void {
    const letters = "YZhifg";

    const x: usize = 1;

    var lang: [3]u8 = undefined;

    lang[0] = letters[x];

    print("letters : {s}\n", .{letters});
    print("Program in {s}!\n", .{lang});
}
