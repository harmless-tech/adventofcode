import java.io.BufferedReader;
import java.io.File;
import java.io.FileReader;
import java.io.IOException;
import java.util.ArrayList;
import java.util.Arrays;

public class D04P1 {
    private static final int ZERO = 48;

    public static void main(String[] args) throws IOException {
        System.out.println("Day 4 - Part 1");

        BufferedReader reader = new BufferedReader(new FileReader(new File("../_input/day4.txt")));
        
        int total = 0;

        String line;
        while((line = reader.readLine()) != null) {
            total += process(line.trim());
        }

        System.out.println("Total: " + total);
    }

    private static int process(String str) {
        String[] nums = str.split(":")[1].split("\\|");
        int[] winning_nums = get_nums(nums[0]);
        int[] have_nums = get_nums(nums[1]);

        Integer count = null;
        for(var i : have_nums) {
            if(Arrays.stream(winning_nums).anyMatch(w -> w == i)) {
                if(count == null) count = 1;
                else count *= 2;
            }
        }

        if(count == null) return 0;
        return count;
    }

    private static int[] get_nums(String str) {
        String[] raws = str.split(" ");
        ArrayList<Integer> builder = new ArrayList<>();
        for(var r : raws) {
            r = r.trim();
            if(!r.isEmpty()) builder.add(stringToInt(r));
        }
        return builder.stream().mapToInt(i -> i).toArray();
    }

    private static int stringToInt(String str) {
        int acc = 0;
        for(int i = 0; i < str.length(); i++) {
            int c = str.charAt(i);
            acc += Math.pow(10.0, str.length() - i - 1) * (c - ZERO);
        }
        return acc;
    }
}
