#include <iostream>

using namespace std;

int main() {
  int *intPtr = new int;
  *intPtr = 7;
  cout << *intPtr << endl;

  // done with what intPtr points to...
  // forget to free it!!!

  return 0;
}
