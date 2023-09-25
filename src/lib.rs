pub mod primes;

use primes::Primes;
use rug::{Complete, Integer as RugInt};

pub enum Pm1Result {
    Fail,
    Incomplete(RugInt),
    Complete(RugInt),
}

#[cfg_attr(feature = "pyo3", pyo3::pyclass(module = "pm1"))]
#[derive(Clone, Copy)]
pub enum Verbosity {
    Silent,
    Tqdm,
}

pub fn pm1_factorbase<Item, Iter>(
    factorbase: Iter,
    exp: u64,
    n: &RugInt,
    mut base: RugInt,
) -> Pm1Result
where
    Item: Into<RugInt>,
    Iter: Iterator<Item = Item>,
{
    let mut g = RugInt::new();
    for x in factorbase.map(|x| x.into()) {
        for _ in 0..exp {
            base = base.pow_mod(&x, n).unwrap();
        }
        (&base - 1i64).complete_into(&mut g);
        g = g.gcd(n);
        if &g == n {
            return Pm1Result::Fail;
        } else if g != 1 {
            return Pm1Result::Complete(g);
        }
    }
    Pm1Result::Incomplete(base)
}

pub fn pm1_factorbase_with_verbosity<Item, Iter>(
    factorbase: Iter,
    exp: u64,
    n: &RugInt,
    base: RugInt,
    verbosity: Verbosity,
) -> Pm1Result
where
    Item: Into<RugInt>,
    Iter: Iterator<Item = Item>,
{
    match verbosity {
        Verbosity::Silent => pm1_factorbase(factorbase, exp, n, base),
        Verbosity::Tqdm => pm1_factorbase(tqdm::tqdm(factorbase), exp, n, base),
    }
}

pub fn pm1(logb1: u64, exp: u64, logb2: u64, n: &RugInt, verbosity: Verbosity) -> Option<RugInt> {
    pm1base(logb1, exp, logb2, n, RugInt::from(2), verbosity)
}

pub fn pm1base(
    logb1: u64,
    exp: u64,
    logb2: u64,
    n: &RugInt,
    mut base: RugInt,
    verbosity: Verbosity,
) -> Option<RugInt> {
    let it1 = Primes::new(2, 1 << logb1);
    let it2 = Primes::new(1 << logb1, 1 << logb2);
    base = match pm1_factorbase_with_verbosity(it1, exp, n, base, verbosity) {
        Pm1Result::Fail => {
            return None;
        }
        Pm1Result::Incomplete(b) => b,
        Pm1Result::Complete(r) => {
            return Some(r);
        }
    };
    match pm1_factorbase_with_verbosity(it2, 1, n, base, verbosity) {
        Pm1Result::Fail => None,
        Pm1Result::Incomplete(_) => None,
        Pm1Result::Complete(r) => Some(r),
    }
}

#[cfg(feature = "pyo3")]
mod pymod {
    use super::*;
    use pyo3::prelude::*;
    use pyo3::types::PyIterator;

    #[derive(Debug, Clone)]
    struct Int(RugInt);

    impl FromPyObject<'_> for Int {
        fn extract(ob: &'_ PyAny) -> PyResult<Self> {
            let s = ob.to_string();
            Ok(Self(RugInt::from_str_radix(&s, 10).unwrap()))
        }
    }

    impl ToPyObject for Int {
        fn to_object(&self, py: Python<'_>) -> PyObject {
            let ty = py.get_type::<pyo3::types::PyLong>();
            let s = self.0.to_string_radix(10).into_py(py);
            ty.call1((s,)).unwrap().to_object(py)
        }
    }

    #[pyclass(module = "pm1")]
    enum PyPm1Result {
        Fail,
        Incomplete,
        Complete,
    }

    impl ToPyObject for Pm1Result {
        fn to_object(&self, py: Python<'_>) -> PyObject {
            let zero = RugInt::from(0);
            let (c, n) = match self {
                Pm1Result::Fail => (PyPm1Result::Fail, &zero),
                Pm1Result::Incomplete(r) => (PyPm1Result::Incomplete, r),
                Pm1Result::Complete(r) => (PyPm1Result::Complete, r),
            };
            (c.into_py(py), Int(n.clone()).to_object(py)).to_object(py)
        }
    }

    #[pyfunction]
    #[pyo3(name = "pm1")]
    fn pypm1(
        py: Python<'_>,
        logb1: u64,
        exp: u64,
        logb2: u64,
        n: Int,
        verbosity: Verbosity,
    ) -> Option<PyObject> {
        pm1(logb1, exp, logb2, &n.0, verbosity).map(|x| Int(x).to_object(py))
    }

    #[pyfunction]
    #[pyo3(name = "pm1base")]
    fn pypm1base(
        py: Python<'_>,
        logb1: u64,
        exp: u64,
        logb2: u64,
        n: Int,
        base: Int,
        verbosity: Verbosity,
    ) -> Option<PyObject> {
        pm1base(logb1, exp, logb2, &n.0, base.0, verbosity).map(|x| Int(x).to_object(py))
    }

    #[pyfunction]
    fn pypm1_factorbase<'py>(
        py: Python<'py>,
        factorbase: &'py PyAny,
        exp: u64,
        n: Int,
        base: Int,
        verbosity: Verbosity,
    ) -> PyResult<Py<PyAny>> {
        let fb1 = factorbase
            .call_method0("__iter__")?
            .downcast::<PyIterator>()?
            .map(|x| {
                let x = x.expect("What is this iteration failure?");
                x.extract::<Int>().expect("Neither int nor pm1.Int?").0
            });
        Ok(pm1_factorbase_with_verbosity(fb1, exp, &n.0, base.0, verbosity).to_object(py))
    }

    #[pyfunction]
    #[pyo3(name = "pm1_custom_primes")]
    fn pypm1_custom_primes<'py>(
        py: Python<'py>,
        factorbase: &mut Primes,
        exp: u64,
        n: Int,
        base: Int,
        verbosity: Verbosity,
    ) -> PyResult<Py<PyAny>> {
        Ok(
            pm1_factorbase_with_verbosity(factorbase.clone(), exp, &n.0, base.0, verbosity)
                .to_object(py),
        )
    }

    #[pymodule]
    #[pyo3(name = "pm1")]
    fn pm1_module(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
        m.add_class::<Verbosity>()?;
        m.add_class::<primes::Primes>()?;
        m.add_class::<PyPm1Result>()?;
        m.add_function(wrap_pyfunction!(pypm1, m)?)?;
        m.add_function(wrap_pyfunction!(pypm1base, m)?)?;
        m.add_function(wrap_pyfunction!(pypm1_factorbase, m)?)?;
        m.add_function(wrap_pyfunction!(pypm1_custom_primes, m)?)?;
        Ok(())
    }
}
