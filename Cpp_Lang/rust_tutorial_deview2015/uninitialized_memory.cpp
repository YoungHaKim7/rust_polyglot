#include <iostream>

struct Point {
  int x;
  int y;
};

using namespace std;

int main(int argc, char *argv[]) {
  Point p;
  cout << p.x << " " << p.y << endl;
  return 0;
}
