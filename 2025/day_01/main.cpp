#include <string>
#include <iostream>
#include <cstdint>
#include <cmath>
#include <cstdlib>
#include <print>
#include <concepts>
#include <tuple>

// Knuth’s modulo
// % operator in C (and thereby C++) is actually “integer division remainder”
// not module like in most other languages like Lua, Python, etc.
constexpr
auto kmod(std::floating_point auto a, std::floating_point auto b) {
  const auto quot = std::floor(a / b);
  const auto rem = a - quot * b;
  return std::make_tuple(std::abs(quot), rem);
}

int main() {
  std::ios::sync_with_stdio(false);
  std::string line;
  float start = 50.0f;
  uint32_t zero_stops = 0;
  uint32_t zero_crosses = 0;
  while (std::getline(std::cin, line)) {
    const float dir = (line[0] == 'R') ? 1.0f : -1.0f;
    const float rot = std::strtof(&line[1], nullptr) * dir;
    auto [crosses, stop] = kmod(start + rot, 100.0f);
    // CASES
    //   1. + → +
    //      10 +  20 → 0, false (zero {crosses, stop})
    //      10 + 120 → 1, false
    //      70 -  20 → 0, false
    //   2. + → -
    //      10 -  20 → 1, false
    //      10 - 120 → 2, false
    //   3. + → 0
    //      90 +  10 → 0, true (👊)
    //      90 + 110 → 1, true (👊)
    //      10 -  10 → 0, true
    //      10 - 110 → 1, true
    //   4. 0 → +
    //      0 +  20 → 0, false
    //      0 + 120 → 1, false
    //   5. 0 → -
    //      0 -  10 → 0, false (👊)
    //      0 - 110 → 1, false (👊)
    //   6. 0 → 0
    //      0 + 100 → 0, true (👊)
    //      0 + 200 → 1, true (👊)
    //      0 - 100 → 0, true (👊)
    //      0 - 200 → 1, true (👊)
    // 👊 needs decrement to crosses
    if (crosses > 0) {
      if (((start == 0.0f) && (stop == 0.0f)) ||
          ((start == 0.0f) && (dir == -1.0f)) ||
          ((stop == 0.0f) && (dir == 1.0f)))
          crosses--;
    }
    zero_crosses += static_cast<uint32_t>(crosses);
    zero_stops += (stop == 0.0f);
    start = stop;
  }
  std::println("Needle stopped at zero {} times.", zero_stops);
  std::println("Needle pointed at zero {} times.", zero_stops + zero_crosses);
}
