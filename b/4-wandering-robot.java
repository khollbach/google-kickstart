import java.util.*;
import java.io.*;

public class Solution {
    public static void main(String[] args) {
        Scanner in = new Scanner(new BufferedReader(new InputStreamReader(System.in)));
        int T = in.nextInt();
        for (int t = 1; t <= T; t++) {
            int W = in.nextInt();
            int H = in.nextInt();
            int L = in.nextInt();
            int U = in.nextInt();
            int R = in.nextInt();
            int D = in.nextInt();
            double result = Solution.success_chance(W, H, L, U, R, D);
            System.out.println("Case #" + t + ": " + result);
        }
    }

    private static double success_chance(int W, int H, int L, int U, int R, int D) {
        L -= 1;
        U -= 1;
        R -= 1;
        D -= 1;

        double grid[] = new double[2 * W];

        for (int h = 0; h < H; h++) {
            for (int w = 0; w < W; w++) {
                double val;
                if (h == 0 && w == 0) {
                    val = 1.0;
                } else if (U <= h && h <= D && L <= w && w <= R) {
                    val = 0.0;
                } else {
                    double above;
                    if (h != 0) {
                        above = grid[(h-1) % 2 * W + w];
                    } else {
                        above = 0.0;
                    }
                    if (w == W - 1) {
                        above *= 2.0;
                    }

                    double left;
                    if (w != 0) {
                        left = grid[h % 2 * W + (w-1)];
                    } else {
                        left = 0.0;
                    }
                    if (h == H - 1) {
                        left *= 2.0;
                    }

                    val = (above + left) / 2.0;
                }
                grid[h % 2 * W + w] = val;
            }
        }

        return grid[(H-1) % 2 * W + (W-1)];
    }
}
