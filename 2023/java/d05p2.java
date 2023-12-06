import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;
import java.util.ArrayList;

public class d05p2 {
    private static final int ZERO = 48;

    public static void main(String[] args) throws IOException {
        System.out.println("Day 5 - Part 2");

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

        ArrayList<Seed> seeds = null;
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

        long min = seeds.getFirst().start;
        for (var s : seeds) min = Math.min(min, s.start);
        return min;
    }

    private static void mapVals(ArrayList<Seed> vals, Mappings[] maps) {
        ArrayList<Boolean> changes = new ArrayList<>();
        for (int i = 0; i < vals.size(); i++) changes.add(false);

        for (var map : maps) {
            var destStart = map.destStart;
            var destEnd = destStart + map.rangeLen - 1;
            var srcStart = map.srcStart;
            var srcEnd = srcStart + map.rangeLen - 1;
            var diff = destStart - srcStart;

            for (int i = 0; i < vals.size(); i++) {
                if (!changes.get(i)) {
                    var val = vals.get(i);
                    var v1 = val.start;
                    var v2 = val.end;

                    if (v1 >= srcStart && v2 <= srcEnd) {
                        // If the range is completly within, then just shift it.
                        var seed = vals.get(i);
                        seed.start += diff;
                        seed.end += diff;
                        changes.set(i, true);
                    } else if (v1 >= srcStart && v1 <= srcEnd && v2 > srcEnd) {
                        // If the upper range is out of bounds
                        var seed = vals.get(i);
                        seed.start += diff;
                        seed.end = destEnd;

                        vals.add(new Seed(srcEnd + 1, v2));

                        changes.set(i, true);
                        changes.add(false);
                    } else if (v1 < srcStart && v2 >= srcStart && v2 <= srcEnd) {
                        // If the lower range is out of bounds
                        var seed = vals.get(i);
                        seed.start = destStart;
                        seed.end += diff;

                        vals.add(new Seed(v1, srcStart - 1));

                        changes.set(i, true);
                        changes.add(false);
                    }
                }
            }
        }
    }

    private static ArrayList<Seed> convertSeeds(String str) {
        String seedLine = str.split(":")[1].trim();
        long[] nums = getNums(seedLine);

        ArrayList<Seed> seeds = new ArrayList<>();
        for (int i = 0; i < nums.length; i += 2)
            seeds.add(new Seed(nums[i], nums[i] + nums[i + 1] - 1));
        return seeds;
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

    private static class Seed {
        public long start;
        public long end;

        public Seed(long start, long end) {
            this.start = start;
            this.end = end;
        }
    }

    private record Mappings(long destStart, long srcStart, long rangeLen) {}
}
