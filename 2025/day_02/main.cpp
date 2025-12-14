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
#include <tuple>

// Returns the number of decimal digits required to represent a number.
unsigned digits(uint64_t number) {
  constexpr uint64_t max_int_in_double =
    1ull << std::numeric_limits<double>::digits;
  assert(number < max_int_in_double);
  return static_cast<unsigned>(std::floor(std::log10(number))) + 1;
}

// Returns |count| least significant decimal digits of |number| along with
// |number| chopped off |count| digits.  Assumes |count| ≤ digits(number);
// undefined otherwise.
auto least_significant(uint64_t number, unsigned count) {
  uint64_t part = 0;
  uint64_t place = 1u;
  for (auto i = 0u; i < count; ++i) {
    const auto value = number % 10;
    part += value * place;
    place *= 10;
    number /= 10;
  }
  return std::make_tuple(part, number);
}

// Returns number of repeats in |number| of an integer sub-sequence.
int has_repeats(uint64_t number) {
  const auto digits = ::digits(number);
  // Cut number into [2, digits] pieces looking for repeats in each set.
  for (auto cuts = 2u; cuts <= digits; ++cuts) {
    if (digits % cuts != 0)
      continue;
    const auto seq_len = digits / cuts;
    auto [seq, rest] = least_significant(number, seq_len);
    for (auto i = 2u; i < cuts; ++i) {
      const auto [new_seq, new_rest] = least_significant(rest, seq_len);
      if (new_seq == seq)
        rest = new_rest;
      else
        break;
    }
    if (rest == seq)
      return cuts;
  }
  return 0;
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
  std::getline(std::cin, input);
  uint64_t invalids_twice = 0u;
  uint64_t invalids = 0u;
  for (const auto interval_range : input | std::views::split(',')) {
    const std::string_view interval(interval_range.cbegin(),
                                    interval_range.cend());
    const auto [a, b] = split(interval);
    const uint64_t left = std::strtoull(a.data(), nullptr, 0);
    const uint64_t right = std::strtoull(b.data(), nullptr, 0);
    for (auto i = left; i <= right; ++i) {
      const auto repeat_count = has_repeats(i);
      invalids_twice += (repeat_count == 2) ? i : 0u;
      invalids += repeat_count ? i : 0u;
    }
  }
  std::println("Sum of all invalids with two repeats: {}", invalids_twice);
  std::println("Sum of all invalids with atleast two repeats: {}", invalids);
}
