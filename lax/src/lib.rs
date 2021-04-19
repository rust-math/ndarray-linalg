//! Linear Algebra eXtension (LAX)
//! ===============================
//!
//! ndarray-free safe Rust wrapper for LAPACK FFI
//!
//! Linear equation, Inverse matrix, Condition number
//! --------------------------------------------------
//!
//! As the property of $A$, several types of triangular factorization are used:
//!
//! - LU-decomposition for general matrix
//!   - $PA = LU$, where $L$ is lower matrix, $U$ is upper matrix, and $P$ is permutation matrix
//! - Bunch-Kaufman diagonal pivoting method for nonpositive-definite Hermitian matrix
//!   - $A = U D U^\dagger$, where $U$ is upper matrix,
//!     $D$ is Hermitian and block diagonal with 1-by-1 and 2-by-2 diagonal blocks.
//!
//! | matrix type                     | Triangler factorization (TRF) | Solve (TRS) | Inverse matrix (TRI) | Reciprocal condition number (CON) |
//! |:--------------------------------|:------------------------------|:------------|:---------------------|:----------------------------------|
//! | General (GE)                    | [lu]                          | [solve]     | [inv]                | [rcond]                           |
//! | Symmetric (SY) / Hermitian (HE) | [bk]                          | [solveh]    | [invh]               | -                                 |
//!
//! [lu]:    solve/trait.Solve_.html#tymethod.lu
//! [solve]: solve/trait.Solve_.html#tymethod.solve
//! [inv]:   solve/trait.Solve_.html#tymethod.inv
//! [rcond]: solve/trait.Solve_.html#tymethod.rcond
//!
//! [bk]:     solveh/trait.Solveh_.html#tymethod.bk
//! [solveh]: solveh/trait.Solveh_.html#tymethod.solveh
//! [invh]:   solveh/trait.Solveh_.html#tymethod.invh
//!
//! Eigenvalue Problem
//! -------------------
//!
//! Solve eigenvalue problem for a matrix $A$
//!
//! $$ Av_i = \lambda_i v_i $$
//!
//! or generalized eigenvalue problem
//!
//! $$ Av_i = \lambda_i B v_i $$
//!
//! | matrix type                     | Eigenvalue (EV) | Generalized Eigenvalue Problem (EG) |
//! |:--------------------------------|:----------------|:------------------------------------|
//! | General (GE)                    |[eig]            | -                                   |
//! | Symmetric (SY) / Hermitian (HE) |[eigh]           |[eigh_generalized]                   |
//!
//! [eig]:              eig/trait.Eig_.html#tymethod.eig
//! [eigh]:             eigh/trait.Eigh_.html#tymethod.eigh
//! [eigh_generalized]: eigh/trait.Eigh_.html#tymethod.eigh_generalized
//!
//! Singular Value Decomposition (SVD), Least square problem
//! ----------------------------------------------------------
//!
//! | matrix type  | Singular Value Decomposition (SVD) | SVD with divided-and-conquer (SDD) | Least square problem (LSD) |
//! |:-------------|:-----------------------------------|:-----------------------------------|:---------------------------|
//! | General (GE) | [svd]                              | [svddc]                            | [least_squares]            |
//!
//! [svd]:   svd/trait.SVD_.html#tymethod.svd
//! [svddc]: svddck/trait.SVDDC_.html#tymethod.svddc
//! [least_squares]: least_squares/trait.LeastSquaresSvdDivideConquer_.html#tymethod.least_squares

#[cfg(any(feature = "intel-mkl-system", feature = "intel-mkl-static"))]
extern crate intel_mkl_src as _src;

#[cfg(any(feature = "openblas-system", feature = "openblas-static"))]
extern crate openblas_src as _src;

#[cfg(any(feature = "netlib-system", feature = "netlib-static"))]
extern crate netlib_src as _src;

pub mod error;
pub mod layout;

mod cholesky;
mod eig;
mod eigh;
mod eigh_generalized;
mod least_squares;
mod opnorm;
mod qr;
mod rcond;
mod solve;
mod solveh;
mod strict;
mod svd;
mod svddc;
mod traits;
mod triangular;
mod tridiagonal;

pub use self::eig::*;
pub use self::eigh::*;
pub use self::eigh_generalized::*;
pub use self::least_squares::*;
pub use self::opnorm::*;
pub use self::qr::*;
pub use self::rcond::*;
pub use self::solve::*;
pub use self::solveh::*;
pub use self::strict::*;
pub use self::svd::*;
pub use self::svddc::*;
pub use self::traits::*;
pub use self::triangular::*;
pub use self::tridiagonal::*;

pub type Pivot = Vec<i32>;

/// Upper/Lower specification for seveal usages
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum UPLO {
    Upper = b'U',
    Lower = b'L',
}

impl UPLO {
    pub fn t(self) -> Self {
        match self {
            UPLO::Upper => UPLO::Lower,
            UPLO::Lower => UPLO::Upper,
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Transpose {
    No = b'N',
    Transpose = b'T',
    Hermite = b'C',
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum NormType {
    One = b'O',
    Infinity = b'I',
    Frobenius = b'F',
}

impl NormType {
    pub fn transpose(self) -> Self {
        match self {
            NormType::One => NormType::Infinity,
            NormType::Infinity => NormType::One,
            NormType::Frobenius => NormType::Frobenius,
        }
    }
}

/// Types of generalized eigenvalue problem
#[allow(dead_code)] // FIXME create interface to use ABxlx and BAxlx
#[repr(i32)]
pub enum ITYPE {
    /// Solve $ A x = \lambda B x $
    AxlBx = 1,
    /// Solve $ A B x = \lambda x $
    ABxlx = 2,
    /// Solve $ B A x = \lambda x $
    BAxlx = 3,
}

/// Create a vector without initialization
///
/// Safety
/// ------
/// - Memory is not initialized. Do not read the memory before write.
///
unsafe fn vec_uninit<T: Sized>(n: usize) -> Vec<T> {
    let mut v = Vec::with_capacity(n);
    v.set_len(n);
    v
}
