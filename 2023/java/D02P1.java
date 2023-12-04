import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;
import java.util.Arrays;
import java.util.stream.Stream;

public class d02p1 {
    private static final int ZERO = 48;

    private static final int MAX_RED = 12;
    private static final int MAX_GREEN = 13;
    private static final int MAX_BLUE = 14;

    public static void main(String[] args) throws IOException {
        System.out.println("Day 2 - Part 1");

        BufferedReader reader = new BufferedReader(new FileReader("../_input/day02.txt"));
        
        int total = 0;

        String line;
        while((line = reader.readLine()) != null) {
            total += process(line.trim());
        }

        System.out.println("Total: " + total);
    }

    private static int process(String str) {
        Game game = getGame(str);
        for(var s : game.subsets) {
            if(s.red > MAX_RED || s.green > MAX_GREEN || s.blue > MAX_BLUE)
                return 0;
        }
        return game.id;
    }

    private static Game getGame(String str) {
        int colon = str.indexOf(":");
        String rawId = str.substring(5, colon);
        int id = stringToInt(rawId);

        String rawSubSets = str.substring(colon + 2);
        Stream<String> ss = Arrays.stream(rawSubSets.split(";")).map(String::trim);
        SubSet[] sets = ss.map(d02p1::getSubSet).toArray(SubSet[]::new);

        return new Game(id, sets);
    }

    private static SubSet getSubSet(String str) {
        String[] sets = Arrays.stream(str.split(",")).map(String::trim).toArray(String[]::new);

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
            acc += (int) (Math.pow(10.0, str.length() - i - 1) * (c - ZERO));
        }
        return acc;
    }

    private record Game(int id, SubSet[] subsets) {}
    private record SubSet(int red, int green, int blue) {}
}
