import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;
import java.util.ArrayList;
import java.util.HashMap;

public class d08p2 {
    public static void main(String[] args) throws IOException {
        System.out.println("Day 8 - Part 2");

        BufferedReader reader = new BufferedReader(new FileReader("../_input/day08.txt"));

        long total = process(reader);
        reader.close();

        System.out.println("Total: " + total);
    }

    private static long process(BufferedReader reader) throws IOException {
        Direction[] instructions = null;
        HashMap<String, Node> nodes = new HashMap<>();
        ArrayList<String> starting = new ArrayList<>();

        String line;
        while ((line = reader.readLine()) != null) {
            if (instructions == null) instructions = processDirections(line);
            else if (!line.isBlank()) {
                var node = processNode(line);
                nodes.put(node.name, node);
                if (node.name.endsWith("A")) starting.add(node.name);
            }
        }

        ArrayList<Long> steps = new ArrayList<>();
        for (String currentNode : starting) {
            int currentDir = 0;
            boolean hit = false;
            long counter = 0;

            while (!hit) {
                Node node = nodes.get(currentNode);
                switch (instructions[currentDir]) {
                    case Direction.LEFT -> currentNode = node.left;
                    case Direction.RIGHT -> currentNode = node.right;
                }

                counter++;
                if (currentNode.endsWith("Z")) hit = true;

                currentDir++;
                if (currentDir >= instructions.length) currentDir = 0;
            }

            steps.add(counter);
        }

        long num = steps.removeFirst();
        for (long n : steps.toArray(Long[]::new)) num = lcm(n, num);

        return num;
    }

    private static long lcm(long a, long b) {
        return (a * b) / gcd(a, b);
    }

    private static long gcd(long a, long b) {
        if (b == 0) return a;
        return gcd(b, a % b);
    }

    private static Direction[] processDirections(String line) {
        ArrayList<Direction> dirs = new ArrayList<>();
        for (int c : line.chars().toArray()) dirs.add(Direction.get(c));
        return dirs.toArray(Direction[]::new);
    }

    private static Node processNode(String line) {
        String[] n = line.split(" = ");
        String[] ns = n[1].substring(1, n[1].length() - 1).split(", ");
        return new Node(n[0], ns[0], ns[1]);
    }

    private enum Direction {
        LEFT,
        RIGHT;

        public static Direction get(int c) {
            switch (c) {
                case (int) 'L' -> {
                    return Direction.LEFT;
                }
                case (int) 'R' -> {
                    return Direction.RIGHT;
                }
                default -> throw new RuntimeException("Not a direction.");
            }
        }
    }

    private record Node(String name, String left, String right) {}
}
