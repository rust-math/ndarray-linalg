
extern crate ndarray;
#[macro_use]
extern crate ndarray_linalg;

use ndarray::prelude::*;
use ndarray_linalg::prelude::*;

#[test]
fn eigen_vector_manual() {
    let a = arr2(&[[3.0, 1.0, 1.0], [1.0, 3.0, 1.0], [1.0, 1.0, 3.0]]);
    let (e, vecs): (Array1<_>, Array2<_>) = (&a).eigh(UPLO::Upper).unwrap();
    assert_close_l2!(&e, &arr1(&[2.0, 2.0, 5.0]), 1.0e-7);
    for (i, v) in vecs.axis_iter(Axis(1)).enumerate() {
        let av = a.dot(&v);
        let ev = v.mapv(|x| e[i] * x);
        assert_close_l2!(&av, &ev, 1.0e-7);
    }
}

#[test]
fn diagonalize() {
    let a = arr2(&[[3.0, 1.0, 1.0], [1.0, 3.0, 1.0], [1.0, 1.0, 3.0]]);
    let (e, vecs): (Array1<_>, Array2<_>) = (&a).eigh(UPLO::Upper).unwrap();
    let s = vecs.t().dot(&a).dot(&vecs);
    for i in 0..3 {
        assert_rclose!(e[i], s[(i, i)], 1e-7);
    }
}
