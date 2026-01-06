#include <algorithm>
#include <charconv>
#include <cmath>
#include <iostream>
#include <limits>
#include <print>
#include <string>
#include <string_view>
#include <vector>

#ifndef NDEBUG
#  include <format>
#endif

struct Point {
  float x = std::numeric_limits<float>::infinity();
  float y = std::numeric_limits<float>::infinity();

  Point(std::string_view s) {
    const auto i = s.find(",");
    const auto p = s.substr(0, i);
    const auto q = s.substr(i + 1);
    std::from_chars(p.data(), q.data() + p.size(), x);
    std::from_chars(q.data(), q.data() + q.size(), y);
  }
};

struct Rect {
  Point p[2];
  float area() const {
    return (1.0f + std::abs(p[0].x - p[1].x)) *
      (1.0f + std::abs(p[0].y - p[1].y));
  }
};

#ifndef NDEBUG

template<>
struct std::formatter<Point> {
  constexpr auto parse(std::format_parse_context& ctx) {
    return ctx.begin();
  }

  template <typename FormatContext>
  auto format(const Point& p, FormatContext& ctx) const {
    auto out = ctx.out();
    out = std::format_to(out, "{},{}", p.x, p.y);
    return out;
  }
};

#endif // NDEBUG

int main() {
  std::ios::sync_with_stdio(false);
  std::cin.tie(nullptr);

  std::vector<Point> reds;
  std::string line;
  while (std::getline(std::cin, line))
    reds.emplace_back(line);

  float max_area = -std::numeric_limits<float>::infinity();
  for (auto i = 0u; i < reds.size(); ++i) {
    for (auto j = i + 1; j < reds.size(); ++j) {
      const float area = Rect{reds[i], reds[j]}.area();
      max_area = std::max(max_area, area);
    }
  }
  std::println("Maximum area rectangle formed with red tiles: {}", max_area);
}
