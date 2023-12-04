import java.io.BufferedReader;
import java.io.File;
import java.io.FileReader;
import java.io.IOException;
import java.util.ArrayList;
import java.util.Arrays;

public class d04p2 {
    private static final int ZERO = 48;

    public static void main(String[] args) throws IOException {
        System.out.println("Day 4 - Part 2");

        BufferedReader reader = new BufferedReader(new FileReader(new File("../_input/day04.txt")));

        int total = process(reader);
        System.out.println("Total: " + total);
    }

    private static int process(BufferedReader reader) throws IOException {
        ArrayList<Integer> cards = new ArrayList<>();
        String line;
        while((line = reader.readLine()) != null) {
            cards.add(process_card(line.trim()));
        }
        int[] ccs = cards.stream().mapToInt(i -> i).toArray();

        int count = 0;
        for(int i = 0; i < ccs.length; i++) count += process_num(i, ccs);
        return count;
    }

    private static int process_num(int card, int[] ccs) {
        int count = 1;
        var c = ccs[card];
        for(int i = 1; i <= c; i++) {
            if(card + i < ccs.length) count += process_num(card + i, ccs);
        }
        return count;
    }

    private static int process_card(String str) {
        String[] nums = str.split(":")[1].split("\\|");
        int[] winning_nums = get_nums(nums[0]);
        int[] have_nums = get_nums(nums[1]);

        int count = 0;
        for(var i : have_nums) {
            if(Arrays.stream(winning_nums).anyMatch(w -> w == i)) count += 1;
        }
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
