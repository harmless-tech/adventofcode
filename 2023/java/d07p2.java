import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;
import java.util.ArrayList;
import java.util.Collections;
import java.util.HashMap;

public class d07p2 {
    private static final int ZERO = 48;

    public static void main(String[] args) throws IOException {
        System.out.println("Day 7 - Part 2");

        BufferedReader reader = new BufferedReader(new FileReader("../_input/day07.txt"));

        long total = process(reader);
        reader.close();

        System.out.println("Total: " + total);
    }

    private static int process(BufferedReader reader) throws IOException {
        ArrayList<Hand> hands = new ArrayList<>();
        String line;
        while ((line = reader.readLine()) != null) hands.add(processLine(line));

        Collections.sort(hands);

        int counter = 0;
        for (int i = 0; i < hands.size(); i++) counter += hands.get(i).bet * (i + 1);
        return counter;
    }

    private static Hand processLine(String line) {
        String[] s = line.split(" ");

        Card[] cards = {Card.TWO, Card.TWO, Card.TWO, Card.TWO, Card.TWO};
        int[] cs = s[0].chars().toArray();
        for (int i = 0; i < cs.length; i++) cards[i] = Card.get(cs[i]);

        return new Hand(cards, HandType.get(cards), stringToInt(s[1]));
    }

    private static int stringToInt(String str) {
        int acc = 0;
        for (int i = 0; i < str.length(); i++) {
            int c = str.charAt(i);
            acc += (int) (Math.pow(10.0, str.length() - i - 1) * (c - ZERO));
        }
        return acc;
    }

    private enum Card {
        JOKER(0),
        TWO(1),
        THREE(2),
        FOUR(3),
        FIVE(4),
        SIX(5),
        SEVEN(6),
        EIGHT(7),
        NINE(8),
        TEN(9),
        QUEEN(10),
        KING(11),
        ACE(12);

        public final int label;

        Card(int label) {
            this.label = label;
        }

        public static Card get(int c) {
            switch (c) {
                case (int) '2' -> {
                    return Card.TWO;
                }
                case (int) '3' -> {
                    return Card.THREE;
                }
                case (int) '4' -> {
                    return Card.FOUR;
                }
                case (int) '5' -> {
                    return Card.FIVE;
                }
                case (int) '6' -> {
                    return Card.SIX;
                }
                case (int) '7' -> {
                    return Card.SEVEN;
                }
                case (int) '8' -> {
                    return Card.EIGHT;
                }
                case (int) '9' -> {
                    return Card.NINE;
                }
                case (int) 'T' -> {
                    return Card.TEN;
                }
                case (int) 'J' -> {
                    return Card.JOKER;
                }
                case (int) 'Q' -> {
                    return Card.QUEEN;
                }
                case (int) 'K' -> {
                    return Card.KING;
                }
                case (int) 'A' -> {
                    return Card.ACE;
                }
                default -> throw new RuntimeException("Could not convert int to card.");
            }
        }
    }

    private enum HandType {
        HIGH_CARD(0),
        ONE_PAIR(1),
        TWO_PAIR(2),
        THREE_KIND(3),
        FULL_HOUSE(4),
        FOUR_KIND(5),
        FIVE_KIND(6);

        public final int label;

        HandType(int label) {
            this.label = label;
        }

        public static HandType get(Card[] cards) {
            if (cards.length != 5) throw new RuntimeException("Cards array is not length of five.");

            HashMap<Card, Integer> map = new HashMap<>();
            for (Card c : cards) {
                var i = map.getOrDefault(c, 0);
                map.put(c, ++i);
            }

            if (map.containsKey(Card.JOKER)) {
                var joker = map.remove(Card.JOKER);

                var add = new Tuple<>(Card.JOKER, 0);
                for (var m : map.entrySet()) {
                    var k = m.getKey();
                    var v = (int) m.getValue();
                    if ((v > add.y) || (v == add.y && k.compareTo(add.x) > 0))
                        add = new Tuple<>(k, v);
                }

                map.put(add.x, map.getOrDefault(add.x, 0) + joker);
            }

            int length = map.keySet().size();
            switch (length) {
                case 1 -> {
                    return HandType.FIVE_KIND;
                }
                case 2 -> {
                    if (map.containsValue(4)) return HandType.FOUR_KIND;
                    else return HandType.FULL_HOUSE;
                }
                case 3 -> {
                    if (map.containsValue(3)) return HandType.THREE_KIND;
                    else return HandType.TWO_PAIR;
                }
                case 4 -> {
                    return HandType.ONE_PAIR;
                }
                case 5 -> {
                    return HandType.HIGH_CARD;
                }
                default -> throw new RuntimeException("Invalid hand.");
            }
        }
    }

    private record Hand(Card[] cards, HandType handType, int bet) implements Comparable<Hand> {
        @Override
        public int compareTo(Hand o) {
            if (handType != o.handType) return handType.label - o.handType.label;
            else {
                for (int i = 0; i < cards.length; i++) {
                    var a = cards[i];
                    var b = o.cards[i];
                    if (a != b) return a.label - b.label;
                }
            }
            return 0;
        }
    }

    private record Tuple<X, Y>(X x, Y y) {}
}
