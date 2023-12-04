import java.io.BufferedReader;
import java.io.File;
import java.io.FileReader;
import java.io.IOException;

public class d01p2 {
    private static final int ZERO = 48;
    private static final int NINE = 57;

    public static void main(String[] args) throws IOException {
        System.out.println("Day 1 - Part 2");

        BufferedReader reader = new BufferedReader(new FileReader(new File("../_input/day01.txt")));
        
        int total = 0;

        String line;
        while((line = reader.readLine()) != null) {
            total += get(line.trim());
        }

        System.out.println("Total: " + total);
    }

    private static int get(String str) {
        for(int i = 0; i < str.length(); i++) {
            if(str.startsWith("one", i)) {
                str = str.substring(0, i) + "1" + str.substring(i + 2);
            }
            else if(str.startsWith("two", i)) {
                str = str.substring(0, i) + "2" + str.substring(i + 2);
            }
            else if(str.startsWith("three", i)) {
                str = str.substring(0, i) + "3" + str.substring(i + 4);
            }
            else if(str.startsWith("four", i)) {
                str = str.substring(0, i) + "4" + str.substring(i + 3);
            }
            else if(str.startsWith("five", i)) {
                str = str.substring(0, i) + "5" + str.substring(i + 3);
            }
            else if(str.startsWith("six", i)) {
                str = str.substring(0, i) + "6" + str.substring(i + 2);
            }
            else if(str.startsWith("seven", i)) {
                str = str.substring(0, i) + "7" + str.substring(i + 4);
            }
            else if(str.startsWith("eight", i)) {
                str = str.substring(0, i) + "8" + str.substring(i + 4);
            }
            else if(str.startsWith("nine", i)) {
                str = str.substring(0, i) + "9" + str.substring(i + 3);
            }
        }

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
