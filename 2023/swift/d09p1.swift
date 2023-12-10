import Foundation

func main() {
    print("Day 9 - Part 1")

    let raw_data = try! NSString(
        contentsOfFile: "../_input/day09.txt", encoding: String.Encoding.utf8.rawValue)
    let data = raw_data as String

    let total = process(String(data))
    print("Total: \(total)")
}

func process(_ string: String) -> Int {
    let readings: [[Int]] = string.split(separator: "\n").map({
        $0.components(separatedBy: " ").map({ Int($0)! })
    })

    var total = 0
    for reading in readings {
        var stack = [reading]

        var i = 0
        while !stack[i].allSatisfy({ $0 == 0 }) {
            let s = stack[i]
            var v: [Int] = []
            var vi = 0

            while vi < s.endIndex - 1 {
                v.append(s[vi + 1] - s[vi])
                vi += 1
            }

            stack.append(v)
            i += 1
        }

        stack.reverse()
        stack[0].append(0)

        i = 1
        while i < stack.count {
            let a = stack[i - 1].last!
            let b = stack[i].last!

            stack[i].append(a + b)
            i += 1
        }

        total += stack.last!.last!
    }

    return total
}

main()
