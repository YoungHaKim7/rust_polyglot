#include <iostream>

template <typename T> T timestwo(T number) { return number + number; }

struct MyStruct {
  int some_integer;
  float some_float;
}

MyStruct c = MyStruct{0, 3.0};
MyStruct d = timestwo(c);

int main() {
  int a = 2;
  int b = timestwo(a);
  return 0;
}
