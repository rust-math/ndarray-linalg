//! Eigenvalue decomposition for Hermite matrices

use ndarray::*;

use crate::diagonal::*;
use crate::error::*;
use crate::layout::*;
use crate::operator::LinearOperator;
use crate::types::*;
use crate::{Range, UPLO};
use std::iter::FromIterator;

/// Eigenvalue decomposition of Hermite matrix reference
pub trait Eigh {
    type EigVal;
    type EigVec;
    type EigReal;
    fn eigh(&self, uplo: UPLO) -> Result<(Self::EigVal, Self::EigVec)>;
    fn eigh_range(
        &self,
        uplo: UPLO,
        range: Range<Self::EigReal>,
        abstol: Self::EigReal,
    ) -> Result<(Self::EigVal, Self::EigVec)>;
}

/// Eigenvalue decomposition of mutable reference of Hermite matrix
pub trait EighInplace {
    type EigVal;
    type EigReal;
    fn eigh_inplace(&mut self, uplo: UPLO) -> Result<(Self::EigVal, &mut Self)>;
    fn eigh_range_inplace(
        &mut self,
        uplo: UPLO,
        range: Range<Self::EigReal>,
        abstol: Self::EigReal,
    ) -> Result<(Self::EigVal, &mut Self)>;
}

/// Eigenvalue decomposition of Hermite matrix
pub trait EighInto: Sized {
    type EigVal;
    type EigReal;
    fn eigh_into(self, uplo: UPLO) -> Result<(Self::EigVal, Self)>;
    fn eigh_range_into(
        self,
        uplo: UPLO,
        range: Range<Self::EigReal>,
        abstol: Self::EigReal,
    ) -> Result<(Self::EigVal, Self)>;
}

impl<A, S> EighInto for ArrayBase<S, Ix2>
where
    A: Scalar + Lapack,
    S: DataMut<Elem = A>,
{
    type EigVal = Array1<A::Real>;
    type EigReal = A::Real;

    fn eigh_into(mut self, uplo: UPLO) -> Result<(Self::EigVal, Self)> {
        let (val, _) = self.eigh_inplace(uplo)?;
        Ok((val, self))
    }

    fn eigh_range_into(
        mut self,
        uplo: UPLO,
        range: Range<Self::EigReal>,
        abstol: Self::EigReal,
    ) -> Result<(Self::EigVal, Self)> {
        let (val, _) = self.eigh_range_inplace(uplo, range, abstol)?;
        Ok((val, self))
    }
}

impl<A, S, S2> EighInto for (ArrayBase<S, Ix2>, ArrayBase<S2, Ix2>)
where
    A: Scalar + Lapack,
    S: DataMut<Elem = A>,
    S2: DataMut<Elem = A>,
{
    type EigVal = Array1<A::Real>;
    type EigReal = A::Real;

    fn eigh_into(mut self, uplo: UPLO) -> Result<(Self::EigVal, Self)> {
        let (val, _) = self.eigh_inplace(uplo)?;
        Ok((val, self))
    }

    fn eigh_range_into(
        mut self,
        uplo: UPLO,
        range: Range<Self::EigReal>,
        abstol: Self::EigReal,
    ) -> Result<(Self::EigVal, Self)> {
        let (val, _) = self.eigh_range_inplace(uplo, range, abstol)?;
        Ok((val, self))
    }
}

impl<A, S> Eigh for ArrayBase<S, Ix2>
where
    A: Scalar + Lapack,
    S: Data<Elem = A>,
{
    type EigVal = Array1<A::Real>;
    type EigVec = Array2<A>;
    type EigReal = A::Real;

    fn eigh(&self, uplo: UPLO) -> Result<(Self::EigVal, Self::EigVec)> {
        let a = self.to_owned();
        a.eigh_into(uplo)
    }

    fn eigh_range(
        &self,
        uplo: UPLO,
        range: Range<Self::EigReal>,
        abstol: Self::EigReal,
    ) -> Result<(Self::EigVal, Self::EigVec)> {
        let a = self.to_owned();
        a.eigh_range_into(uplo, range, abstol)
    }
}

impl<A, S, S2> Eigh for (ArrayBase<S, Ix2>, ArrayBase<S2, Ix2>)
where
    A: Scalar + Lapack,
    S: Data<Elem = A>,
    S2: Data<Elem = A>,
{
    type EigVal = Array1<A::Real>;
    type EigVec = (Array2<A>, Array2<A>);
    type EigReal = A::Real;

    fn eigh(&self, uplo: UPLO) -> Result<(Self::EigVal, Self::EigVec)> {
        let (a, b) = (self.0.to_owned(), self.1.to_owned());
        (a, b).eigh_into(uplo)
    }

    fn eigh_range(
        &self,
        uplo: UPLO,
        range: Range<Self::EigReal>,
        abstol: Self::EigReal,
    ) -> Result<(Self::EigVal, Self::EigVec)> {
        let (a, b) = (self.0.to_owned(), self.1.to_owned());
        (a, b).eigh_range_into(uplo, range, abstol)
    }
}

impl<A, S> EighInplace for ArrayBase<S, Ix2>
where
    A: Scalar + Lapack,
    S: DataMut<Elem = A>,
{
    type EigVal = Array1<A::Real>;
    type EigReal = A::Real;

    fn eigh_inplace(&mut self, uplo: UPLO) -> Result<(Self::EigVal, &mut Self)> {
        let layout = self.square_layout()?;
        // XXX Force layout to be Fortran (see #146)
        match layout {
            MatrixLayout::C { .. } => self.swap_axes(0, 1),
            MatrixLayout::F { .. } => {}
        }
        let s = A::eigh(true, self.square_layout()?, uplo, self.as_allocated_mut()?)?;
        Ok((ArrayBase::from(s), self))
    }

    fn eigh_range_inplace(
        &mut self,
        uplo: UPLO,
        range: Range<Self::EigReal>,
        abstol: Self::EigReal,
    ) -> Result<(Self::EigVal, &mut Self)> {
        let layout = self.square_layout()?;
        // XXX Force layout to be Fortran (see #146)
        match layout {
            MatrixLayout::C { .. } => self.swap_axes(0, 1),
            MatrixLayout::F { .. } => {}
        }
        let s = A::eigh_range(
            true,
            self.square_layout()?,
            uplo,
            range,
            abstol,
            self.as_allocated_mut()?,
        )?;
        Ok((ArrayBase::from(s), self))
    }
}

impl<A, S, S2> EighInplace for (ArrayBase<S, Ix2>, ArrayBase<S2, Ix2>)
where
    A: Scalar + Lapack,
    S: DataMut<Elem = A>,
    S2: DataMut<Elem = A>,
{
    type EigVal = Array1<A::Real>;
    type EigReal = A::Real;

    fn eigh_inplace(&mut self, uplo: UPLO) -> Result<(Self::EigVal, &mut Self)> {
        let layout = self.0.square_layout()?;
        // XXX Force layout to be Fortran (see #146)
        match layout {
            MatrixLayout::C { .. } => self.0.swap_axes(0, 1),
            MatrixLayout::F { .. } => {}
        }

        let layout = self.1.square_layout()?;
        match layout {
            MatrixLayout::C { .. } => self.1.swap_axes(0, 1),
            MatrixLayout::F { .. } => {}
        }

        let s = A::eigh_generalized(
            true,
            self.0.square_layout()?,
            uplo,
            self.0.as_allocated_mut()?,
            self.1.as_allocated_mut()?,
        )?;

        Ok((ArrayBase::from(s), self))
    }

    fn eigh_range_inplace(
        &mut self,
        uplo: UPLO,
        range: Range<Self::EigReal>,
        abstol: Self::EigReal,
    ) -> Result<(Self::EigVal, &mut Self)> {
        todo!()
    }
}

/// Calculate eigenvalues without eigenvectors
pub trait EigValsh {
    type EigVal;
    type EigReal;
    fn eigvalsh(&self, uplo: UPLO) -> Result<Self::EigVal>;
    fn eigvalsh_range(
        &self,
        uplo: UPLO,
        range: Range<Self::EigReal>,
        abstol: Self::EigReal,
    ) -> Result<Self::EigVal>;
}

/// Calculate eigenvalues without eigenvectors
pub trait EigValshInto {
    type EigVal;
    type EigReal;
    fn eigvalsh_into(self, uplo: UPLO) -> Result<Self::EigVal>;
    fn eigvalsh_range_into(
        self,
        uplo: UPLO,
        range: Range<Self::EigReal>,
        abstol: Self::EigReal,
    ) -> Result<Self::EigVal>;
}

/// Calculate eigenvalues without eigenvectors
pub trait EigValshInplace {
    type EigVal;
    type EigReal;
    fn eigvalsh_inplace(&mut self, uplo: UPLO) -> Result<Self::EigVal>;
    fn eigvalsh_range_inplace(
        &mut self,
        uplo: UPLO,
        range: Range<Self::EigReal>,
        abstol: Self::EigReal,
    ) -> Result<Self::EigVal>;
}

impl<A, S> EigValshInto for ArrayBase<S, Ix2>
where
    A: Scalar + Lapack,
    S: DataMut<Elem = A>,
{
    type EigVal = Array1<A::Real>;
    type EigReal = A::Real;

    fn eigvalsh_into(mut self, uplo: UPLO) -> Result<Self::EigVal> {
        self.eigvalsh_inplace(uplo)
    }

    fn eigvalsh_range_into(
        mut self,
        uplo: UPLO,
        range: Range<Self::EigReal>,
        abstol: Self::EigReal,
    ) -> Result<Self::EigVal> {
        self.eigvalsh_range_inplace(uplo, range, abstol)
    }
}

impl<A, S> EigValsh for ArrayBase<S, Ix2>
where
    A: Scalar + Lapack,
    S: Data<Elem = A>,
{
    type EigVal = Array1<A::Real>;
    type EigReal = A::Real;

    fn eigvalsh(&self, uplo: UPLO) -> Result<Self::EigVal> {
        let a = self.to_owned();
        a.eigvalsh_into(uplo)
    }

    fn eigvalsh_range(
        &self,
        uplo: UPLO,
        range: Range<Self::EigReal>,
        abstol: Self::EigReal,
    ) -> Result<Self::EigVal> {
        let a = self.to_owned();
        a.eigvalsh_range_into(uplo, range, abstol)
    }
}

impl<A, S> EigValshInplace for ArrayBase<S, Ix2>
where
    A: Scalar + Lapack,
    S: DataMut<Elem = A>,
{
    type EigVal = Array1<A::Real>;
    type EigReal = A::Real;

    fn eigvalsh_inplace(&mut self, uplo: UPLO) -> Result<Self::EigVal> {
        let s = A::eigh(false, self.square_layout()?, uplo, self.as_allocated_mut()?)?;
        Ok(ArrayBase::from(s))
    }

    fn eigvalsh_range_inplace(
        &mut self,
        uplo: UPLO,
        range: Range<Self::EigReal>,
        abstol: Self::EigReal,
    ) -> Result<Self::EigVal> {
        let s = A::eigh_range(
            false,
            self.square_layout()?,
            uplo,
            range,
            abstol,
            self.as_allocated_mut()?,
        )?;
        Ok(ArrayBase::from(s))
    }
}

/// Calculate symmetric square-root matrix using `eigh`
pub trait SymmetricSqrt {
    type Output;
    fn ssqrt(&self, uplo: UPLO) -> Result<Self::Output>;
}

impl<A, S> SymmetricSqrt for ArrayBase<S, Ix2>
where
    A: Scalar + Lapack,
    S: Data<Elem = A>,
{
    type Output = Array2<A>;

    fn ssqrt(&self, uplo: UPLO) -> Result<Self::Output> {
        let a = self.to_owned();
        a.ssqrt_into(uplo)
    }
}

/// Calculate symmetric square-root matrix using `eigh`
pub trait SymmetricSqrtInto {
    type Output;
    fn ssqrt_into(self, uplo: UPLO) -> Result<Self::Output>;
}

impl<A, S> SymmetricSqrtInto for ArrayBase<S, Ix2>
where
    A: Scalar + Lapack,
    S: DataMut<Elem = A> + DataOwned,
{
    type Output = Array2<A>;

    fn ssqrt_into(self, uplo: UPLO) -> Result<Self::Output> {
        let (e, v) = self.eigh_into(uplo)?;
        let e_sqrt = Array::from_iter(e.iter().map(|r| Scalar::from_real(r.sqrt())));
        let ev = e_sqrt.into_diagonal().apply2(&v.t());
        Ok(v.apply2(&ev))
    }
}
