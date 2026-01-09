#include <algorithm>
#include <charconv>
#include <cmath>
#include <iostream>
#include <limits>
#include <print>
#include <string>
#include <string_view>
#include <unordered_map>
#include <utility>
#include <vector>

#ifndef NDEBUG
#  include <format>
#endif

struct Point {
  float x = std::numeric_limits<float>::infinity();
  float y = std::numeric_limits<float>::infinity();

  Point() = default;
  Point(float a, float b) : x{a}, y{b} {};
  Point(std::string_view s) {
    const auto i = s.find(",");
    const auto p = s.substr(0, i);
    const auto q = s.substr(i + 1);
    std::from_chars(p.data(), q.data() + p.size(), x);
    std::from_chars(q.data(), q.data() + q.size(), y);
  }
};

bool operator==(Point a, Point b) {
  return (a.x == b.x) && (a.y == b.y);
}

struct Rect {
  Point p[2];

  float area() const {
    return (1.0f + std::abs(p[0].x - p[1].x)) *
      (1.0f + std::abs(p[0].y - p[1].y));
  }

  auto others() const {
    return std::pair{Point{p[0].x, p[1].y}, Point{p[1].x, p[0].y}};
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

struct Line {
  float left = std::numeric_limits<float>::infinity();
  float right = -std::numeric_limits<float>::infinity();

  bool contains_point(float x) const {
    return (left <= x) && (right >= x);
  }
};

struct Polygon {

  Polygon(const std::vector<Point> points) {
    const auto N = points.size();
    bounds.reserve(N);
    Point prev;
    prev = points.front();
    auto delta = 0.0f;
    for (auto i = 0; i <= N; ++i) {
      auto cur = points[i % N];
      delta = (cur.y > prev.y) ? 1.0f : ((cur.y < prev.y) ? -1.0f : 0.0f);
      for (auto y = prev.y + delta; y != cur.y; y += delta)
        set(Point{cur.x, y});
      set(cur);
      prev = cur;
    }
  }

  bool is_inside(Point p) const {
    const auto it = bounds.find(p.y);
    return (it != bounds.cend()) ? it->second.contains_point(p.x) : false;
  }

private:
  void set(Point p) {
    auto& line = bounds[p.y];
    line.left = std::min(p.x, line.left);
    line.right = std::max(p.x, line.right);
  }

  std::unordered_map<float, Line> bounds;
};

int main() {
  std::ios::sync_with_stdio(false);
  std::cin.tie(nullptr);

  std::vector<Point> reds;
  std::string line;
  while (std::getline(std::cin, line))
    reds.emplace_back(line);
  const auto N = reds.size();

  // Part 1
  float max_area = -std::numeric_limits<float>::infinity();
  for (auto i = 0u; i < N; ++i) {
    for (auto j = i + 1; j < reds.size(); ++j) {
      const float area = Rect{reds[i], reds[j]}.area();
      max_area = std::max(max_area, area);
    }
  }
  std::println("Maximum area rectangle formed with red tiles: {}", max_area);

  Polygon p(reds);

  // A local solution -- assuming if three points are ‘in’ then just testing
  // fourth -- won’t work.  This figure can grow either way: encompassing ? or
  // leaving it out.  Land inlets is the Achilles heel of a local solution.
  //
  //   |
  //   +------O   ?
  //          |   +---
  //          |   |
  //          O---O
  //

  // Iterate every point with 2 preceding and trailing ones.
  float max_area_within = -std::numeric_limits<float>::infinity();
  for (auto i = 0u; i < N; ++i) {
    const auto p1 = reds[i];
    const auto p2 = reds[(i + 1) % N];
    const auto p3 = reds[(i + 2) % N];
    const auto r = Rect{p1, p3};
    auto [p41, p42] = r.others();
    if (p.is_inside((p41 == p2) ? p42 : p41))
      max_area_within = std::max(max_area_within, r.area());
  }
  std::println("Maximum area rectangle formed by red-green tiles: {}",
               max_area_within);
}
