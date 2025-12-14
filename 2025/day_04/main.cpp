#include <iostream>
#include <cstdint>
#include <cstdlib>
#include <string>
#include <string_view>
#include <vector>
#include <algorithm>
#include <numeric>
#include <iterator>
#include <print>
#include <cassert>

struct Grid {
  Grid(size_t cols) : columns(cols + 2) {
    data.assign(columns, false);
  }

  void add_row(std::string_view line) {
    assert(line.size() == (columns - 2));
    data.push_back(false);
    std::transform(line.cbegin(), line.cend(), std::back_inserter(data),
                   [](char c) { return c == '@'; });
    data.push_back(false);
  }

  void finalize() {
    data.insert(data.cend(), columns, false);
  }

  size_t col() const { return columns - 2; }
  size_t row() const { return (data.size() / columns) - 2; }

  bool cell(size_t c, size_t r) const {
    return data[(r + 1) * columns + (c + 1)];
  }

  // https://en.wikipedia.org/wiki/Kernel_(image_processing)
  // Kernel
  // 1 1 1
  // 1 0 1
  // 1 1 1
  uint8_t sum_adjacent_set(size_t x, size_t y) const {
    const auto start1 = (y + 0) * columns + x;
    const auto start2 = (y + 1) * columns + x;
    const auto start3 = (y + 2) * columns + x;
    const auto r1 = std::accumulate(data.begin() + start1,
                                    data.begin() + start1 + 3,
                                    0u);
    const auto r2 = std::accumulate(data.begin() + start2,
                                    data.begin() + start2 + 3,
                                    0u);
    const auto r3 = std::accumulate(data.begin() + start3,
                                    data.begin() + start3 + 3,
                                    0u);
    return r1 + r2 + r3 - cell(x, y);
  }

private:
  std::vector<bool> data;
  size_t columns = 0u;
};

int main() {
  std::ios::sync_with_stdio(false);
  std::cin.tie(nullptr);

  // Create grid with boundary on all sides (convolution) for easy mask read.
  std::string line;
  std::getline(std::cin, line);
  Grid g(line.size());
  g.add_row(line);
  while (std::getline(std::cin, line))
    g.add_row(line);
  g.finalize();

  uint32_t liftable_rolls = 0u;
  for (auto r = 0u; r < g.row(); ++r)
    for (auto c = 0u; c < g.col(); ++c)
      liftable_rolls += g.cell(c, r) && (g.sum_adjacent_set(c, r) < 4);

  std::println("Paperrolls accessible by forklifts: {}", liftable_rolls);
}
