const std = @import("std");
const day = @import("day.zig");

pub fn main() !void {
    const stdout = std.io.getStdOut().writer();

    var file = try std.fs.cwd().openFile("input.txt", .{});
    defer file.close();

    try stdout.print("Solution: {any}\n", .{day.day(file)});
}
