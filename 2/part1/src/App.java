import java.io.BufferedReader;
import java.io.FileReader;

public class App {
    public static void main(String[] args) throws Exception {
        FileReader f = new FileReader("input.txt");
        try (BufferedReader buf = new BufferedReader(f)) {
            int points = 0;
            while (true) {
                String line = buf.readLine();
                if (line == null) {
                    break;
                }
                char opponent = line.charAt(0);
                System.out.println("Opponent: "+opponent);
                char me = line.charAt(2);
                System.out.println("Me: "+me);
                if (me == 'X') {
                    points += 1;
                    if (opponent == 'A'){
                        points += 3;
                    } else if (opponent == 'C') {
                        points += 6;
                    }
                } else if (me == 'Y') {
                    points += 2;
                    if (opponent == 'B'){
                        points += 3;
                    } else if (opponent == 'A') {
                        points += 6;
                    }
                } else {
                    points += 3;
                    if (opponent == 'C'){
                        points += 3;
                    } else if (opponent == 'B') {
                        points += 6;
                    }
                }

            }
            System.out.println(points);
        }

    }

}
