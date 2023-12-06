import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;
import java.util.ArrayList;

public class d06p1 {
    private static final int ZERO = 48;

    public static void main(String[] args) throws IOException {
        System.out.println("Day 6 - Part 1");

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

        long[] times = getNums(rawTimes.split(":")[1]);
        long[] distances = getNums(rawDists.split(":")[1]);

        long total = 1;
        for (int i = 0; i < times.length; i++) {
            var t = times[i];
            var d = distances[i];

            long counter = 0;
            for (int speed = 0; speed <= t; speed++) {
                var timeLeft = t - speed;
                var dis = timeLeft * speed;
                if (dis > d) counter++;
            }
            total *= counter;
        }

        return total;
    }

    private static long[] getNums(String str) {
        String[] raws = str.split(" ");
        ArrayList<Long> builder = new ArrayList<>();
        for (var r : raws) {
            r = r.trim();
            if (!r.isEmpty()) builder.add(stringToLong(r));
        }
        return builder.stream().mapToLong(i -> i).toArray();
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
