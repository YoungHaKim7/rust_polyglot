#include <chrono>
#include <iostream>

using namespace std;

int main() {
  auto start = chrono::steady_clock::now();
  int MAX_TERMS = 100000000;
  double small = 1e-5;
  double Sm = 1;
  double sum = 1;

  for (int m = 1; m < MAX_TERMS; m++) {
    Sm = Sm * m / (2.0 * m + 1);
    sum += Sm;
  }

  cout.precision(12);

  cout << 2 * sum << endl;
  for (int i = 0; i < 1e7; i++) {
    cout << "";
  }

  auto end = chrono::steady_clock::now();

  double elapsed_time_ns =
      double(chrono::duration_cast<chrono::nanoseconds>(end - start).count());

  cout << "Elapsed Time (s): " << elapsed_time_ns / 1e9 << endl;
  return 0;
}
