#include <algorithm>
#include <charconv>
#include <cmath>
#include <cstdint>
#include <cstdio>
#include <format>
#include <functional>
#include <iostream>
#include <iterator>
#include <limits>
#include <unordered_map>
#include <print>
#include <queue>
#include <ranges>
#include <string>
#include <string_view>
#include <system_error>
#include <utility>
#include <vector>

#ifdef __GNUG__
#  define ALIGN(x) __attribute__ ((aligned(x)))
#elif defined(_MSC_VER)
#  define ALIGN(x) __declspec(align(x))
#else
#  error "Unknown compiler; can't set alignment attribute!"
#endif

struct ALIGN(16) Point {
  float x = 0.f;
  float y = 0.f;
  float z = 0.f;
  float w = 0.f;
};

// Returns the vector from point |a| to |b|.
Point vector(Point from, Point to) {
  return Point{to.x - from.x, to.y - from.y, to.z - from.z, to.w - from.w};
}

// Returns length of |p| (treating it as a vector).
float length(Point p) {
  return std::sqrt((p.x * p.x) + (p.y * p.y) + (p.z * p.z) + (p.w * p.w));
}

// Returns Euclidean distance between |a| and |b|.
float distance(Point a, Point b) {
  return length(vector(a, b));
}

template<>
struct std::formatter<Point> {
  constexpr auto parse(std::format_parse_context& ctx) {
    return ctx.begin();
  }

  template <typename FormatContext>
  auto format(const Point& p, FormatContext& ctx) const {
    auto out = ctx.out();
    out = std::format_to(out, "{},{},{}", p.x, p.y, p.z);
    return out;
  }
};

using Box = uint16_t;
using Circuit = uint16_t;
constexpr Box INVALID_BOX = std::numeric_limits<Box>::max();

struct Connection {
  std::pair<Box, Box> boxes{INVALID_BOX, INVALID_BOX};
  float distance = std::numeric_limits<float>::infinity();
};

bool operator>(const Connection& a, const Connection& b) {
  return a.distance > b.distance;
}

std::vector<Connection>
calculate_distances(const std::vector<Point>& positions) {
  const size_t n = positions.size();
  // Out of an n × n table only the right-top triangle is filled.
  // Summation is n(n+1)/2 but reduce one as we skip self tests.
  std::vector<Connection> c;
  c.reserve(n * (n - 1u) / 2);
  for (Box i = 0u; i < n; ++i) {
    for (Box j = i + 1; j < n; ++j) {
      const float d = distance(positions[i], positions[j]);
      c.push_back({{i, j}, d});
    }
  }
  return c;
}

int main() {
  std::ios::sync_with_stdio(false);
  std::cin.tie(nullptr);

  std::vector<Point> positions;
  positions.reserve(1000);
  std::string line;
  while (std::getline(std::cin, line)) {
    float f[4]{};
    auto i = 0u;
    for (auto&& coord : line | std::views::split(',')) {
      if (i > 2) {
        std::println(stderr, "Invalid input (too many coordinates): {}", line);
        return -1;
      }
      const std::string_view c(coord.cbegin(), coord.cend());
      const auto [_, e] = std::from_chars(c.data(), c.data() + c.size(), f[i]);
      if (e != std::errc{}) {
        std::println(stderr, "Invalid input (non-numeric): {}", line);
        return -1;
      }
      ++i;
    }
    if (i != 3) {
      std::println(stderr, "Invalid input (too few coordinates): {}", line);
      return -1;
    }
    positions.push_back(Point{f[0], f[1], f[2]});
  }

  std::priority_queue<Connection, std::vector<Connection>, std::greater<>> pq{
    std::greater<>{}, calculate_distances(positions)
  };
  Circuit next_circuit = 0u;
  std::unordered_map<Box, Circuit> box2circuit;
  std::unordered_map<Circuit, std::vector<Box>> circuit2box;
  Connection unify;
  uint32_t i = 0u;
  size_t top[4] = {};
  while (!pq.empty()) {
    const auto p = pq.top();
    pq.pop();
    ++i;
    auto it1 = box2circuit.find(p.boxes.first);
    auto it2 = box2circuit.find(p.boxes.second);
    if ((it1 != box2circuit.cend()) && (it2 != box2circuit.cend())) {
      if (it1->second == it2->second)
        continue;
      // Merge circuits as two boxes from different circuits are close.
      // Copy circuit IDs; box2circuit[b] assignment will invalidate `it2`.
      const Circuit from = it2->second;
      const Circuit to = it1->second;
      for (auto b : circuit2box[from])
        box2circuit[b] = to;
      auto it3 = circuit2box.find(from);
      circuit2box[to].append_range(std::move(it3->second));
      circuit2box.erase(it3);
    }
    // Create a new circuit; both boxes don’t belong in one.
    else if (it1 == box2circuit.cend() && it2 == box2circuit.cend()) {
      box2circuit[p.boxes.first] = next_circuit;
      box2circuit[p.boxes.second] = next_circuit;
      circuit2box[next_circuit].push_back(p.boxes.first);
      circuit2box[next_circuit].push_back(p.boxes.second);
      next_circuit++;
    }
    else if (it1 != box2circuit.cend()) {
      box2circuit[p.boxes.second] = it1->second;
      circuit2box[it1->second].push_back(p.boxes.second);
    }
    else /*if (it2 != box2circuit.cend())*/ {
      box2circuit[p.boxes.first] = it2->second;
      circuit2box[it2->second].push_back(p.boxes.first);
    }

    // Part 1: sort out top 3 populated circuits at threshold.
    // NOTE: Set this to 10 for sample input
    if (i == 1000) {
      for (const auto& [circuit, boxes] : circuit2box) {
        if (top[3] < boxes.size()) {
          top[3] = boxes.size();
          std::sort(std::begin(top), std::end(top), std::greater{});
        }
      }
    }

    // Part 2: terminate loop if we just made the final unifying connection.
    if (auto it3 = circuit2box.cbegin();
        ((it3 != circuit2box.cend()) &&
         (it3->second.size() == positions.size()))) {
      unify = p;
      break;
    }
  }

  std::println("Product of sizes of the three largest circuits: {}",
               top[0] * top[1] * top[2]);
  const auto x1 = positions[unify.boxes.first].x;
  const auto x2 = positions[unify.boxes.second].x;
  std::println("Product of unifying boxes' X coordinates: {}", x1 * x2);
}
