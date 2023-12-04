import java.io.BufferedReader;
import java.io.File;
import java.io.FileReader;
import java.io.IOException;
import java.util.Arrays;
import java.util.stream.Stream;

public class d02p2 {
    private static final int ZERO = 48;

    public static void main(String[] args) throws IOException {
        System.out.println("Day 2 - Part 2");

        BufferedReader reader = new BufferedReader(new FileReader(new File("../_input/day02.txt")));
        
        int total = 0;

        String line;
        while((line = reader.readLine()) != null) {
            total += process(line.trim());
        }

        System.out.println("Total: " + total);
    }

    private static int process(String str) {
        Game game = getGame(str);

        int maxRed = 0;
        int maxGreen = 0;
        int maxBlue = 0;

        for(var s : game.subsets) {
            maxRed = Math.max(maxRed, s.red);
            maxGreen = Math.max(maxGreen, s.green);
            maxBlue = Math.max(maxBlue, s.blue);
        }

        return maxRed * maxGreen * maxBlue;
    }

    private static Game getGame(String str) {
        int colon = str.indexOf(":");
        String rawSubSets = str.substring(colon + 2);
        Stream<String> ss = Arrays.stream(rawSubSets.split(";")).map(i -> i.trim());
        SubSet[] sets = ss.map(i -> getSubSet(i)).toArray(size -> new SubSet[size]);

        return new Game(sets);
    }

    private static SubSet getSubSet(String str) {
        String[] sets = Arrays.stream(str.split(",")).map(i -> i.trim()).toArray(size -> new String[size]);

        int red = 0;
        int green = 0;
        int blue = 0;

        for(var s : sets) {
            if(s.endsWith("red"))
                red += stringToInt(s.substring(0, s.length() - 4));
            else if(s.endsWith("green"))
                green += stringToInt(s.substring(0, s.length() - 6));
            else if(s.endsWith("blue"))
                blue += stringToInt(s.substring(0, s.length() - 5));  
            else {
                throw new RuntimeException("Unknown color");
            }
        }

        return new SubSet(red, green, blue);
    }

    private static int stringToInt(String str) {
        int acc = 0;
        for(int i = 0; i < str.length(); i++) {
            int c = str.charAt(i);
            acc += Math.pow(10.0, str.length() - i - 1) * (c - ZERO);
        }
        return acc;
    }

    private record Game(SubSet[] subsets) {}
    private record SubSet(int red, int green, int blue) {}
}
