// #define CUTOFF 5
// int fib_s(int n) {
//   if (n == 0 || n == 1)
//     return n;
//   int res, a, b;
//   a = fib_s(n - 1);
//   b = fib_s(n - 2);
//   res = a + b;
//   return res;
// }
// int fib_m(int n, int co) {
//   if (co >= CUTOFF)
//     return fib_s(n);
//   if (n == 0 || n == 1)
//     return n;
//   int res, a, b;
//   co++;
// #pragma omp task shared(a)
//   a = fib_m(n - 1, co);
// #pragma omp task shared(b)
//   b = fib_m(n - 2, co);
// #pragma omp taskwait
//   res = a + b;
//   return res;
// }

// int main() {
//   omp_set_nested(1);
//   omp_set_num_threads(4);
//   double start_time = omp_get_wtime();
// #pragma omp parallel
//   {
// #pragma omp single
//     { cout << fib_m(25, 1) << endl; }
//   }
//   double time = omp_get_wtime() - start_time;
//   std::cout << "Time(ms): " << time * 1000 << std::endl;
//   return 0;
// }
#include <iostream>
#include <omp.h>

using namespace std;

int fib(int n) {
  if (n == 0 || n == 1)
    return n;

  int res, a, b;
#pragma omp parallel
  {
#pragma omp single
    {
#pragma omp task shared(a)
      a = fib(n - 1);
#pragma omp task shared(b)
      b = fib(n - 2);
#pragma omp taskwait
      res = a + b;
    }
  }
  return res;
}

int main() { cout << fib(40); }
