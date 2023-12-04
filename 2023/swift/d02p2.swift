import Foundation

struct Game {
    let subsets: [SubSet]
}

struct SubSet {
    let red: Int
    let green: Int
    let blue: Int
}

func main() {
    print("Day 2 - Part 2")

    let raw_data = try! NSString(
        contentsOfFile: "../_input/day2.txt", encoding: String.Encoding.utf8.rawValue)
    let data = raw_data as String

    var total = 0
    for line in data.split(separator: "\n") {
        total += process(String(line))
    }

    print("Total: \(total)")
}

func process(_ string: String) -> Int {
    let game = getGame(string)

    var r = 0
    var g = 0
    var b = 0

    for s in game.subsets {
        r = max(r, s.red)
        g = max(g, s.green)
        b = max(b, s.blue)
    }
    return r * g * b
}

func getGame(_ string: String) -> Game {
    let colon = string.firstIndex(of: ":")!
    let raw_subsets = string[string.index(colon, offsetBy: 2)...]
    let ss = raw_subsets.split(separator: ";").map({
        String($0).trimmingCharacters(in: CharacterSet.whitespacesAndNewlines)
    })
    let subsets = ss.map({ getSubSet($0) })

    return Game(subsets: subsets)
}

func getSubSet(_ string: String) -> SubSet {
    let sets = string.split(separator: ",").map({
        String($0).trimmingCharacters(in: CharacterSet.whitespacesAndNewlines)
    })

    var red = 0
    var green = 0
    var blue = 0

    for s in sets {
        if s.hasSuffix("red") {
            red += Int(s[s.startIndex..<s.index(s.endIndex, offsetBy: -4)])!
        } else if s.hasSuffix("green") {
            green += Int(s[s.startIndex..<s.index(s.endIndex, offsetBy: -6)])!
        } else if s.hasSuffix("blue") {
            blue += Int(s[s.startIndex..<s.index(s.endIndex, offsetBy: -5)])!
        } else {
            fatalError("Unknown Color")
        }
    }

    return SubSet(red: red, green: green, blue: blue)
}

main()
