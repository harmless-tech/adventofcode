const std = @import("std");

const ZERO: u8 = 48;
const NINE: u8 = 57;

pub fn main() anyerror!void {
    std.debug.print("Day 1 - Part 1\n", .{});

    const file = try std.fs.cwd().openFile("../_input/day1.txt", .{});
    defer file.close();

    var buf_reader = std.io.bufferedReader(file.reader());
    const in = buf_reader.reader();

    var total: u32 = 0;

    var buffer: [1024]u8 = undefined;
    while (try in.readUntilDelimiterOrEof(&buffer, '\n')) |line| {
        total += get(line);
    }

    std.debug.print("Total: {}\n", .{total});
}

fn get(str: []u8) u32 {
    var first: ?u32 = null;
    var last: ?u32 = null;

    for(str) |c| {
        if(c >= ZERO and c <= NINE) {
            if(first == null) {
                first = c - ZERO;
            }
            else {
                last = c - ZERO;
            }
        }
    }

    if(last == null) {
        last = first;
    }

    return (first.? * 10) + last.?;
}
