#include <mutex>
void print_func(int id) {
  std::lock_guard<std::mutex> g(my_mutex);
  std::cout << "Printing form thread: " << id << '\n';
}

int main() {
  std::thread t0(print_func, 0);
  std::thread t1(print_func, 1);
  std::thread t2(print_func, 2);
  std::thread t3(print_func, 3);

  t0.join();
  t1.join();
  t2.join();
  t3.join();

  return 0;
}
