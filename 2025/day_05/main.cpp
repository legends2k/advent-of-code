#include <algorithm>
#include <charconv>
#include <compare>
#include <cstdint>
#include <format>
#include <iostream>
#include <print>
#include <set>
#include <string>
#include <string_view>
#include <utility>
#include <vector>

struct Interval
{
  Interval(uint64_t x = 0u, uint64_t y = 0u) : left{x}, right{y} {
    // Set |right| to |x| if |y| was omitted.
    if (right == 0u)
      right = x;
    if (left > right)
      std::swap(left, right);
  }

  enum Orientation {
    Disjoint,
    Touch,
    Overlap,
    Contain, // parent
    Inside,  // child
  };

  // Returns orientation with respect to |other|.  Commutative.
  constexpr Orientation wrt(const Interval& other) const {
    if ((left > other.right) || (right < other.left))
      return Orientation::Disjoint;
    else if (contains(other.left) && contains(other.right))
      return Orientation::Contain;
    else if (other.contains(left) && other.contains(right))
      return Orientation::Inside;
    else if ((left == other.right) || (right == other.left))
      return Orientation::Touch;
    return Orientation::Overlap;
  }

  // Returns true if |x| ∈ [left, right].
  constexpr bool contains(uint64_t x) const {
    return (x >= left) && (x <= right);
  }

  // Returns length of the contained interval.
  constexpr uint64_t length() const {
    return right - left + 1u;
  }

  // Returns true if length == 1.
  constexpr bool is_degenrate() const { return left == right; }

  // Order prioritizing |left|, tie break with length.
  // If length were a data member, just “ = default” would be enough.
  constexpr std::strong_ordering operator<=>(const Interval& other) const {
    if (auto cmp = left <=> other.left; cmp != 0)
      return cmp;
    // Trivially-constructed, degenerate intervals should order after equivalent
    // original interval with same |left|.
    if (is_degenrate())
      return std::strong_ordering::greater;
    else if (other.is_degenrate())
      return std::strong_ordering::less;
    return length() <=> other.length();
  }

  constexpr bool operator==(const Interval& other) const = default;

  uint64_t left;
  uint64_t right;
};

// Interval to string (for debugging).
template<>
struct std::formatter<Interval> {
  constexpr auto parse(std::format_parse_context& ctx) {
    return ctx.begin();
  }

  template <typename FormatContext>
  auto format(const Interval& i, FormatContext& ctx) const {
    return std::format_to(ctx.out(), "[{}, {}]", i.left, i.right);
  }
};

struct IntervalSet {
  void add(Interval k) {
    // We’ve a few possibilities: |k| is
    //   1. Disjoint (∅)
    //   2. Contained (1)
    //   3. Touch (2)
    //   4. Overlap (n)
    // Use set::upper_bound as it leads to lesser cases (only > not ≥).
    auto it = m_intervals.upper_bound(k.left);
    if (auto it_begin = m_intervals.cbegin(); it == it_begin) {
      if (k.contains(it_begin->left) || ((k.right + 1) == it_begin->left)) {
        k.right = it_begin->right;
        m_intervals.erase(it_begin);
      }
      m_intervals.insert(k);
    }
    else {
      --it;
      std::vector<decltype(it)> overlapping;
      while (it != m_intervals.cend() && it->left <= k.right) {
        const auto orientation = it->wrt(k);
        if (orientation == Interval::Orientation::Contain)
          return;
        else if ((orientation != Interval::Orientation::Disjoint) ||
                 ((it->right + 1) == k.left) || ((k.right + 1) == it->left))
          overlapping.push_back(it);
        ++it;
      }
      if (!overlapping.empty()) {
        k.left = std::min(overlapping.front()->left, k.left);
        k.right = std::max(overlapping.back()->right, k.right);
      }
      for (auto i : overlapping)
        m_intervals.erase(i);
      m_intervals.insert(k);
    }
  }

  bool is_present(uint64_t x) const {
    if (auto it = m_intervals.upper_bound(x); it != m_intervals.cbegin()) {
      --it;
      return it->contains(x);
    }
    return false;
  }

private:
  std::set<Interval> m_intervals;
};

int main() {
  std::ios::sync_with_stdio(false);
  std::cin.tie(nullptr);

  IntervalSet s;
  std::string line;
  bool parsing_intervals = true;
  uint64_t total_fresh = 0u;
  while (std::getline(std::cin, line)) {
    if (line.empty())
      parsing_intervals = false;
    else if (parsing_intervals) {
      const auto hyphen = line.find('-');
      const std::string_view left(line.cbegin(), line.cbegin() + hyphen);
      const std::string_view right(line.cbegin() + hyphen + 1, line.cend());
      uint64_t x = 0u, y = 0u;
      std::from_chars(left.data(), left.data() + left.size(), x);
      std::from_chars(right.data(), right.data() + right.size(), y);
      s.add(Interval(x, y));
    }
    else {
      uint64_t x;
      std::from_chars(line.data(), line.data() + line.size(), x);
      total_fresh += s.is_present(x);
    }
  }
  std::println("Total fresh ingredients: {}", total_fresh);
}
