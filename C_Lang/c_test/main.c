#include <stdio.h>
#include <stdlib.h>
#include <time.h>

int fib(int n) {
  int i, j;
  if (n < 2)
    return n;
  i = fib(n - 1);
  j = fib(n - 2);
  return (i + j);
}
int main() {
  double start, end;
  int n = 31;
  start = (double)clock() / CLOCKS_PER_SEC;
  printf("fib(%d) = %d \n", n, fib(n));
  end = (((double)clock()) / CLOCKS_PER_SEC);
  printf("fib time : %lf sec \n", (end - start));

  return 0;
}
