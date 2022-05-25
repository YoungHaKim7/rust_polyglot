#include <cstddef>
#include <iostream>
#include <mutex>
#include <thread>
#include <vector>

using namespace std;

bool isPrime(int n) {
  int limit = (int)::sqrt(n);
  for (int i = 2; i <= limit; i++)
    if (n % i == 0)
      return false;
  return true;
}

size_t calcPrimes(int n, int firt, int last) {
  vector<thread> threads;
  thread.reserve(n);

  int chunk = (last - first + 1) / n;

  vector<int> result;

  for (int i = 0; i < n; i++) {
    int start = first + i * chunk;
    int end = i == n - 1 ? last : start + chunk - 1;

    thread t(
        [&result](int from, int to) {
          for (int i = from; i <= to; i++)
            if (isPrime(i))
              result.push_back(i);
        },
        start, end);
    threads.push_back(move(t));
  }

  for (auto &t : threads)
    t.join();

  return result.size();
}

size_t calcPrimesFixed(int n, int first, int last) {
  vector<thread> threads;
  threads.reserve(n);

  int chunk = (last - first + 1) / n;

  vector<int> result;
  mutex lock;

  for (int i = 0; i < n; i++) {
    int start = first + i * chunk;
    int end = i == n - 1 ? last : start + chunk - 1;

    thread t(
        [&result, &lock](int from, int to) {
          for (int i = from; i <= to; i++)
            if (isPrime(i)) {
              lock_guard locker(lock);
              result.push_back(i);
            }
        },
        start, end);
  }
  threads.push_back(move(t));
}

int main() {
  vector<string> v;
  v.push_back("hello");

  auto &hello = v[0];
  cout << hello.c_str() << endl;

  v.push_back("C++");
  cout << hello.c_str() << endl;

  return 0;
}
