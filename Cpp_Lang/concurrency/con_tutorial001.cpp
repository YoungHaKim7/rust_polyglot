// g++ -std=c++17 con_tutorial001.cpp -o prog -lpthread
#include <iostream>
#include <thread>

void test(int x) {
  std::cout << "Hello from thread!" << std::endl;
  std::cout << "Argument passed in : " << x << std::endl;
}

int main() {

  std::thread myThread(&test, 100);
  myThread.join();

  std::cout << "hello from my main thread " << std::endl;
  return 0;
}
