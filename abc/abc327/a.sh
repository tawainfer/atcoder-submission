# https://atcoder.jp/contests/abc327/submissions/47519099

#!/bin/sh

read n
read s

if [ $(echo $s | grep -E '(ab|ba)' | wc -l) -ne 0 ]
then
  echo 'Yes'
else
  echo 'No'
fi

