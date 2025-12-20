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

struct Point {
  Point(size_t c = 0, size_t r = 0) : col(c), row(r) { }

  size_t col = 0;
  size_t row = 0;
};

struct Grid {
  // Create grid with boundary on all sides (convolution) for easy mask read.
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

  bool cell(Point p) const {
    return data[(p.row + 1) * columns + (p.col + 1)];
  }

  void clear(Point p) {
    data[(p.row + 1) * columns + (p.col + 1)] = false;
  }

  // https://en.wikipedia.org/wiki/Kernel_(image_processing)
  // Kernel
  // 1 1 1
  // 1 0 1
  // 1 1 1
  uint8_t sum_adjacent_set(Point p) const {
    const auto start1 = (p.row + 0) * columns + p.col;
    const auto start2 = (p.row + 1) * columns + p.col;
    const auto start3 = (p.row + 2) * columns + p.col;
    const auto r1 = std::accumulate(data.begin() + start1,
                                    data.begin() + start1 + 3,
                                    0u);
    const auto r2 = std::accumulate(data.begin() + start2,
                                    data.begin() + start2 + 3,
                                    0u);
    const auto r3 = std::accumulate(data.begin() + start3,
                                    data.begin() + start3 + 3,
                                    0u);
    return r1 + r2 + r3 - cell(p);
  }

private:
  std::vector<bool> data;
  size_t columns = 0u;
};

int main() {
  std::ios::sync_with_stdio(false);
  std::cin.tie(nullptr);

  std::string line;
  std::getline(std::cin, line);
  Grid g(line.size());
  g.add_row(line);
  while (std::getline(std::cin, line))
    g.add_row(line);
  g.finalize();

  std::vector<Point> liftable;
  for (auto r = 0u; r < g.row(); ++r)
    for (auto c = 0u; c < g.col(); ++c)
      if (g.cell(Point(c, r)) && (g.sum_adjacent_set(Point(c, r)) < 4))
        liftable.emplace_back(c, r);
  std::println("Paper rolls accessible by forklifts: {}", liftable.size());

  size_t total_liftable = 0u;
  while (!liftable.empty()) {
    total_liftable += liftable.size();
    for (auto cell : liftable)
      g.clear(cell);
    liftable.clear();

    for (auto r = 0u; r < g.row(); ++r)
      for (auto c = 0u; c < g.col(); ++c)
        if (g.cell(Point(c, r)) && (g.sum_adjacent_set(Point(c, r)) < 4))
          liftable.emplace_back(c, r);
  }
  std::println("All paper rolls accessible by forklifts: {}", total_liftable);
}
