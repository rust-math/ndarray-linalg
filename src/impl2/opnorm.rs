//! Implement Operator norms for matrices

use lapack::c;
use lapack::c::Layout::ColumnMajor as cm;

use types::*;
use layout::Layout;

#[repr(u8)]
pub enum NormType {
    One = b'o',
    Infinity = b'i',
    Frobenius = b'f',
}

impl NormType {
    fn transpose(self) -> Self {
        match self {
            NormType::One => NormType::Infinity,
            NormType::Infinity => NormType::One,
            NormType::Frobenius => NormType::Frobenius,
        }
    }
}

pub trait OperatorNorm_: AssociatedReal {
    fn opnorm(NormType, Layout, &[Self]) -> Self::Real;
}

macro_rules! impl_opnorm {
    ($scalar:ty, $lange:path) => {
impl OperatorNorm_ for $scalar {
    fn opnorm(t: NormType, l: Layout, a: &[Self]) -> Self::Real {
        match l {
            Layout::F((col, lda)) => $lange(cm, t as u8, lda, col, a, lda),
            Layout::C((row, lda)) => $lange(cm, t.transpose() as u8, lda, row, a, lda),
        }
    }
}
}} // impl_opnorm!

impl_opnorm!(f64, c::dlange);
impl_opnorm!(f32, c::slange);
impl_opnorm!(c64, c::zlange);
impl_opnorm!(c32, c::clange);
