// https://atcoder.jp/contests/abc306/submissions/46441422

using System;

public class A
{
  static void Main()
  {
    Console.ReadLine();
    string s = Console.ReadLine();

    for(int i = 0; i < s.Length; i++)
    {
      Console.Write($"{s[i]}{s[i]}");
    }
  }
}

