#include <iostream>
#include <thread>

void test(int x) {
  std::cout << "Hello from thread ! " << std::endl;
  std::cout << "Argument passed in : " << x << std::endl;
}

int main(int argc, char *argv[]) {
  std::thread myThread(&test, 100);
  myThread.join();
  std::thread myThread1(&test, 100);
  myThread1.join();
  std::thread myThread2(&test, 100);
  myThread2.join();
  std::thread myThread3(&test, 100);
  myThread3.join();
  std::cout << " Hello from my main thread" << std::endl;
  std::cout << " Hello from my main thread" << std::endl;
  std::cout << " Hello from my main thread" << std::endl;
  std::cout << " Hello from my main thread" << std::endl;

  return 0;
}
