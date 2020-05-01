use safe_simd::vector::Widest;

#[safe_simd::dispatch(feature)]
pub fn add_one_aligned(x: &mut [f32]) {
    let (start, vecs, end) = feature.align_widest_mut(x.as_mut());
    for s in start.iter_mut().chain(end.iter_mut()) {
        *s += 1.;
    }

    let ones = feature.splat_widest(1f32);
    for v in vecs {
        *v = *v + ones;
    }

    for s in x.iter() {
        assert_eq!(*s, 1f32);
    }
}
