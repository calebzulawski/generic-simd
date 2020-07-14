use generic_simd::vector::Handle;

#[generic_simd::dispatch(feature)]
pub fn add_one_aligned(x: &mut [f32]) {
    let (start, vecs, end) = feature.align_native_mut(x.as_mut());
    for s in start.iter_mut().chain(end.iter_mut()) {
        *s += 1.;
    }

    for v in vecs {
        *v += 1.;
    }
}

#[generic_simd::dispatch(_feature)]
pub fn add_one_aligned_dispatch(x: &mut [f32]) {
    dispatch!(add_one_aligned(x))
}
