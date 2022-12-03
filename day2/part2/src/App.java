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
                    if (opponent == 'A'){
                        points += 3;
                    } else if (opponent == 'B') {
                        points += 1;
                    } else {
                        points += 2;
                    }
                } else if (me == 'Y') {
                    points += 3;
                    if (opponent == 'A'){
                        points += 1;
                    } else if (opponent == 'B') {
                        points += 2;
                    } else {
                        points += 3;
                    }
                } else {
                    points += 6;
                    if (opponent == 'A'){
                        points += 2;
                    } else if (opponent == 'B') {
                        points += 3;
                    } else {
                        points += 1;
                    }
                }
            }
            System.out.println(points);
        }
    }
}
