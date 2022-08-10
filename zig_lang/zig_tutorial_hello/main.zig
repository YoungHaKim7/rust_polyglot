const std = @import("std");
const print = std.debug.print;

pub fn main() void {
    const lyrics =
        \\ Zigggy played guitar
        \\ Jamming good with Andrew Kelley
        \\ And the Spiders from Mars
    ;

    print("{s}\n", .{lyrics});
}
