#include <cstdint>
#include <cstdio>
#include <iostream>
#include <print>
#include <string>
#include <utility>
#include <vector>
#include <algorithm>
#include <iterator>

struct Simulation {
  Simulation(std::vector<char> manifold, size_t width)
    : manifold(std::move(manifold))
    , width(width) {
    // TODO: throw an exception if manifold.size() % width != 0.
  }

  // Returns the number of Tachyon beam splits.
  size_t run() const {
    // Copy manifold as we want it clean for other runs.
    std::vector<char> m{manifold};
    std::vector<size_t> prev, next;  // store only X
    prev.reserve(width);
    next.reserve(width);
    prev.push_back(get_start().value());
    size_t splits = 0u;
    for (auto y = 1u; y < height(); ++y) {
      for (const auto x: prev) {
        switch (get(m, x, y)) {
        case '.':
          set(&m, x, y, '|');
          next.push_back(x);
          break;
        case '^':
          set(&m, x - 1, y, '|');
          set(&m, x + 1, y, '|');
          next.push_back(x - 1);
          next.push_back(x + 1);
          splits++;
        }
      }
      prev.clear();
      std::swap(prev, next);
    }
    return splits;
  }

  size_t height() const {
    return manifold.size() / width;
  }

  void set(std::vector<char>* m, size_t x, size_t y, char c) const {
    const auto offset = width * y + x;
    (*m)[offset] = c;
  }

  char get(const std::vector<char>& m, size_t x, size_t y) const {
    const auto offset = width * y + x;
    return m[offset];
  }

  std::optional<size_t> get_start() const {
    const auto it = std::find(manifold.cbegin(),
                              manifold.cbegin() + width,
                              'S');
    if (*it != 'S')
      return {};
    return std::distance(manifold.cbegin(), it);
  }

  friend struct std::formatter<Simulation>;

private:
  const std::vector<char> manifold;
  const size_t width;
};

// Simulation manifold to string (for debugging).
template<>
struct std::formatter<Simulation> {
  constexpr auto parse(std::format_parse_context& ctx) {
    return ctx.begin();
  }

  template <typename FormatContext>
  auto format(const Simulation& s, FormatContext& ctx) const {
    auto out = ctx.out();
    for (auto y = 0u; y < s.height(); ++y) {
      const char* row = s.manifold.data() + y * s.width;
      out = std::format_to(out, "{:.{}}", row, s.width);
      if (y + 1 < s.height())
        *out++ = '\n';
    }
    return out;
  }
};

int main() {
  std::ios::sync_with_stdio(false);
  std::cin.tie(nullptr);

  std::string line;
  std::getline(std::cin, line);
  const uint16_t columns = line.size();
  std::vector<char> manifold;
  manifold.reserve(columns * 150);
  manifold.insert(manifold.cend(), line.cbegin(), line.cend());
  while (std::getline(std::cin, line)) {
    if (line.size() != columns) {
      std::println(stderr, "Error: invalid input.");
      return -1;
    }
    manifold.insert(manifold.cend(), line.cbegin(), line.cend());
  }
  Simulation sim{std::move(manifold), columns};
  std::println("Tachyon Beam Splits: {}", sim.run());
}
