// https://atcoder.jp/contests/abc314/submissions/44488966

#include <bits/stdc++.h>
using namespace std;

int main() {
  string s = ".1415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421170679";

  int n;
  cin >> n;

  cout << 3;
  for(int i = 0; i <= n; i++) {
    cout << s[i];
  }
}
