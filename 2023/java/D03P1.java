import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;
import java.util.HashSet;

public class d03p1 {
    private static final int ZERO = 48;
    private static final int NINE = 57;

    public static void main(String[] args) throws IOException {
        System.out.println("Day 3 - Part 1");

        BufferedReader reader = new BufferedReader(new FileReader("../_input/day03.txt"));
        int[][] lines = reader.lines().map(line -> line.chars().toArray()).toArray(int[][]::new);

        int total = process(lines);
        System.out.println("Total: " + total);
    }

    private static int process(final int[][] lines) {
        int total = 0;

        for(int row = 0; row < lines.length; row++) {
            int[] line = lines[row];
            for(int col = 0; col < line.length; col++) {
                int c = line[col];

                if(c >= ZERO && c <= NINE) {
                    HashSet<SymbolID> syms = new HashSet<>();
                    StringBuilder num = new StringBuilder();

                    while(col < line.length && c >= ZERO && c <= NINE) {
                        for(int rr = Math.max(row - 1, 0); rr < row + 2 && rr < lines.length; rr++) {
                            for(int cc = Math.max(col - 1, 0); cc < col + 2 && cc < line.length; cc++) {
                                int sc = lines[rr][cc];
                                if(!(sc >= ZERO && sc <= NINE) && sc != '.')
                                    syms.add(new SymbolID(rr, cc));
                            }
                        }
                        
                        num.append((char) c);
                        col++;
                        c = line[col < line.length ? col : 0];
                    }

                    int number = stringToInt(num.toString());
                    if(!syms.isEmpty()) total += number;
                }
            }
        }

        return total;
    }

    private static int stringToInt(String str) {
        int acc = 0;
        for(int i = 0; i < str.length(); i++) {
            int c = str.charAt(i);
            acc += (int) (Math.pow(10.0, str.length() - i - 1) * (c - ZERO));
        }
        return acc;
    }

    private record SymbolID(int row, int col) {
        @Override
        public int hashCode() {
            return row * col + row;
        }
    }
}
