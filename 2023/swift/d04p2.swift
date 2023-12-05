import Foundation

func main() {
    print("Day 4 - Part 2")

    let raw_data = try! NSString(
        contentsOfFile: "../_input/day04.txt", encoding: String.Encoding.utf8.rawValue)
    let data = raw_data as String

    let total = process(String(data))
    print("Total: \(total)")
}

func process(_ string: String) -> Int {
    var ccs: [Int] = []
    for line in string.split(separator: "\n") {
        ccs.append(process_card(line))
    }
    let cards = ccs

    var count = 0
    for i in 0..<cards.count {
        count += process_num(i, cards)
    }

    return count
}

func process_num(_ card: Int, _ cards: [Int]) -> Int {
    var count = 1
    let c = cards[card]
    if c < 1 { return count }
    for i in 1...c {
        if card + i < cards.count {
            count += process_num(card + i, cards)
        }
    }
    return count
}

func process_card(_ string: Substring) -> Int {
    let nums = string.split(separator: ":")[1].split(separator: "|")
    let winning_nums = process_nums(nums[0])
    let have_nums = process_nums(nums[1])

    var count = 0
    for n in have_nums {
        if winning_nums.contains(n) {
            count += 1
        }
    }

    return count
}

func process_nums(_ string: Substring) -> [Int] {
    return string.split(separator: " ")
        .map({ $0.trimmingCharacters(in: CharacterSet.whitespacesAndNewlines) })
        .filter({ !$0.isEmpty })
        .map({ Int($0)! })
}

main()
