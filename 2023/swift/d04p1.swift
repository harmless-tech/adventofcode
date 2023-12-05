import Foundation

func main() {
    print("Day 4 - Part 1")

    let raw_data = try! NSString(
        contentsOfFile: "../_input/day04.txt", encoding: String.Encoding.utf8.rawValue)
    let data = raw_data as String

    var total = 0
    for line in data.split(separator: "\n") {
        total += process(String(line))
    }

    print("Total: \(total)")
}

func process(_ string: String) -> Int {
    let nums = string.split(separator: ":")[1].split(separator: "|")
    let winning_nums = process_nums(nums[0])
    let have_nums = process_nums(nums[1])

    var count: Int? = nil
    for n in have_nums {
        if winning_nums.contains(n) {
            if count == nil {
                count = 1
            } else {
                count! *= 2
            }
        }
    }

    return count ?? 0
}

func process_nums(_ string: Substring) -> [Int] {
    return string.split(separator: " ")
        .map({ $0.trimmingCharacters(in: CharacterSet.whitespacesAndNewlines) })
        .filter({ !$0.isEmpty })
        .map({ Int($0)! })
}

main()
