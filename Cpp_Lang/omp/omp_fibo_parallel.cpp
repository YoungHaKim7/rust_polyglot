#include <omp.h>
#include <stdio.h>

int fib(int n) {
  int x, y;
  if (n <= 1)
    return 1;
#pragma omp task shared(x)
  {
    printf("the value of n=%d from thread = %d\n", n, omp_get_thread_num());
    x = fib(n - 1);
  }
#pragma omp task shared(y)
  y = fib(n - 2);
#pragma omp taskwait
  return x + y;
}

int main() {
  double s = omp_get_wtime();
  int z = fib(42);
  double sec = omp_get_wtime() - s;

  printf("the result = %d in time = %f", z, sec);
}
