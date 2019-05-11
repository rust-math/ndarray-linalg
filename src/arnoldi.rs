use crate::{generate::*, inner::*, norm::Norm, types::*};
use ndarray::*;

/// Iterative orthogonalizer using modified Gram-Schmit procedure
#[derive(Debug, Clone)]
pub struct MGS<A> {
    /// Dimension of base space
    dimension: usize,
    /// Basis of spanned space
    q: Vec<Array1<A>>,
}

/// Q-matrix (unitary)
pub type Q<A> = Array2<A>;
/// R-matrix (upper triangle)
pub type R<A> = Array2<A>;

impl<A: Scalar> MGS<A> {
    /// Create empty linear space
    ///
    /// ```rust
    /// # use ndarray_linalg::*;
    /// const N: usize = 5;
    /// let mgs = arnoldi::MGS::<f32>::new(N);
    /// assert_eq!(mgs.dim(), N);
    /// assert_eq!(mgs.len(), 0);
    /// ```
    pub fn new(dimension: usize) -> Self {
        Self {
            dimension,
            q: Vec::new(),
        }
    }

    pub fn dim(&self) -> usize {
        self.dimension
    }

    pub fn len(&self) -> usize {
        self.q.len()
    }

    /// Orthogonalize given vector using current basis
    ///
    /// Panic
    /// -------
    /// - if the size of the input array mismaches to the dimension
    pub fn orthogonalize<S>(&self, a: &mut ArrayBase<S, Ix1>) -> Array1<A>
    where
        A: Lapack,
        S: DataMut<Elem = A>,
    {
        assert_eq!(a.len(), self.dim());
        let mut coef = Array1::zeros(self.len() + 1);
        for i in 0..self.len() {
            let q = &self.q[i];
            let c = q.inner(&a);
            azip!(mut a (&mut *a), q (q) in { *a = *a - c * q } );
            coef[i] = c;
        }
        let nrm = a.norm_l2();
        coef[self.len()] = A::from_real(nrm);
        coef
    }

    /// Add new vector if the residual is larger than relative tolerance
    ///
    /// Panic
    /// -------
    /// - if the size of the input array mismaches to the dimension
    ///
    /// ```rust
    /// # use ndarray::*;
    /// # use ndarray_linalg::*;
    /// let mut mgs = arnoldi::MGS::new(3);
    /// let coef = mgs.append(array![0.0, 1.0, 0.0], 1e-9).unwrap();
    /// close_l2(&coef, &array![1.0], 1e-9).unwrap();
    ///
    /// let coef = mgs.append(array![1.0, 1.0, 0.0], 1e-9).unwrap();
    /// close_l2(&coef, &array![1.0, 1.0], 1e-9).unwrap();
    ///
    /// assert!(mgs.append(array![1.0, 2.0, 0.0], 1e-9).is_err());  // Fail if the vector is linearly dependend
    ///
    /// if let Err(coef) = mgs.append(array![1.0, 2.0, 0.0], 1e-9) {
    ///     close_l2(&coef, &array![2.0, 1.0, 0.0], 1e-9).unwrap(); // You can get coefficients of dependent vector
    /// }
    /// ```
    pub fn append<S>(&mut self, a: ArrayBase<S, Ix1>, rtol: A::Real) -> Result<Array1<A>, Array1<A>>
    where
        A: Lapack,
        S: Data<Elem = A>,
    {
        let mut a = a.into_owned();
        let coef = self.orthogonalize(&mut a);
        let nrm = coef[coef.len() - 1].re();
        if nrm < rtol {
            // Linearly dependent
            return Err(coef);
        }
        azip!(mut a in { *a = *a / A::from_real(nrm) });
        self.q.push(a);
        Ok(coef)
    }

    /// Get orthogonal basis as Q matrix
    pub fn get_q(&self) -> Q<A> {
        hstack(&self.q).unwrap()
    }
}

/// Strategy for linearly dependent vectors appearing in iterative QR decomposition
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Strategy {
    /// Terminate iteration if dependent vector comes
    Terminate,

    /// Skip dependent vector
    Skip,

    /// Orghotonalize dependent vector without adding to Q,
    /// thus R must be non-regular like following:
    ///
    /// ```ignore
    /// x x x x x
    /// 0 x x x x
    /// 0 0 0 x x
    /// 0 0 0 0 x
    /// 0 0 0 0 0   // 0-filled to be square matrix
    /// ```
    Full,
}

pub fn mgs<A, S>(
    iter: impl Iterator<Item = ArrayBase<S, Ix1>>,
    dim: usize,
    rtol: A::Real,
    strategy: Strategy,
) -> (Q<A>, R<A>)
where
    A: Scalar + Lapack,
    S: Data<Elem = A>,
{
    let mut ortho = MGS::new(dim);
    let mut coefs = Vec::new();
    for a in iter {
        match ortho.append(a, rtol) {
            Ok(coef) => coefs.push(coef),
            Err(coef) => match strategy {
                Strategy::Terminate => break,
                Strategy::Skip => continue,
                Strategy::Full => coefs.push(coef),
            },
        }
    }
    let m = coefs.len();
    let mut r = Array2::zeros((m, m));
    for i in 0..m {
        for j in 0..=i {
            r[(j, i)] = coefs[i][j];
        }
    }
    (ortho.get_q(), r)
}
