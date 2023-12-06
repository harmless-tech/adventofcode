import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;
import java.util.ArrayList;

public class d06p2 {
    private static final int ZERO = 48;

    public static void main(String[] args) throws IOException {
        System.out.println("Day 6 - Part 2");

        BufferedReader reader = new BufferedReader(new FileReader("../_input/day06.txt"));

        long total = process(reader);
        reader.close();

        System.out.println("Total: " + total);
    }

    private static long process(BufferedReader reader) throws IOException {
        String rawTimes = null;
        String rawDists = null;

        String line;
        while ((line = reader.readLine()) != null) {
            if (rawTimes == null) rawTimes = line;
            else rawDists = line;
        }

        long time = getNum(rawTimes.split(":")[1]);
        long distance = getNum(rawDists.split(":")[1]);

        long counter = 0;
        for (int speed = 0; speed <= time; speed++) {
            var timeLeft = time - speed;
            var dis = timeLeft * speed;
            if (dis > distance) counter++;
        }

        return counter;
    }

    private static long getNum(String str) {
        String[] raws = str.split(" ");
        ArrayList<String> builder = new ArrayList<>();
        for (var r : raws) {
            r = r.trim();
            if (!r.isEmpty()) builder.add(r);
        }

        StringBuilder app = new StringBuilder();
        for (var s : builder) {
            app.append(s);
        }
        return stringToLong(app.toString());
    }

    private static long stringToLong(String str) {
        long acc = 0;
        for (int i = 0; i < str.length(); i++) {
            long c = str.charAt(i);
            acc += (long) (Math.pow(10.0, str.length() - i - 1) * (c - ZERO));
        }
        return acc;
    }
}
