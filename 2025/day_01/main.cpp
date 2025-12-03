#include <string>
#include <iostream>
#include <cstdint>
#include <print>
#include <type_traits>

// Knuth’s modulo
// % operator in C (and thereby C++) is actually “integer division remainder”
// not module like in most other languages like Lua, Python, etc.
constexpr
auto kmod(std::integral auto a, std::integral auto b) {
  return ((a % b) + b) % b;
}

int main() {
  std::ios::sync_with_stdio(false);
  std::string line;
  int8_t needle = 50;
  uint32_t zero_stops = 0;
  while (std::getline(std::cin, line)) {
    const int dir = (line[0] == 'R') ? 1 : -1;
    const long rot = std::strtol(&line[1], nullptr, 0) * dir;
    needle = kmod((needle + rot), 100);
    zero_stops += (needle == 0);
  }
  std::println("Needle stopped at zero {} times.", zero_stops);
}
