#include <iostream>
#include <string>
#include <string_view>
#include <print>
#include <cstdint>
#include <ranges>
#include <utility>

// Returns the two maximum digits in a number
uint8_t max_joltage(std::string_view bank) {
  auto tens = bank[0];
  auto units = bank[1];
  for (const char digit : bank | std::views::drop(2)) {
    if (units > tens) {
      std::swap(units, tens);
      units = digit;
    }
    else if (digit > units)
      units = digit;
  }
  return (10 * (tens - '0')) + (units - '0');
}

int main() {
  std::ios::sync_with_stdio(false);
  std::cin.tie(nullptr);

  std::string line;
  uint32_t sum_bank_jolts = 0u;
  while (std::getline(std::cin, line)) {
    sum_bank_jolts += max_joltage(line);
  }
  std::println("Total output joltage: {}", sum_bank_jolts);
}
