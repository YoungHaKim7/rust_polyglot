#include <iostream>
using namespace std;

template <class T> class Harry {
public:
  T data;
  Harry(T a) { data = a; }
  void display();
};

template <class T> void Harry<T>::display() { cout << data; }

void func(int a) {}
