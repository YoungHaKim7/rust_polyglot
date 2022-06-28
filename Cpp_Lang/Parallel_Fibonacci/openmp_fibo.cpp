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
