#include <algorithm>
#include <concepts>
#include <cstdint>
#include <cstdio>
#include <iostream>
#include <iterator>
#include <limits>
#include <numeric>
#include <optional>
#include <print>
#include <string>
#include <utility>
#include <vector>

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

  // Returns the number of worlds the beam entered.
  size_t run_many_worlds() const {
    using uint = uint64_t;
    constexpr auto DOT = std::numeric_limits<uint>::min();
    constexpr auto CAP = std::numeric_limits<uint>::max();
    std::vector<uint> m(manifold.size(), DOT);
    // Transform-Copy to u64 skipping the first line with 'S'.
    std::transform(manifold.cbegin() + width,
                   manifold.cend(),
                   m.begin() + width,
                   [](char c) { return (c == '^') ? CAP : DOT; });
    std::vector<size_t> prev, next;  // store only X
    prev.reserve(width);
    next.reserve(width);
    const auto start = get_start().value();
    set(&m, start /*x*/, 0u /*y*/, uint(1));
    prev.push_back(start);
    for (auto y = 1u; y < height(); ++y) {
      for (const auto x: prev) {
        const auto beams_in = get(m, x, y - 1u);
        const auto beams = get(m, x, y);
        switch (beams) {
        case DOT:
        {
          set(&m, x, y, beams_in);
          next.push_back(x);
        }
        break;
        case CAP:
        {
          const auto beams_left = get(m, x - 1, y);
          const auto beams_right = get(m, x + 1, y);
          set(&m, x - 1, y, beams_left + beams_in);
          set(&m, x + 1, y, beams_right + beams_in);
          if (!beams_left)
            next.push_back(x - 1);
          if (!beams_right)
            next.push_back(x + 1);
        }
        break;
        default:
          set(&m, x, y, beams + beams_in);
        }
      }
      prev.clear();
      std::swap(prev, next);
    }

#ifdef DEBUG_PRINT
    for (auto r = 0u; r < height(); ++r) {
      for (auto c = 0u; c < width; ++c) {
        const auto value = get(m, c, r);
        if (value == CAP)
          std::print("^");
        else if (value == DOT)
          std::print(".");
        else
          std::print("{}", value);
      }
      std::println();
    }
#endif

    return std::accumulate(m.cbegin() + (width * (height() - 1u)),
                           m.cend(),
                           uint(0));
  }

  size_t height() const {
    return manifold.size() / width;
  }

  template <std::integral T>
  void set(std::vector<T>* m, size_t x, size_t y, T c) const {
    const auto offset = width * y + x;
    (*m)[offset] = c;
  }

  template <std::integral T>
  T get(const std::vector<T>& m, size_t x, size_t y) const {
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
  std::println("Tachyon Worlds: {}", sim.run_many_worlds());
}
