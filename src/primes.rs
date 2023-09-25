use autocxx::WithinUniquePtr;
#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

autocxx::include_cpp! {
    #include "primesieve.hpp"
    generate!("primesieve::iterator")
}

#[cfg_attr(feature = "pyo3", pyclass(unsendable, module = "pm1"))]
pub struct Primes {
    sieve: cxx::UniquePtr<ffi::primesieve::iterator>,
    stop: u64,
}

impl Primes {
    pub fn new(low: u64, high: u64) -> Self {
        Primes {
            sieve: unsafe { ffi::primesieve::iterator::new1(low, high) }.within_unique_ptr(),
            stop: high,
        }
    }
}

#[cfg(feature = "pyo3")]
#[pymethods]
impl Primes {
    #[new]
    fn pynew(low: u64, high: u64) -> Self {
        Self::new(low, high)
    }

    fn __next__(mut s: PyRefMut<'_, Self>) -> Option<u64> {
        s.next()
    }

    fn __iter__(s: PyRef<'_, Self>) -> PyRef<'_, Self> {
        s
    }

    fn __repr__(mut s: PyRefMut<'_, Self>) -> String {
        let start;
        unsafe {
            start = s.sieve.pin_mut().next_prime();
            s.sieve.pin_mut().prev_prime(); // Rewind
        }
        format!("Primes({}, {})", start, s.stop)
    }
}

impl Iterator for Primes {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let n = unsafe { self.sieve.pin_mut().next_prime() };
        if n <= self.stop {
            Some(n)
        } else {
            None
        }
    }
}
