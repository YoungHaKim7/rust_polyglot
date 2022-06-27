
#include <inttypes.h>
#include <math.h>
#include <omp.h>
#include <stdio.h>

uint64_t fib(unsigned m) { // Direct Calculation, correct for abs(m) <= 71
  double sqrt5r = 1.0 / sqrt(5.0);
  double golden = (1.0 + sqrt(5.0)) / 2.0;
  return rint(pow(golden, m) * sqrt5r);
}

void fib_sequence(unsigned n, uint64_t *f) {
#pragma omp parallel
  {
    size_t t = omp_get_thread_num();
    size_t nt = omp_get_num_threads();
    int x0 = (t + 0) * n / nt;
    int x1 = (t + 1) * n / nt;
    f[x0 + 0] = fib(x0 + 0);
    f[x0 + 1] = fib(x0 + 1);
    for (int i = x0 + 2; i < x1; i++) {
      f[i] = f[i - 2] + f[i - 1];
    }
  }
}
