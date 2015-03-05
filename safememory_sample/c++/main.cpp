#include <iostream>
#include <vector>
#include <string>

int main() {
  std::vector<std::string> v;
  
  v.push_back("hello");
  std::string& x = v[0];
  
  v.push_back("world");
  
  std::cout << x << std::endl;
}