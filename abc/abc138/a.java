// https://atcoder.jp/contests/abc138/submissions/43260147

import java.util.Scanner;

class Main {
  public static void main(String[] args) {
    Scanner stdIn = new Scanner(System.in);

    int a = stdIn.nextInt();
    String s = stdIn.next();

    if(a >= 3200) {
      System.out.println(s);
    } else {
      System.out.println("red");
    }
  }
}
