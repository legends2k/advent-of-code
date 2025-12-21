#include <charconv>
#include <cstdint>
#include <cstdio>
#include <iostream>
#include <numeric>
#include <print>
#include <ranges>
#include <string>
#include <utility>
#include <vector>

enum Operation {
  Multiply,
  Add
};

struct Offset {
  size_t left;
  size_t right;
};

std::vector<uint64_t>
get_field(const std::vector<char>& table,
          const std::vector<std::pair<Operation, Offset>>& ops,
          size_t columns,
          size_t f) {
  // Read the last column for field width.
  const auto rows = table.size() / columns;
  const auto& left = ops[f].second.left;
  const auto& right = ops[f].second.right;
  const auto width = right - left;
  std::vector<uint64_t> numbers;
  numbers.reserve(rows);
  for (auto i = 0u; i < rows; ++i) {
    auto offset = i * columns + left;
    const auto end = table.data() + offset + width;
    // Skip preceding spaces if any as from_chars chokes on those!
    while (table[offset] == ' ')
      offset++;
    uint64_t number = 0;
    // TODO: check return value.
    std::from_chars(&table[offset], end, number);
    numbers.push_back(number);
  }
  return std::move(numbers);
}

std::vector<uint64_t>
get_field_transpose(const std::vector<char>& table,
                    const std::vector<std::pair<Operation, Offset>>& ops,
                    size_t columns,
                    size_t f) {
  // Read the last column for field width.
  const auto rows = table.size() / columns;
  const auto& left = ops[f].second.left;
  const auto& right = ops[f].second.right;
  const auto width = right - left;
  std::vector<uint64_t> numbers;
  numbers.reserve(width);
  std::string num;
  num.reserve(10);
  for (auto c = left; c < right; ++c) {
    num.clear();
    for (auto r = 0u; r < rows; ++r)
      if (auto ch  = table[r * columns + c]; ch != ' ')
        num.push_back(ch);
    numbers.push_back(std::stoull(num));
  }
  return std::move(numbers);
}

int main() {
  std::ios::sync_with_stdio(false);
  std::cin.tie(nullptr);

  std::string line;
  std::getline(std::cin, line);
  const uint16_t columns = line.size();
  // Store numbers column-wise.
  std::vector<char> table;
  // Preallocate assuming 10 lines.
  table.reserve(columns * 10u);
  table.insert(table.cend(), line.cbegin(), line.cend());
  while (std::getline(std::cin, line)) {
    table.insert(table.cend(), line.cbegin(), line.cend());
    if (line.size() != columns) {
      std::println(stderr, "Error: Invalid input.");
      return -1;
    }
  }
  const auto size_sans_ops = table.size() - columns;
  const auto rows = (table.size() / columns) - 1u;
  std::vector<std::pair<Operation, Offset>> ops;
  for (auto i = (rows * columns); i < table.size(); ++i) {
    if (table[i] != ' ')
      ops.push_back(
        std::make_pair(Operation(table[i] == '+'),
                       Offset{i - size_sans_ops, i - size_sans_ops}));
    else
      ops.back().second.right++;
  }
  // Last operation doesn’t have a border space.
  ops.back().second.right++;
  // Truncate last row with just operators.
  table.resize(size_sans_ops);

  // Part 1
  uint64_t result1 = 0u;
  for (auto i = 0u; i < ops.size(); ++i) {
    const auto numbers = get_field(table, ops, columns, i);
    if (ops[i].first == Operation::Multiply)
      result1 += std::accumulate(numbers.cbegin(),
                                 numbers.cend(),
                                 uint64_t(1u),
                                 [](auto b, auto i) { return b * i; });
    else
      result1 += std::accumulate(numbers.cbegin(),
                                 numbers.cend(),
                                 uint64_t(0u),
                                 [](auto b, auto i) { return b + i; });
  }
  std::println("Total: {}", result1);

  // Part 2: transpose
  uint64_t result2 = 0u;
  for (auto i = 0u; i < ops.size(); ++i) {
    const auto numbers = get_field_transpose(table, ops, columns, i);
    if (ops[i].first == Operation::Multiply)
      result2 += std::accumulate(numbers.cbegin(),
                                 numbers.cend(),
                                 uint64_t(1u),
                                 [](auto b, auto i) { return b * i; });
    else
      result2 += std::accumulate(numbers.cbegin(),
                                 numbers.cend(),
                                 uint64_t(0u),
                                 [](auto b, auto i) { return b + i; });
  }
  std::println("Total Transposed: {}", result2);
}
