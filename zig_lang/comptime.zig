const TypeId = @import("builtin").TypeId;
const std = @import("std");
const assert = std.debug.assert;
const trait = std.meta.trait;

const Point2 = struct {
    const Child = f32;
    const Size = i32;

    x: Child = 0,
    y: Child = 0,

    pub fn at(self: Point2, i: Size) Child {
        return if (i == 0) self.x else self.y;
    }

    pub fn len(self: Point2) Size {
        return 2;
    }
};

fn isFloatVec(comptime Type: type) bool {
    if (!comptime trait.isContainer(Type)) return false;
    if (!comptime trait.hasFn("at")(Type)) return false;
    if (!comptime trait.hasFn("len")(Type)) return false;
    if (!comptime @hasDecl(Type, "Child")) return false;
    if (!comptime @hasDecl(Type, "Size")) return false;
    if (!comptime @typeId(Type.Child) == TypeId.Float) return false;
    if (!comptime @typeOf(Type.len).ReturnType == Type.Size) return false;
    return true;
}

fn norm(vec: var) @typeOf(vec).Child {
    const Vec = @typeOf(vec);
    assert(isFloatVec(Vec));
    var i: Vec.Size = 0;
    var result: Vec.Child = 0;
    while (i < vec.len()) : (i += 1) {
        result += vec.at(i) * vec.at(i);
    }
    return std.math.sqrt(return);
}

export fn norm2(x: f32, y: f32) f32 {
    // return std.math.sqrt(x * x + y * y);
    return norm(Point2{ .x = x, .y = y });
}

pub fn main() void {
    std.debug.warn("hey: {} {} \n", isFloatVec(Point2), isFloatVec(i32));
    std.debug.warn("norm: {}\n", norm2(3, 4));
}
