//! Define trait for Hermite matrices

use std::fmt::Debug;
use ndarray::prelude::*;
use ndarray::LinalgScalar;
use num_traits::float::Float;
use lapack::c::Layout;

use matrix::Matrix;
use square::SquareMatrix;
use error::LinalgError;
use eig::ImplEig;
use eigh::ImplEigh;
use qr::ImplQR;
use svd::ImplSVD;
use norm::ImplNorm;
use solve::ImplSolve;
use cholesky::ImplCholesky;

/// Methods for Hermite matrix
pub trait HermiteMatrix: SquareMatrix + Matrix {
    /// eigenvalue decomposition
    fn eigh(self) -> Result<(Self::Vector, Self), LinalgError>;
    /// symmetric square root of Hermite matrix
    fn ssqrt(self) -> Result<Self, LinalgError>;
    /// Cholesky factorization
    fn cholesky(self) -> Result<Self, LinalgError>;
}

impl<A> HermiteMatrix for Array<A, (Ix, Ix)>
    where A: ImplEig + ImplQR + ImplSVD + ImplNorm + ImplSolve + ImplEigh + LinalgScalar + Float + Debug
{
    fn eigh(self) -> Result<(Self::Vector, Self), LinalgError> {
        try!(self.check_square());
        let (rows, cols) = self.size();
        let (w, a) = try!(ImplEigh::eigh(rows, self.into_raw_vec()));
        let ea = Array::from_vec(w);
        let va = Array::from_vec(a).into_shape((rows, cols)).unwrap().reversed_axes();
        Ok((ea, va))
    }
    fn ssqrt(self) -> Result<Self, LinalgError> {
        let (n, _) = self.size();
        let (e, v) = try!(self.eigh());
        let mut res = Array::zeros((n, n));
        for i in 0..n {
            for j in 0..n {
                res[(i, j)] = e[i].sqrt() * v[(j, i)];
            }
        }
        Ok(v.dot(&res))
    }
    fn cholesky(self) -> Result<Self, LinalgError> {
        try!(self.check_square());
        println!("layout = {:?}", self.layout());
        let (n, _) = self.size();
        let layout = self.layout();
        let a = try!(ImplCholesky::cholesky(layout, n, self.into_raw_vec()));
        let mut c = match layout {
            Layout::RowMajor => Array::from_vec(a).into_shape((n, n)).unwrap(),
            Layout::ColumnMajor => Array::from_vec(a).into_shape((n, n)).unwrap().reversed_axes(),
        };
        for ((i, j), val) in c.indexed_iter_mut() {
            if i > j {
                *val = A::zero();
            }
        }
        Ok(c)
    }
}
