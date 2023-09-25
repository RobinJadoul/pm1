# Pollard's p - 1 algorithm for factorization

Written in rust, using [pyo3](https://pyo3.rs) to provide python bindings and [primesieve](https://github.com/kimwalisch/primesieve) for fast prime enumeration.
`libprimesieve.so` and `primesieve.hpp` are required for compilation and running, and assumed to be present on the machine.

The `pm1` function provides a factorization with a slightly unusual interface, see below, and `pm1base` is the same thing, but where you can specify the base `a` of the exponentiation `pow(a, M, n)`.
Additionally `pm1_factorbase` allows you to specify a a custom factor base for the algorithm, rather than the default choice of primes below `2^n`.

The interface provided by `pm1` and `pm1base` will compute $x = a^{M_1 \cdot M_2} \mod n$ and test whether $\gcd(x, n)$ reveals a factor of $n$.
Here, $M_0 = \prod_{p \le B_1} p^{\mathrm{exp}}$ and $M_1 = \prod_{B_1 \le p \le B_2} p$.
This choice was made for the cases where small prime factors are more likely to have multiple occurences.
More control can be achieved with the python bindings and repeated use of the `pm1_factorbase` approach.
The classical extension of a second bound between under which only a single prime factor can be tolerated is currently not implemented.


A rust-driven CLI tool is provided that can perform the `pm1base` functionality.
If you want to compile the CLI only, without python bindings being involved, use the `--no-default-features` flag for cargo.
