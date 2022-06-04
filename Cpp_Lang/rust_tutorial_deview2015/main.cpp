// C++  무효화
#include <iostream>
#include <map>

using namespace std;

int main() {
  map<string, string> m;
  m["kr"] = "Korea";
  auto item = m.find("kr"); // m 의 내부를 참조
  m.clear();                // 변경으로 참조가 무효화
  if (item != m.end()) {
    cout << item->first << " = " << item->second << endl;
  }
}
