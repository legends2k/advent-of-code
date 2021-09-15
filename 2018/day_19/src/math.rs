pub mod prime {
  use std::iter;

  /// Extracts all f-ness is n
  /// e.g. extract all evenness in n by passing `(n, 2)`
  fn extract(n: &mut u64, f: u64) -> u32 {
    let mut count = 0;
    while *n % f == 0 {
      *n /= f;
      count += 1;
    }
    count
  }

  // Basic Idea is to iterate over (2..=n), test for clean divisibility, divide
  // and append factor.  However, once factor 2 iteration is done, all further
  // divisiors/factors will be odd, so idx f can be double incremented.  Special
  // case 2 and do regular iteration from 3 onwards.  Note: we don’t check if f is
  // prime since we’re iterating from 2, we’d’ve already tried to divide n by
  // prime factors some composite/non-prime number is composed of.
  //
  // REFERENCES
  //     https://en.wikipedia.org/wiki/Integer_factorization
  //     https://en.wikipedia.org/wiki/Trial_division
  //     https://www.geeksforgeeks.org/print-all-prime-factors-of-a-given-number/
  /// Returns prime factors of `n`.
  pub fn factors(n: u64) -> Vec<u64> {
    let mut m = n;
    let i = extract(&mut m, 2);
    let mut v = vec![2; i as usize];
    let mut f = 3;
    // Optimisation 1: run only till √n as factors of a number can’t be
    // greater. Anything pending will be a prime number.
    // https://stackoverflow.com/q/5811151/183120
    while (f * f) <= n {
      let i = extract(&mut m, f);
      v.extend(iter::repeat(f).take(i as usize));
      f += 2;
      // Optimisation 2: skip even numbers since all evenness
      // has been extracted from n.
    }
    // we’ve a prime number pending
    if m != 1 {
      v.push(m);
    }
    v
  }
}

// Refer itertools::cartesian_product for an iterator-based solution
/// Returns cartesian product of `a` and `b` by computing it with `f`.
fn cartesian_product<P, R>(a: &[u64], b: &[u64], f: P) -> Vec<R>
where
  P: Fn(u64, u64) -> R,
{
  let mut products = Vec::with_capacity(a.len() * b.len());
  for &x in a {
    for &y in b {
      products.push(f(x, y));
    }
  }
  products
}

// Deducing factors of n from its prime factors; not doing the brute
// force approach of iterating over 1..=n and checking n % i == 0
// https://math.stackexchange.com/q/2782625/51968
// https://stackoverflow.com/a/29451566/183120
/// Returns all factors of `n`.
pub fn factors(n: u64) -> Vec<u64> {
  let primes = prime::factors(n);
  // prime::factors(90) = 2 * 3 * 3 * 5
  // seeds = 1, 2, 1, 3, 9, 1, 5; 3-tuple cartesean product gives result
  // result: 1, 2, 3, 5, 6, 9, 10, 15, 18, 30, 45, 90
  let mut seeds = Vec::with_capacity(primes.len() * 2);
  // `indices` are slices of `seeds` for each prime factor
  let mut indices = Vec::with_capacity(primes.len());
  let mut prime = 0;
  let mut prev = 0;
  for p in primes {
    if prime != p {
      prime = p;
      seeds.push(1);
      // bookmark the prime ranges in seeds
      if seeds.len() != 1 {
        indices.push((prev, seeds.len() - 1));
        prev = seeds.len() - 1;
      }
    }
    let t = seeds.last().unwrap() * p;
    seeds.push(t);
  }
  // there’s only one unique prime factor
  if indices.is_empty() {
    return seeds;
  }
  indices.push((prev, seeds.len()));
  let ranges = indices
    .iter()
    .map(|&(i, j)| &seeds[i..j])
    .collect::<Vec<_>>();
  // Total factors needed = ranges.iter().fold(1, |acc, &r| acc * r.len())

  let productor = |x, y| x * y;
  let mut factors = cartesian_product(ranges[0], ranges[1], productor);
  for &r in &ranges[2..] {
    factors = cartesian_product(&factors, r, productor);
  }
  factors
}
