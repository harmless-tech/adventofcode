import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;
import java.util.ArrayList;

public class d05p1 {
    private static final int ZERO = 48;

    public static void main(String[] args) throws IOException {
        System.out.println("Day 5 - Part 1");

        BufferedReader reader = new BufferedReader(new FileReader("../_input/day05.txt"));

        long total = process(reader);
        reader.close();

        System.out.println("Total: " + total);
    }

    private static long process(BufferedReader reader) throws IOException {
        ArrayList<String> linesArr = new ArrayList<>();
        String line;
        while ((line = reader.readLine()) != null) {
            linesArr.add(line.trim());
        }
        String[] lines = linesArr.toArray(String[]::new);

        long[] seeds = null;
        ArrayList<Mappings[]> mappings = new ArrayList<>();
        ArrayList<String> mapLines = new ArrayList<>();

        int i = 0;
        while (i < lines.length) {
            line = lines[i];
            if (line.isBlank() || i + 1 == lines.length) {
                if (mapLines.get(0).startsWith("seeds:")) seeds = convertSeeds(mapLines.get(0));
                else mappings.add(convertMap(mapLines.toArray(String[]::new)));
                mapLines = new ArrayList<>();
            } else mapLines.add(line);

            i++;
        }

        for (Mappings[] m : mappings) mapVals(seeds, m);

        long min = seeds[0];
        for (var s : seeds) min = Math.min(min, s);

        return min;
    }

    private static void mapVals(long[] vals, Mappings[] maps) {
        boolean[] changes = new boolean[vals.length];
        for (var map : maps) {
            var destStart = map.destStart;
            var srcStart = map.srcStart;
            var srcEnd = srcStart + map.rangeLen - 1;
            var diff = destStart - srcStart;

            for (int i = 0; i < vals.length; i++) {
                var v = vals[i];
                if (v >= srcStart && v <= srcEnd && !changes[i]) {
                    vals[i] += diff;
                    changes[i] = true;
                }
            }
        }
    }

    private static long[] convertSeeds(String str) {
        String seedLine = str.split(":")[1].trim();
        return getNums(seedLine);
    }

    private static Mappings[] convertMap(String[] items) {
        ArrayList<Mappings> maps = new ArrayList<>();
        for (int i = 1; i < items.length; i++) {
            long[] nums = getNums(items[i]);
            maps.add(new Mappings(nums[0], nums[1], nums[2]));
        }
        return maps.toArray(Mappings[]::new);
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

    private record Mappings(long destStart, long srcStart, long rangeLen) {}
}
