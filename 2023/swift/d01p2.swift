import Foundation

func main() {
    print("Day 1 - Part 2")

    let raw_data = try! NSString(
        contentsOfFile: "../_input/day1.txt", encoding: String.Encoding.utf8.rawValue)
    let data = raw_data as String

    var total = 0
    for line in data.split(separator: "\n") {
        total += get(line)
    }

    print("Total: \(total)")
}

func get(_ string: Substring) -> Int {
    var str = string

    var i = 0
    while i < str.count {
        let start = str.index(str.startIndex, offsetBy: i)
        let c = str[start...]

        if c.starts(with: "one") {
            str.replaceSubrange(start..<str.index(str.startIndex, offsetBy: i + 2), with: "1")
        } else if c.starts(with: "two") {
            str.replaceSubrange(start..<str.index(str.startIndex, offsetBy: i + 2), with: "2")
        } else if c.starts(with: "three") {
            str.replaceSubrange(start..<str.index(str.startIndex, offsetBy: i + 4), with: "3")
        } else if c.starts(with: "four") {
            str.replaceSubrange(start..<str.index(str.startIndex, offsetBy: i + 3), with: "4")
        } else if c.starts(with: "five") {
            str.replaceSubrange(start..<str.index(str.startIndex, offsetBy: i + 3), with: "5")
        } else if c.starts(with: "six") {
            str.replaceSubrange(start..<str.index(str.startIndex, offsetBy: i + 2), with: "6")
        } else if c.starts(with: "seven") {
            str.replaceSubrange(start..<str.index(str.startIndex, offsetBy: i + 4), with: "7")
        } else if c.starts(with: "eight") {
            str.replaceSubrange(start..<str.index(str.startIndex, offsetBy: i + 4), with: "8")
        } else if c.starts(with: "nine") {
            str.replaceSubrange(start..<str.index(str.startIndex, offsetBy: i + 3), with: "9")
        }

        i += 1
    }

    var first: Int? = nil
    var last: Int? = nil

    for c in str {
        if c.isWholeNumber {
            if first == nil {
                first = Int(String(c))
            } else {
                last = Int(String(c))
            }
        }
    }

    if last == nil {
        last = first
    }

    return (first! * 10) + last!
}

main()
