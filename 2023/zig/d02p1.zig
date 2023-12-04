const std = @import("std");
const allocator = std.heap.page_allocator;

const ZERO: u8 = 48;

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

const Game = struct { id: u32, subsets: std.ArrayList(SubSet) };
const SubSet = struct { red: u32, green: u32, blue: u32 };

pub fn main() anyerror!void {
    std.debug.print("Day 2 - Part 1\n", .{});

    const file = try std.fs.cwd().openFile("../_input/day2.txt", .{});
    defer file.close();

    var buf_reader = std.io.bufferedReader(file.reader());
    const in = buf_reader.reader();

    var total: u32 = 0;

    var buffer: [1024]u8 = undefined;
    while (try in.readUntilDelimiterOrEof(&buffer, '\n')) |line| {
        total += process(line);
    }

    std.debug.print("Total: {}\n", .{total});
}

fn process(str: []const u8) u32 {
    var game = get_game(str);
    const ss = unwrap([]SubSet, game.subsets.toOwnedSlice());
    for(ss) |sub| {
        if(sub.red > MAX_RED or sub.green > MAX_GREEN or sub.blue > MAX_BLUE) {
            return 0;
        }
    }
    return game.id;
}

fn get_game(str: []const u8) Game {
    const colon = std.mem.indexOf(u8, str, ":").?;
    const raw_id = str[5..colon];
    const id = str_to_u32(raw_id);

    const raw_subsets = str[(colon + 1)..str.len];
    var ss = std.mem.splitAny(u8, raw_subsets, ";");
    var subsets = std.ArrayList(SubSet).init(allocator);
    while(ss.next()) |s| {
        const set = get_subset(s);
        unwrap(void, subsets.append(set));
    }

    return Game { .id = id, .subsets = subsets };
}

fn get_subset(str: []const u8) SubSet {
    var subs = std.mem.splitAny(u8, str, ",");

    var red: u32 = 0;
    var green: u32 = 0;
    var blue: u32 = 0;

    while(subs.next()) |sset| {
        const s = std.mem.trim(u8, sset, " \t\n");
        if(std.mem.endsWith(u8, s, "red")) {
            red += str_to_u32(s[0..(s.len - 4)]);
        }
        else if(std.mem.endsWith(u8, s, "green")) {
            green += str_to_u32(s[0..(s.len - 6)]);
        }
        else if(std.mem.endsWith(u8, s, "blue")) {
            blue += str_to_u32(s[0..(s.len - 5)]);
        }
        else {
            std.debug.panic("Unknown color", .{});
        }
    }

    return SubSet { .red = red, .green = green, .blue = blue };
}

fn str_to_u32(str: []const u8) u32 {
    var acc: u32 = 0;
    for(str, 0..) |c, i| {
        acc += (unwrap(u32, std.math.powi(u32, 10, @as(u32, @intCast(str.len - i - 1))))) * (c - ZERO);
    }
    return acc;
}

fn unwrap(comptime T: type, val: anyerror!T) T {
    if(val) |value|{
        return value;
    } else |err|{
        std.debug.panic("Panicked at Error: {any}",.{err});
    }
}
