use super::*;
use crate::{inner::*, norm::*};
use num_traits::Zero;

/// Iterative orthogonalizer using Householder reflection
#[derive(Debug, Clone)]
pub struct Householder<A: Scalar> {
    dim: usize,
    v: Vec<Array1<A>>,
}

impl<A: Scalar> Householder<A> {
    pub fn new(dim: usize) -> Self {
        Householder { dim, v: Vec::new() }
    }

    /// Take a Reflection `P = I - 2ww^T`
    fn reflect<S: DataMut<Elem = A>>(&self, k: usize, a: &mut ArrayBase<S, Ix1>) {
        assert!(k < self.v.len());
        assert_eq!(a.len(), self.dim);

        let w = self.v[k].slice(s![k..]);
        let mut a_slice = a.slice_mut(s![k..]);
        let c = A::from(2.0).unwrap() * w.inner(&a_slice);
        for l in 0..self.dim - k {
            a_slice[l] -= c * w[l];
        }
    }
}

impl<A: Scalar + Lapack> Orthogonalizer for Householder<A> {
    type Elem = A;

    fn dim(&self) -> usize {
        self.dim
    }

    fn len(&self) -> usize {
        self.v.len()
    }

    fn orthogonalize<S>(&self, a: &mut ArrayBase<S, Ix1>) -> A::Real
    where
        S: DataMut<Elem = A>,
    {
        for k in 0..self.len() {
            self.reflect(k, a);
        }
        if self.is_full() {
            return Zero::zero();
        }
        // residual norm
        a.slice(s![self.len()..]).norm_l2()
    }

    fn append<S>(&mut self, mut a: ArrayBase<S, Ix1>, rtol: A::Real) -> Result<Array1<A>, Array1<A>>
    where
        S: DataMut<Elem = A>,
    {
        assert_eq!(a.len(), self.dim);
        let alpha = self.orthogonalize(&mut a);

        // Generate coefficient vector
        let mut coef = Array::zeros(self.len() + 1);
        for i in 0..self.len() {
            coef[i] = a[i];
        }
        coef[self.len()] = A::from_real(alpha);

        if alpha < rtol {
            return Err(coef);
        }

        // Add reflector
        let k = self.len();
        let xi = a[k].re();
        a[k] = A::from(if xi > Zero::zero() { xi + alpha } else { xi - alpha }).unwrap();
        let norm = a.slice(s![k..]).norm_l2();
        dbg!(alpha);
        dbg!(norm);
        azip!(mut a (a.slice_mut(s![k..])) in { *a = a.div_real(norm)} );
        self.v.push(a.into_owned());
        dbg!(&self.v);
        Ok(coef)
    }

    fn get_q(&self) -> Q<A> {
        assert!(self.len() > 0);
        let mut a = Array::eye(self.len());
        for mut col in a.axis_iter_mut(Axis(0)) {
            for l in 0..self.len() {
                self.reflect(l, &mut col);
            }
        }
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert::*;

    #[test]
    fn householder_append() {
        let mut householder = Householder::new(3);
        let coef = householder.append(array![0.0, 1.0, 0.0], 1e-9).unwrap();
        close_l2(&coef, &array![1.0], 1e-9).unwrap();

        let coef = householder.append(array![1.0, 1.0, 0.0], 1e-9).unwrap();
        close_l2(&coef, &array![1.0, 1.0], 1e-9).unwrap();

        assert!(householder.append(array![1.0, 2.0, 0.0], 1e-9).is_err());

        if let Err(coef) = householder.append(array![1.0, 2.0, 0.0], 1e-9) {
            dbg!(&coef);
            close_l2(&coef, &array![2.0, 1.0, 0.0], 1e-9).unwrap();
        }
    }

}
