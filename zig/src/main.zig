const std = @import("std");

const ProductRangeError = error{ParseError};

fn parse_number(s: *[]const u8) !u64 {
    const end = std.mem.indexOfNone(u8, s.*, "0123456789") orelse s.len;

    if (end == 0) {
        return error.ParseError;
    }

    const number = try std.fmt.parseInt(u64, s.*[0..end], 10);

    s.* = s.*[end..];

    return number;
}

test "parse_number exact" {
    var s: []const u8 = "1234";

    const n = try parse_number(&s);

    try std.testing.expectEqual(1234, n);
    try std.testing.expectEqualStrings("", s);
}

test "parse_number trailing" {
    var s: []const u8 = "1234hello";

    const n = try parse_number(&s);

    try std.testing.expectEqual(1234, n);
    try std.testing.expectEqualStrings("hello", s);
}

test "parse_number empty" {
    var s: []const u8 = "";

    try std.testing.expectError(error.ParseError, parse_number(&s));
}

test "parse_number non number" {
    var s: []const u8 = "hello";

    try std.testing.expectError(error.ParseError, parse_number(&s));
}

const ProductRange = struct {
    start: u64,
    end: u64,

    fn parse(s: []const u8) !@This() {
        var s_inner = s; // NOTE: This seems to be needed to ensure the param isn't `const`.
        const start = try parse_number(&s_inner);

        if (!(s_inner.len > 0 and s_inner.ptr[0] == '-')) {
            return error.ParseError;
        }

        s_inner = s_inner[1..];

        const end = try parse_number(&s_inner);

        if (s_inner.len != 0) {
            std.debug.print("{s}\n", .{s_inner});
            return error.ParseError;
        }

        return .{
            .start = start,
            .end = end,
        };
    }
};

test "ProductRange.parse" {
    try std.testing.expectEqual(ProductRange{ .start = 1234, .end = 4321 }, try ProductRange.parse("1234-4321"));
}

const ProductRangeIterator = struct {
    reader: *std.io.Reader,

    fn next(self: *@This()) !?ProductRange {
        while (std.ascii.isWhitespace(self.reader.peekByte() catch |err| switch (err) {
            error.EndOfStream => return null,
            else => return err,
        })) {
            self.reader.toss(1);
        }

        const s = try self.reader.takeDelimiter(',') orelse return null;
        const s_trimmed = std.mem.trim(u8, s, &std.ascii.whitespace);
        return try ProductRange.parse(s_trimmed);
    }
};

test "ProductRangeIterator empty" {
    var reader = std.io.Reader.fixed("");
    var iter = ProductRangeIterator{ .reader = &reader };
    try std.testing.expectEqual(null, try iter.next());
}

test "ProductRangeIterator single" {
    var reader = std.io.Reader.fixed("1234-4321");
    var iter = ProductRangeIterator{ .reader = &reader };
    try std.testing.expectEqual(ProductRange{ .start = 1234, .end = 4321 }, try iter.next());
    try std.testing.expectEqual(null, try iter.next());
}

test "ProductRangeIterator multiple" {
    var reader = std.io.Reader.fixed("1234-4321,999-111,92384-15692");
    var iter = ProductRangeIterator{ .reader = &reader };
    try std.testing.expectEqual(ProductRange{ .start = 1234, .end = 4321 }, try iter.next());
    try std.testing.expectEqual(ProductRange{ .start = 999, .end = 111 }, try iter.next());
    try std.testing.expectEqual(ProductRange{ .start = 92384, .end = 15692 }, try iter.next());
    try std.testing.expectEqual(null, try iter.next());
}

pub fn main() !void {
    const allocator = std.heap.page_allocator;

    var file = try read_file(allocator, "inputs");
    defer file.close();

    var buf: [1024]u8 = undefined;
    var reader = file.reader(&buf);

    const part1_solution = try part1(&reader.interface);

    try reader.seekTo(0);

    const part2_solution = try part2(&reader.interface);

    std.debug.print("part 1: {}\n", .{part1_solution});
    std.debug.print("part 2: {}\n", .{part2_solution});
}

fn read_file(allocator: std.mem.Allocator, kind: []const u8) !std.fs.File {
    const input_path = try std.fs.path.join(allocator, &[_][]const u8{ "../data", kind, "02.txt" });
    defer allocator.free(input_path);

    return try std.fs.cwd().openFile(input_path, .{ .mode = .read_only });
}

fn part1(reader: *std.io.Reader) !u64 {
    var ranges = ProductRangeIterator{
        .reader = reader,
    };

    var answer: u64 = 0;
    while (try ranges.next()) |range| {
        for (range.start..range.end + 1) |n| {
            if (repeated_times(n, 2)) {
                answer += n;
            }
        }
    }
    return answer;
}

test part1 {
    const allocator = std.testing.allocator;
    const kind: []const u8 = "examples";
    var file = try read_file(allocator, kind);
    defer file.close();
    var buf: [1024]u8 = undefined;
    var reader = file.reader(&buf);
    try std.testing.expectEqual(1227775554, try part1(&reader.interface));
}

fn part2(reader: *std.io.Reader) !u64 {
    var ranges = ProductRangeIterator{
        .reader = reader,
    };

    var answer: u64 = 0;
    while (try ranges.next()) |range| {
        for (range.start..range.end + 1) |n| {
            const tens = std.math.log10_int(n) + 1;

            for (2..tens + 1) |i| {
                if (repeated_times(n, @intCast(i))) {
                    answer += n;
                    break;
                }
            }
        }
    }
    return answer;
}

test part2 {
    const allocator = std.testing.allocator;
    const kind: []const u8 = "examples";
    var file = try read_file(allocator, kind);
    defer file.close();
    var buf: [1024]u8 = undefined;
    var reader = file.reader(&buf);
    try std.testing.expectEqual(4174379265, try part2(&reader.interface));
}

fn repeated_times(n: u64, i: u32) bool {
    const tens = std.math.log10_int(n) + 1;

    if (tens % i != 0) {
        return false;
    }

    const digits = tens / i;
    const cmp = extract_digits(n, 0, digits);

    for (1..i) |j| {
        if (cmp != extract_digits(n, j * digits, digits)) {
            return false;
        }
    }

    return true;
}

fn extract_digits(n: u64, start: u64, count: u64) u64 {
    return n / std.math.pow(u64, 10, start) % std.math.pow(u64, 10, count);
}
