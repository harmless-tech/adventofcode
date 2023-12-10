import Foundation

enum Direction: Character {
    case left = "L"
    case right = "R"
}

struct Node {
    let name: String
    let left: String
    let right: String

    static func fromString(_ string: String) -> Self {
        fromString(Substring(string))
    }

    static func fromString(_ string: Substring) -> Self {
        let s1 = string.split(separator: " = ", maxSplits: 1)
        let s2 = s1[1].trimmingCharacters(in: CharacterSet(charactersIn: "()")).split(
            separator: ", ", maxSplits: 1)
        return Self(name: String(s1[0]), left: String(s2[0]), right: String(s2[1]))
    }
}

func main() {
    print("Day 8 - Part 2")

    let raw_data = try! NSString(
        contentsOfFile: "../_input/day08.txt", encoding: String.Encoding.utf8.rawValue)
    let data = raw_data as String

    let total = process(String(data))
    print("Total: \(total)")
}

func process(_ string: String) -> Int {
    let input = string.trimmingCharacters(in: CharacterSet.whitespacesAndNewlines).split(
        separator: "\n")
    let instructions = input[0].map({ Direction(rawValue: $0)! })
    let (starting, map) = processMap(input[1..<input.endIndex])

    let steps = starting.map {
        var currentNode = $0
        var currentDir = 0
        var hit = false
        var counter = 0
        while !hit {
            let node = map[currentNode]!
            switch instructions[currentDir] {
            case .left:
                currentNode = node.left
            case .right:
                currentNode = node.right
            }

            counter += 1
            if currentNode.hasSuffix("Z") {
                hit = true
            }

            currentDir += 1
            if currentDir >= instructions.count {
                currentDir = 0
            }
        }

        return counter
    }

    return steps.reduce(steps[0]) { lcm($0, $1) }
}

func lcm(_ a: Int, _ b: Int) -> Int {
    return (a * b) / gcd(a, b)
}

func gcd(_ a: Int, _ b: Int) -> Int {
    return b == 0 ? a : gcd(b, a % b)
}

func processMap(_ string: ArraySlice<Substring>) -> ([String], [String: Node]) {
    var map: [String: Node] = [:]
    var starting: [String] = []

    for s in string {
        let node = Node.fromString(s)
        if node.name.hasSuffix("A") { starting.append(node.name) }
        map[node.name] = node
    }

    return (starting, map)
}

main()
