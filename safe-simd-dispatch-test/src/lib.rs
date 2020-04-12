use safe_simd::vector::{Feature, Loader};

#[safe_simd::dispatch(handle)]
pub fn add_one_aligned() {
    let mut x = [0f32; 32];

    let (start, vecs, end) = handle.align_mut(x.as_mut());
    for s in start.iter_mut().chain(end.iter_mut()) {
        *s += 1.;
    }

    let ones = handle.splat(1f32);
    for v in vecs {
        *v += ones;
    }

    for s in x.iter() {
        assert_eq!(*s, 1f32);
    }
}
