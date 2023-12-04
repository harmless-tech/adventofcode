import Foundation

func main() {
    print("Day 1 - Part 1")

    let raw_data = try! NSString(
        contentsOfFile: "../_input/day1.txt", encoding: String.Encoding.utf8.rawValue)
    let data = raw_data as String

    var total = 0
    for line in data.split(separator: "\n") {
        total += get(line)
    }

    print("Total: \(total)")
}

func get(_ str: Substring) -> Int {
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
