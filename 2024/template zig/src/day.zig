const std = @import("std");

pub fn day(file: std.fs.File) !i32 {
    const stdout = std.io.getStdOut().writer();

    var buf: [1024]u8 = undefined;

    var buf_reader = std.io.bufferedReader(file.reader());
    const in_stream = buf_reader.reader();

    while (try in_stream.readUntilDelimiterOrEof(&buf, '\n')) |line| {
        try stdout.print("{s}\n", .{line});
    }

    return -1;
}
