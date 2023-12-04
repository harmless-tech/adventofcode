import java.io.BufferedReader;
import java.io.File;
import java.io.FileReader;
import java.io.IOException;

public class D01P1 {
    private static final int ZERO = 48;
    private static final int NINE = 57;

    public static void main(String[] args) throws IOException {
        System.out.println("Day 1 - Part 1");

        BufferedReader reader = new BufferedReader(new FileReader(new File("../_input/day1.txt")));
        
        int total = 0;

        String line;
        while((line = reader.readLine()) != null) {
            total += get(line.trim());
        }

        System.out.println("Total: " + total);
    }

    private static int get(String str) {
        int first = -1;
        int last = -1;

        for(int c : str.chars().toArray()) {
            if(c >= ZERO && c <= NINE) {
                if(first == -1) first = c - ZERO;
                else {
                    last = c - ZERO;
                }
            }
        }

        if(last == -1) last = first;
        return (first * 10) + last;
    }
}
