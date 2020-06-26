use cauchy::Scalar;
use intel_mkl_sys::*;
use num_complex::{Complex32 as c32, Complex64 as c64};

trait VecMath: Scalar {
    /* Arthmetic */
    fn add(a: &[Self], b: &[Self], out: &mut [Self]);
    fn sub(a: &[Self], b: &[Self], out: &mut [Self]);
    fn mul(a: &[Self], b: &[Self], out: &mut [Self]);
    fn abs(in_: &[Self], out: &mut [Self::Real]);

    /* Power and Root */
    fn div(a: &[Self], b: &[Self], out: &mut [Self]);
    fn sqrt(in_: &[Self], out: &mut [Self]);
    fn pow(a: &[Self], b: &[Self], out: &mut [Self]);
    fn powx(a: &[Self], b: Self, out: &mut [Self]);

    /* Exponential and Logarithmic */
    fn exp(in_: &[Self], out: &mut [Self]);
    fn ln(in_: &[Self], out: &mut [Self]);
    fn log10(in_: &[Self], out: &mut [Self]);

    /* Trigonometric */
    fn cos(in_: &[Self], out: &mut [Self]);
    fn sin(in_: &[Self], out: &mut [Self]);
    fn tan(in_: &[Self], out: &mut [Self]);
    fn acos(in_: &[Self], out: &mut [Self]);
    fn asin(in_: &[Self], out: &mut [Self]);
    fn atan(in_: &[Self], out: &mut [Self]);

    /* Hyperbolic */
    fn cosh(in_: &[Self], out: &mut [Self]);
    fn sinh(in_: &[Self], out: &mut [Self]);
    fn tanh(in_: &[Self], out: &mut [Self]);
    fn acosh(in_: &[Self], out: &mut [Self]);
    fn asinh(in_: &[Self], out: &mut [Self]);
    fn atanh(in_: &[Self], out: &mut [Self]);
}

trait VecMathReal: Sized {
    /* Arthmetic */
    fn sqr(in_: &[Self], out: &mut [Self]);
    fn linear_frac(
        a: &[Self],
        b: &[Self],
        scale_a: Self,
        shift_a: Self,
        scale_b: Self,
        shift_b: Self,
        out: &mut [Self],
    );
    fn fmod(a: &[Self], b: &[Self], out: &mut [Self]);
    fn remainder(a: &[Self], b: &[Self], out: &mut [Self]);

    /* Power and Root */
    fn inv(in_: &[Self], out: &mut [Self]);
    fn inv_sqrt(in_: &[Self], out: &mut [Self]);
    fn cbrt(in_: &[Self], out: &mut [Self]);
    fn inv_cbrt(in_: &[Self], out: &mut [Self]);
    fn pow2o3(in_: &[Self], out: &mut [Self]);
    fn pow3o2(in_: &[Self], out: &mut [Self]);
    fn powr(a: &[Self], b: &[Self], out: &mut [Self]);
    fn hypot(a: &[Self], b: &[Self], out: &mut [Self]);

    /* Exponential and Logarithmic */
    fn exp2(in_: &[Self], out: &mut [Self]);
    fn exp10(in_: &[Self], out: &mut [Self]);
    fn expm1(in_: &[Self], out: &mut [Self]);
    fn log2(in_: &[Self], out: &mut [Self]);
    fn log10(in_: &[Self], out: &mut [Self]);
    fn log1p(in_: &[Self], out: &mut [Self]);
    fn logb(in_: &[Self], out: &mut [Self]);

    /* Trigonometric */
    fn sin_cos(theta: &[Self], sin: &mut [Self], cos: &mut [Self]);
    fn atan2(sin: &[Self], cos: &[Self], theta: &mut [Self]);

    /* Special */
    fn erf(in_: &[Self], out: &mut [Self]);
    fn erfc(in_: &[Self], out: &mut [Self]);
    fn cdf_normal(in_: &[Self], out: &mut [Self]);
    fn erf_inv(in_: &[Self], out: &mut [Self]);
    fn erfc_inv(in_: &[Self], out: &mut [Self]);
    fn cdf_normal_inv(in_: &[Self], out: &mut [Self]);
    fn ln_gamma(in_: &[Self], out: &mut [Self]);
    fn gamma(in_: &[Self], out: &mut [Self]);
    fn exp_integral(in_: &[Self], out: &mut [Self]);

    /* Rounding */
    fn floor(in_: &[Self], out: &mut [Self]);
    fn ceil(in_: &[Self], out: &mut [Self]);
    fn trunc(in_: &[Self], out: &mut [Self]);
    fn round(in_: &[Self], out: &mut [Self]);
    fn near_by_int(in_: &[Self], out: &mut [Self]);
    fn rint(in_: &[Self], out: &mut [Self]);
    fn modf(a: &[Self], y: &mut [Self], z: &mut [Self]);
    fn frac(in_: &[Self], out: &mut [Self]);

    /* Miscellaneous */
    fn copy_sign(a: &[Self], b: &[Self], out: &mut [Self]);
    fn next_after(a: &[Self], b: &[Self], out: &mut [Self]);
    fn fdim(a: &[Self], b: &[Self], out: &mut [Self]);
    fn fmax(a: &[Self], b: &[Self], out: &mut [Self]);
    fn fmin(a: &[Self], b: &[Self], out: &mut [Self]);
    fn maxmag(a: &[Self], b: &[Self], out: &mut [Self]);
    fn minmag(a: &[Self], b: &[Self], out: &mut [Self]);
}

trait VecMathComplex: Scalar {
    /* Arthmetic */
    fn mul_by_conj(a: &[Self], b: &[Self], out: &mut [Self]);
    fn conj(in_: &[Self], out: &mut [Self]);
    fn arg(in_: &[Self], out: &mut [Self::Real]);

    /* Trigonometric */
    fn cis(in_: &[Self::Real], out: &mut [Self]);
}

macro_rules! impl_unary {
    ($scalar:ty, $name:ident, $impl_name:ident) => {
        fn $name(in_: &[$scalar], out: &mut [$scalar]) {
            assert_eq!(in_.len(), out.len());
            let n = in_.len() as i32;
            unsafe { $impl_name(n, in_.as_ptr(), out.as_mut_ptr()) }
        }
    };
}

macro_rules! impl_binary {
    ($scalar:ty, $name:ident, $impl_name:ident) => {
        fn $name(a: &[$scalar], b: &[$scalar], out: &mut [$scalar]) {
            assert_eq!(a.len(), out.len());
            assert_eq!(b.len(), out.len());
            let n = out.len() as i32;
            unsafe { $impl_name(n, a.as_ptr(), b.as_ptr(), out.as_mut_ptr()) }
        }
    };
}

macro_rules! impl_powx {
    ($scalar:ty, $name:ident, $impl_name:ident) => {
        fn $name(a: &[$scalar], b: $scalar, out: &mut [$scalar]) {
            assert_eq!(a.len(), out.len());
            let n = out.len() as i32;
            unsafe { $impl_name(n, a.as_ptr(), b, out.as_mut_ptr()) }
        }
    };
}

impl VecMath for f32 {
    impl_binary!(f32, add, vsAdd);
    impl_binary!(f32, sub, vsSub);
    impl_binary!(f32, mul, vsMul);
    impl_unary!(f32, abs, vsAbs);

    impl_binary!(f32, div, vsDiv);
    impl_unary!(f32, sqrt, vsSqrt);
    impl_binary!(f32, pow, vsPow);
    impl_powx!(f32, powx, vsPowx);

    impl_unary!(f32, exp, vsExp);
    impl_unary!(f32, ln, vsLn);
    impl_unary!(f32, log10, vsLog10);

    impl_unary!(f32, cos, vsCos);
    impl_unary!(f32, sin, vsSin);
    impl_unary!(f32, tan, vsTan);
    impl_unary!(f32, acos, vsAcos);
    impl_unary!(f32, asin, vsAsin);
    impl_unary!(f32, atan, vsAtan);

    impl_unary!(f32, cosh, vsCosh);
    impl_unary!(f32, sinh, vsSinh);
    impl_unary!(f32, tanh, vsTanh);
    impl_unary!(f32, acosh, vsAcosh);
    impl_unary!(f32, asinh, vsAsinh);
    impl_unary!(f32, atanh, vsAtanh);
}

impl VecMath for f64 {
    impl_binary!(f64, add, vdAdd);
    impl_binary!(f64, sub, vdSub);
    impl_binary!(f64, mul, vdMul);
    impl_unary!(f64, abs, vdAbs);

    impl_binary!(f64, div, vdDiv);
    impl_unary!(f64, sqrt, vdSqrt);
    impl_binary!(f64, pow, vdPow);
    impl_powx!(f64, powx, vdPowx);

    impl_unary!(f64, exp, vdExp);
    impl_unary!(f64, ln, vdLn);
    impl_unary!(f64, log10, vdLog10);

    impl_unary!(f64, cos, vdCos);
    impl_unary!(f64, sin, vdSin);
    impl_unary!(f64, tan, vdTan);
    impl_unary!(f64, acos, vdAcos);
    impl_unary!(f64, asin, vdAsin);
    impl_unary!(f64, atan, vdAtan);

    impl_unary!(f64, cosh, vdCosh);
    impl_unary!(f64, sinh, vdSinh);
    impl_unary!(f64, tanh, vdTanh);
    impl_unary!(f64, acosh, vdAcosh);
    impl_unary!(f64, asinh, vdAsinh);
    impl_unary!(f64, atanh, vdAtanh);
}

macro_rules! impl_unary2 {
    ($scalar:ty, $name:ident, $impl_name:ident) => {
        fn $name(in_: &[$scalar], out1: &mut [$scalar], out2: &mut [$scalar]) {
            assert_eq!(in_.len(), out1.len());
            assert_eq!(in_.len(), out2.len());
            let n = in_.len() as i32;
            unsafe { $impl_name(n, in_.as_ptr(), out1.as_mut_ptr(), out2.as_mut_ptr()) }
        }
    };
}

macro_rules! impl_linearfrac {
    ($scalar:ty, $name:ident, $impl_name:ident) => {
        fn $name(
            a: &[$scalar],
            b: &[$scalar],
            scale_a: $scalar,
            shift_a: $scalar,
            scale_b: $scalar,
            shift_b: $scalar,
            out: &mut [$scalar],
        ) {
            assert_eq!(a.len(), out.len());
            assert_eq!(b.len(), out.len());
            let n = out.len() as i32;
            unsafe {
                $impl_name(
                    n,
                    a.as_ptr(),
                    b.as_ptr(),
                    scale_a,
                    shift_a,
                    scale_b,
                    shift_b,
                    out.as_mut_ptr(),
                )
            }
        }
    };
}

impl VecMathReal for f32 {
    impl_unary!(f32, sqr, vsSqr);
    impl_linearfrac!(f32, linear_frac, vsLinearFrac);
    impl_binary!(f32, fmod, vsFmod);
    impl_binary!(f32, remainder, vsRemainder);

    impl_unary!(f32, inv, vsInv);
    impl_unary!(f32, inv_sqrt, vsInvSqrt);
    impl_unary!(f32, cbrt, vsCbrt);
    impl_unary!(f32, inv_cbrt, vsInvCbrt);
    impl_unary!(f32, pow2o3, vsPow2o3);
    impl_unary!(f32, pow3o2, vsPow3o2);
    impl_binary!(f32, powr, vsPowr);
    impl_binary!(f32, hypot, vsHypot);

    impl_unary!(f32, exp2, vsExp2);
    impl_unary!(f32, exp10, vsExp10);
    impl_unary!(f32, expm1, vsExpm1);
    impl_unary!(f32, log2, vsLog2);
    impl_unary!(f32, log10, vsLog10);
    impl_unary!(f32, log1p, vsLog1p);
    impl_unary!(f32, logb, vsLogb);

    impl_unary2!(f32, sin_cos, vsSinCos);
    impl_binary!(f32, atan2, vsAtan2);

    impl_unary!(f32, erf, vsErf);
    impl_unary!(f32, erfc, vsErfc);
    impl_unary!(f32, cdf_normal, vsCdfNorm);
    impl_unary!(f32, erf_inv, vsErfInv);
    impl_unary!(f32, erfc_inv, vsErfcInv);
    impl_unary!(f32, cdf_normal_inv, vsCdfNormInv);
    impl_unary!(f32, ln_gamma, vsLGamma);
    impl_unary!(f32, gamma, vsTGamma);
    impl_unary!(f32, exp_integral, vsExpInt1);

    impl_unary!(f32, floor, vsFloor);
    impl_unary!(f32, ceil, vsCeil);
    impl_unary!(f32, trunc, vsTrunc);
    impl_unary!(f32, round, vsRound);
    impl_unary!(f32, near_by_int, vsNearbyInt);
    impl_unary!(f32, rint, vsRint);
    impl_unary2!(f32, modf, vsModf);
    impl_unary!(f32, frac, vsFrac);

    impl_binary!(f32, copy_sign, vsCopySign);
    impl_binary!(f32, next_after, vsNextAfter);
    impl_binary!(f32, fdim, vsFdim);
    impl_binary!(f32, fmax, vsFmax);
    impl_binary!(f32, fmin, vsFmin);
    impl_binary!(f32, maxmag, vsMaxMag);
    impl_binary!(f32, minmag, vsMinMag);
}

impl VecMathReal for f64 {
    impl_unary!(f64, sqr, vdSqr);
    impl_linearfrac!(f64, linear_frac, vdLinearFrac);
    impl_binary!(f64, fmod, vdFmod);
    impl_binary!(f64, remainder, vdRemainder);

    impl_unary!(f64, inv, vdInv);
    impl_unary!(f64, inv_sqrt, vdInvSqrt);
    impl_unary!(f64, cbrt, vdCbrt);
    impl_unary!(f64, inv_cbrt, vdInvCbrt);
    impl_unary!(f64, pow2o3, vdPow2o3);
    impl_unary!(f64, pow3o2, vdPow3o2);
    impl_binary!(f64, powr, vdPowr);
    impl_binary!(f64, hypot, vdHypot);

    impl_unary!(f64, exp2, vdExp2);
    impl_unary!(f64, exp10, vdExp10);
    impl_unary!(f64, expm1, vdExpm1);
    impl_unary!(f64, log2, vdLog2);
    impl_unary!(f64, log10, vdLog10);
    impl_unary!(f64, log1p, vdLog1p);
    impl_unary!(f64, logb, vdLogb);

    impl_unary2!(f64, sin_cos, vdSinCos);
    impl_binary!(f64, atan2, vdAtan2);

    impl_unary!(f64, erf, vdErf);
    impl_unary!(f64, erfc, vdErfc);
    impl_unary!(f64, cdf_normal, vdCdfNorm);
    impl_unary!(f64, erf_inv, vdErfInv);
    impl_unary!(f64, erfc_inv, vdErfcInv);
    impl_unary!(f64, cdf_normal_inv, vdCdfNormInv);
    impl_unary!(f64, ln_gamma, vdLGamma);
    impl_unary!(f64, gamma, vdTGamma);
    impl_unary!(f64, exp_integral, vdExpInt1);

    impl_unary!(f64, floor, vdFloor);
    impl_unary!(f64, ceil, vdCeil);
    impl_unary!(f64, trunc, vdTrunc);
    impl_unary!(f64, round, vdRound);
    impl_unary!(f64, near_by_int, vdNearbyInt);
    impl_unary!(f64, rint, vdRint);
    impl_unary2!(f64, modf, vdModf);
    impl_unary!(f64, frac, vdFrac);

    impl_binary!(f64, copy_sign, vdCopySign);
    impl_binary!(f64, next_after, vdNextAfter);
    impl_binary!(f64, fdim, vdFdim);
    impl_binary!(f64, fmax, vdFmax);
    impl_binary!(f64, fmin, vdFmin);
    impl_binary!(f64, maxmag, vdMaxMag);
    impl_binary!(f64, minmag, vdMinMag);
}

macro_rules! impl_unary_c {
    ($scalar:ty, $mkl_complex:ty, $name:ident, $impl_name:ident) => {
        fn $name(in_: &[$scalar], out: &mut [$scalar]) {
            assert_eq!(in_.len(), out.len());
            let n = in_.len() as i32;
            unsafe {
                $impl_name(
                    n,
                    in_.as_ptr() as *const $mkl_complex,
                    out.as_mut_ptr() as *mut $mkl_complex,
                )
            }
        }
    };
}

macro_rules! impl_unary_real_c {
    ($scalar:ty, $mkl_complex:ty, $name:ident, $impl_name:ident) => {
        fn $name(in_: &[$scalar], out: &mut [<$scalar as Scalar>::Real]) {
            assert_eq!(in_.len(), out.len());
            let n = in_.len() as i32;
            unsafe {
                $impl_name(
                    n,
                    in_.as_ptr() as *const $mkl_complex,
                    out.as_mut_ptr() as *mut <$scalar as Scalar>::Real,
                )
            }
        }
    };
}

macro_rules! impl_real_unary_c {
    ($scalar:ty, $mkl_complex:ty, $name:ident, $impl_name:ident) => {
        fn $name(in_: &[<$scalar as Scalar>::Real], out: &mut [$scalar]) {
            assert_eq!(in_.len(), out.len());
            let n = in_.len() as i32;
            unsafe {
                $impl_name(
                    n,
                    in_.as_ptr() as *const <$scalar as Scalar>::Real,
                    out.as_mut_ptr() as *mut $mkl_complex,
                )
            }
        }
    };
}

macro_rules! impl_binary_c {
    ($scalar:ty, $mkl_complex:ty, $name:ident, $impl_name:ident) => {
        fn $name(a: &[$scalar], b: &[$scalar], out: &mut [$scalar]) {
            assert_eq!(a.len(), out.len());
            assert_eq!(b.len(), out.len());
            let n = out.len() as i32;
            unsafe {
                $impl_name(
                    n,
                    a.as_ptr() as *const $mkl_complex,
                    b.as_ptr() as *const $mkl_complex,
                    out.as_mut_ptr() as *mut $mkl_complex,
                )
            }
        }
    };
}

macro_rules! impl_binary_scalar_c {
    ($scalar:ty, $mkl_complex:ty, $name:ident, $impl_name:ident) => {
        fn $name(a: &[$scalar], b: $scalar, out: &mut [$scalar]) {
            assert_eq!(a.len(), out.len());
            let n = out.len() as i32;
            unsafe {
                $impl_name(
                    n,
                    a.as_ptr() as *const $mkl_complex,
                    b.into_mkl(),
                    out.as_mut_ptr() as *mut $mkl_complex,
                )
            }
        }
    };
}

trait IntoMKL {
    type Output;
    fn into_mkl(self) -> Self::Output;
}

impl IntoMKL for c32 {
    type Output = MKL_Complex8;
    fn into_mkl(self) -> MKL_Complex8 {
        MKL_Complex8 {
            real: self.re,
            imag: self.im,
        }
    }
}

impl IntoMKL for c64 {
    type Output = MKL_Complex16;
    fn into_mkl(self) -> MKL_Complex16 {
        MKL_Complex16 {
            real: self.re,
            imag: self.im,
        }
    }
}

impl VecMath for c32 {
    impl_binary_c!(c32, MKL_Complex8, add, vcAdd);
    impl_binary_c!(c32, MKL_Complex8, sub, vcSub);
    impl_binary_c!(c32, MKL_Complex8, mul, vcMul);
    impl_unary_real_c!(c32, MKL_Complex8, abs, vcAbs);

    impl_binary_c!(c32, MKL_Complex8, div, vcDiv);
    impl_unary_c!(c32, MKL_Complex8, sqrt, vcSqrt);
    impl_binary_c!(c32, MKL_Complex8, pow, vcPow);
    impl_binary_scalar_c!(c32, MKL_Complex8, powx, vcPowx);

    impl_unary_c!(c32, MKL_Complex8, exp, vcExp);
    impl_unary_c!(c32, MKL_Complex8, ln, vcLn);
    impl_unary_c!(c32, MKL_Complex8, log10, vcLog10);

    impl_unary_c!(c32, MKL_Complex8, cos, vcCos);
    impl_unary_c!(c32, MKL_Complex8, sin, vcSin);
    impl_unary_c!(c32, MKL_Complex8, tan, vcTan);
    impl_unary_c!(c32, MKL_Complex8, acos, vcAcos);
    impl_unary_c!(c32, MKL_Complex8, asin, vcAsin);
    impl_unary_c!(c32, MKL_Complex8, atan, vcAtan);

    impl_unary_c!(c32, MKL_Complex8, cosh, vcCosh);
    impl_unary_c!(c32, MKL_Complex8, sinh, vcSinh);
    impl_unary_c!(c32, MKL_Complex8, tanh, vcTanh);
    impl_unary_c!(c32, MKL_Complex8, acosh, vcAcosh);
    impl_unary_c!(c32, MKL_Complex8, asinh, vcAsinh);
    impl_unary_c!(c32, MKL_Complex8, atanh, vcAtanh);
}

impl VecMath for c64 {
    impl_binary_c!(c64, MKL_Complex16, add, vzAdd);
    impl_binary_c!(c64, MKL_Complex16, sub, vzSub);
    impl_binary_c!(c64, MKL_Complex16, mul, vzMul);
    impl_unary_real_c!(c64, MKL_Complex16, abs, vzAbs);

    impl_binary_c!(c64, MKL_Complex16, div, vzDiv);
    impl_unary_c!(c64, MKL_Complex16, sqrt, vzSqrt);
    impl_binary_c!(c64, MKL_Complex16, pow, vzPow);
    impl_binary_scalar_c!(c64, MKL_Complex16, powx, vzPowx);

    impl_unary_c!(c64, MKL_Complex16, exp, vzExp);
    impl_unary_c!(c64, MKL_Complex16, ln, vzLn);
    impl_unary_c!(c64, MKL_Complex16, log10, vzLog10);

    impl_unary_c!(c64, MKL_Complex16, cos, vzCos);
    impl_unary_c!(c64, MKL_Complex16, sin, vzSin);
    impl_unary_c!(c64, MKL_Complex16, tan, vzTan);
    impl_unary_c!(c64, MKL_Complex16, acos, vzAcos);
    impl_unary_c!(c64, MKL_Complex16, asin, vzAsin);
    impl_unary_c!(c64, MKL_Complex16, atan, vzAtan);

    impl_unary_c!(c64, MKL_Complex16, cosh, vzCosh);
    impl_unary_c!(c64, MKL_Complex16, sinh, vzSinh);
    impl_unary_c!(c64, MKL_Complex16, tanh, vzTanh);
    impl_unary_c!(c64, MKL_Complex16, acosh, vzAcosh);
    impl_unary_c!(c64, MKL_Complex16, asinh, vzAsinh);
    impl_unary_c!(c64, MKL_Complex16, atanh, vzAtanh);
}

impl VecMathComplex for c32 {
    impl_binary_c!(c32, MKL_Complex8, mul_by_conj, vcMulByConj);
    impl_unary_c!(c32, MKL_Complex8, conj, vcConj);
    impl_unary_real_c!(c32, MKL_Complex8, arg, vcArg);
    impl_real_unary_c!(c32, MKL_Complex8, cis, vcCIS);
}

impl VecMathComplex for c64 {
    impl_binary_c!(c64, MKL_Complex16, mul_by_conj, vzMulByConj);
    impl_unary_c!(c64, MKL_Complex16, conj, vzConj);
    impl_unary_real_c!(c64, MKL_Complex16, arg, vzArg);
    impl_real_unary_c!(c64, MKL_Complex16, cis, vzCIS);
}
