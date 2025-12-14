#include <iostream>
#include <string>
#include <string_view>
#include <print>
#include <cstdint>
#include <iterator>
#include <algorithm>
#include <cassert>

// Returns |batteries| digits in |bank| in sequence with maximum value.
uint64_t max_joltage(std::string_view bank, int8_t batteries) {
  const int8_t N = bank.size();
  assert(batteries <= N);
  int8_t prev = -1;
  uint64_t result = 0u;
  while (batteries--) {
    const auto start = prev + 1;
    const auto window_size = N - start - batteries;
    assert(window_size);
    const auto look_window = bank.substr(start, window_size);
    auto it = std::ranges::max_element(look_window);
    result = result * 10u + (*it - '0');
    prev = start + std::distance(look_window.cbegin(), it);
  }
  return result;
}

int main() {
  std::ios::sync_with_stdio(false);
  std::cin.tie(nullptr);

  std::string line;
  uint64_t sum_jolts_2b = 0u;
  uint64_t sum_jolts_12b = 0u;
  while (std::getline(std::cin, line)) {
    const auto jolt_2 = max_joltage(line, 2);
    const auto jolt_12 = max_joltage(line, 12);
    sum_jolts_2b += jolt_2;
    sum_jolts_12b += jolt_12;
  }
  std::println("Total output joltage (2 batteries): {}", sum_jolts_2b);
  std::println("Total output joltage (12 batteries): {}", sum_jolts_12b);
}
