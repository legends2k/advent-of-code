#include <iostream>
#include <print>
#include <string>
#include <string_view>
#include <ranges>
#include <cstdlib>
#include <cstdint>
#include <cmath>
#include <cassert>
#include <iterator>
#include <utility>
#include <limits>

// Returns the number of decimal digits required to represent a number.
unsigned digits(uint64_t number) {
  constexpr uint64_t max_int_in_double =
    1ull << std::numeric_limits<double>::digits;
  assert(number < max_int_in_double);
  return static_cast<unsigned>(std::floor(std::log10(number))) + 1;
}

// Returns true if a number is made of two repeats of an integer sequence.
bool has_repeat(uint64_t num, unsigned digits) {
  // Cut the number into two halves and return true if they’re equal.
  digits /= 2;
  uint64_t half = 0;
  uint64_t place = 1u;
  for (auto i = 0u; i < digits; ++i) {
    const auto value = num % 10;
    half += value * place;
    place *= 10;
    num /= 10;
  }
  return num == half;
}

// Returns two numbers as string views assuming the input is
// of format "num1-num2".
auto split(std::string_view interval) {
  const auto pos = interval.find('-');
  assert (pos != std::string_view::npos);
  return std::make_pair(interval.substr(0, pos), interval.substr(pos + 1));
}

int main() {
  std::ios::sync_with_stdio(false);
  std::cin.tie(nullptr);

  std::string input;
  std::getline(std::cin, input, '\0');
  input.pop_back();
  uint64_t invalids = 0;
  for (const auto interval_range : input | std::views::split(',')) {
    const std::string_view interval(interval_range.cbegin(),
                                    interval_range.cend());
    const auto [a, b] = split(interval);
    const uint64_t left = std::strtoull(a.data(), nullptr, 0);
    const uint64_t right = std::strtoull(b.data(), nullptr, 0);
    for (auto i = left; i <= right; ++i) {
      const auto d = digits(i);
      if ((d % 2) != 0)
        continue;
      invalids += has_repeat(i, d) ? i : 0u;
    }
  }
  std::println("Sum of all invalids: {}", invalids);
}
