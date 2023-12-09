import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;
import java.util.ArrayList;

public class d09p2 {
    public static void main(String[] args) throws IOException {
        System.out.println("Day 9 - Part 2");

        BufferedReader reader = new BufferedReader(new FileReader("../_input/day09.txt"));

        int total = process(reader);
        reader.close();

        System.out.println("Total: " + total);
    }

    private static int process(BufferedReader reader) throws IOException {
        ArrayList<ArrayList<Integer>> readings = new ArrayList<>();
        String line;
        while ((line = reader.readLine()) != null) readings.add(getNums(line));

        int total = 0;
        for (var reading : readings) {
            ArrayList<ArrayList<Integer>> stack = new ArrayList<>();
            stack.add(reading);

            for (int i = 0; !stack.get(i).stream().allMatch(a -> a == 0); i++) {
                var s = stack.get(i);
                ArrayList<Integer> v = new ArrayList<>();
                for (int vi = 1; vi < s.size(); vi++) v.add(s.get(vi) - s.get(vi - 1));
                stack.add(v);
            }

            var rev = stack.reversed();
            rev.get(0).add(0, 0);

            for (int i = 1; i < rev.size(); i++) {
                var a = rev.get(i - 1).getFirst();
                var b = rev.get(i).getFirst();
                rev.get(i).add(0, b - a);
            }

            total += rev.getLast().getFirst();
        }

        return total;
    }

    private static ArrayList<Integer> getNums(String str) {
        String[] raws = str.split(" ");
        ArrayList<Integer> builder = new ArrayList<>();
        for (var r : raws) {
            r = r.trim();
            if (!r.isEmpty()) builder.add(stringToInt(r));
        }
        return builder;
    }

    private static int stringToInt(String str) {
        return Integer.parseInt(str);
    }
}
