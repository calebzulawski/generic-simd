//! Slices of vectors.

use crate::arch::Token;
use crate::vector::{width, Native, NativeWidth, ScalarSized, Vector};
use core::marker::PhantomData;

/// A slice of scalars.
pub trait SliceSized<Token, Width>
where
    Token: crate::arch::Token,
    Width: width::Width,
{
    type Token: crate::arch::Token + From<Token>;
    type Vector: Vector<Token = Self::Token, Width = Width>;

    /// Extract a slice of aligned vectors, as if by [`align_to`].
    ///
    /// [`align_to`]: https://doc.rust-lang.org/std/primitive.slice.html#method.align_to
    #[allow(clippy::type_complexity)]
    fn align(
        &self,
        #[allow(unused_variables)] token: Self::Token,
    ) -> (
        &[<Self::Vector as Vector>::Scalar],
        &[Self::Vector],
        &[<Self::Vector as Vector>::Scalar],
    );

    /// Extract a slice of aligned mutable vectors, as if by [`align_to_mut`].
    ///
    /// [`align_to_mut`]: https://doc.rust-lang.org/std/primitive.slice.html#method.align_to_mut
    #[allow(clippy::type_complexity)]
    fn align_mut(
        &mut self,
        #[allow(unused_variables)] token: Self::Token,
    ) -> (
        &mut [<Self::Vector as Vector>::Scalar],
        &mut [Self::Vector],
        &mut [<Self::Vector as Vector>::Scalar],
    );

    /// Create a slice of overlapping vectors from a slice of scalars.
    fn overlapping(&self, token: Self::Token) -> Overlapping<'_, Self::Vector>;

    /// Create a mutable slice of overlapping vectors from a slice of scalars.
    fn overlapping_mut(&mut self, token: Self::Token) -> OverlappingMut<'_, Self::Vector>;
}

impl<T, Token, Width> SliceSized<Token, Width> for [T]
where
    T: ScalarSized<Token, Width>,
    Token: crate::arch::Token,
    Width: width::Width,
{
    type Token = T::Token;
    type Vector = T::Vector;

    #[allow(clippy::type_complexity)]
    #[inline]
    fn align(
        &self,
        #[allow(unused_variables)] token: Self::Token,
    ) -> (
        &[<Self::Vector as Vector>::Scalar],
        &[Self::Vector],
        &[<Self::Vector as Vector>::Scalar],
    ) {
        unsafe { self.align_to() }
    }

    #[allow(clippy::type_complexity)]
    #[inline]
    fn align_mut(
        &mut self,
        #[allow(unused_variables)] token: Self::Token,
    ) -> (
        &mut [<Self::Vector as Vector>::Scalar],
        &mut [Self::Vector],
        &mut [<Self::Vector as Vector>::Scalar],
    ) {
        unsafe { self.align_to_mut() }
    }

    #[inline]
    fn overlapping(&self, token: Self::Token) -> Overlapping<'_, Self::Vector> {
        Overlapping::new(token, self)
    }

    #[inline]
    fn overlapping_mut(&mut self, token: Self::Token) -> OverlappingMut<'_, Self::Vector> {
        OverlappingMut::new(token, self)
    }
}

macro_rules! slice_impl {
    {
        $width:literal,
        $width_type:ty,
        $align:ident,
        $align_mut:ident,
        $overlapping:ident,
        $overlapping_mut:ident
    } => {
        #[doc = "Align a slice of scalars to vectors with "]
        #[doc = $width]
        #[doc = ".\n\nSee [`align`](trait.SliceSized.html#tymethod.align)."]
        #[allow(clippy::type_complexity)]
        #[inline]
        fn $align(&self, token: Token) ->
        (
            &[<<Self as SliceSized<Token, $width_type>>::Vector as Vector>::Scalar],
            &[<Self as SliceSized<Token, $width_type>>::Vector],
            &[<<Self as SliceSized<Token, $width_type>>::Vector as Vector>::Scalar],
        ) {
            <Self as SliceSized<Token, $width_type>>::align(self, <Self as SliceSized<Token, $width_type>>::Token::from(token))
        }

        #[doc = "Align a slice of scalars to vectors with "]
        #[doc = $width]
        #[doc = ".\n\nSee [`align_mut`](trait.SliceSized.html#tymethod.align_mut)."]
        #[allow(clippy::type_complexity)]
        #[inline]
        fn $align_mut(&mut self, token: Token) ->
        (
            &mut [<<Self as SliceSized<Token, $width_type>>::Vector as Vector>::Scalar],
            &mut [<Self as SliceSized<Token, $width_type>>::Vector],
            &mut [<<Self as SliceSized<Token, $width_type>>::Vector as Vector>::Scalar],
        ){
            <Self as SliceSized<Token, $width_type>>::align_mut(self, <Self as SliceSized<Token, $width_type>>::Token::from(token))
        }

        #[doc = "Create a slice of overlapping vectors of "]
        #[doc = $width]
        #[doc = "from a slice of scalars.\n\nSee [`overlapping`](trait.SliceSized.html#tymethod.overlapping)."]
        #[inline]
        fn $overlapping(&self, token: Token) -> crate::slice::Overlapping<'_, <Self as SliceSized<Token, $width_type>>::Vector> {
            <Self as SliceSized<Token, $width_type>>::overlapping(self, <Self as SliceSized<Token, $width_type>>::Token::from(token))
        }

        #[doc = "Create a mutable slice of overlapping vectors of "]
        #[doc = $width]
        #[doc = "from a slice of scalars.\n\nSee [`overlapping_mut`](trait.SliceSized.html#tymethod.overlapping_mut)."]
        #[inline]
        fn $overlapping_mut(
            &mut self,
            token: Token,
        ) -> crate::slice::OverlappingMut<'_, <Self as SliceSized<Token, $width_type>>::Vector> {
            <Self as SliceSized<Token, $width_type>>::overlapping_mut(self, <Self as SliceSized<Token, $width_type>>::Token::from(token))
        }
    }
}

impl<T, Token> Native<Token> for [T]
where
    T: Native<Token>,
{
    type Width = T::Width;
}

/// A pointer to a vector.
pub trait Slice<Token>:
    Native<Token>
    + SliceSized<Token, width::W1>
    + SliceSized<Token, width::W2>
    + SliceSized<Token, width::W4>
    + SliceSized<Token, width::W8>
    + SliceSized<Token, NativeWidth<Self, Token>>
where
    Token: crate::arch::Token,
{
    slice_impl! { "the native number of lanes", <Self as Native<Token>>::Width, align_native, align_native_mut, overlapping_native, overlapping_native_mut }
    slice_impl! { "1 lane",   width::W1, align1, align1_mut, overlapping1, overlapping1_mut }
    slice_impl! { "2 lanes",  width::W2, align2, align2_mut, overlapping2, overlapping2_mut }
    slice_impl! { "4 lanes",  width::W4, align4, align4_mut, overlapping4, overlapping4_mut }
    slice_impl! { "8 lanes",  width::W8, align8, align8_mut, overlapping8, overlapping8_mut }
}

impl<T, Token> Slice<Token> for T
where
    T: ?Sized
        + Native<Token>
        + SliceSized<Token, width::W1>
        + SliceSized<Token, width::W2>
        + SliceSized<Token, width::W4>
        + SliceSized<Token, width::W8>
        + SliceSized<Token, NativeWidth<Self, Token>>,
    Token: crate::arch::Token,
{
}

/// Wrapper for producing a mutable reference from an unaligned pointer.
pub struct RefMut<'a, V>
where
    V: Vector,
{
    source: *mut V::Scalar,
    temp: V,
    lifetime: PhantomData<&'a V::Scalar>,
}

impl<'a, V> RefMut<'a, V>
where
    V: Vector,
{
    fn new(token: V::Token, source: *mut V::Scalar) -> Self {
        Self {
            source,
            temp: V::zeroed(token),
            lifetime: PhantomData,
        }
    }
}

impl<'a, V> core::ops::Deref for RefMut<'a, V>
where
    V: Vector,
{
    type Target = V;
    fn deref(&self) -> &V {
        &self.temp
    }
}

impl<'a, V> core::ops::DerefMut for RefMut<'a, V>
where
    V: Vector,
{
    fn deref_mut(&mut self) -> &mut V {
        &mut self.temp
    }
}

impl<'a, V> core::ops::Drop for RefMut<'a, V>
where
    V: Vector,
{
    fn drop(&mut self) {
        unsafe {
            self.temp.write_ptr(self.source);
        }
    }
}

/// Wrapper for indexing into overlapping vectors.
pub struct Overlapping<'a, V>
where
    V: Vector,
{
    slice: &'a [V::Scalar],
    phantom: PhantomData<V>,
}

#[allow(clippy::len_without_is_empty)]
impl<'a, V> Overlapping<'a, V>
where
    V: Vector,
{
    /// Create a new overlapping vector slice.
    #[inline]
    pub fn new(
        #[allow(unused_variables)] token: impl Into<V::Token>,
        slice: &'a [V::Scalar],
    ) -> Self {
        assert!(
            slice.len() >= V::width(),
            "slice must be at least as wide as the vector"
        );
        Self {
            slice,
            phantom: PhantomData,
        }
    }

    /// Returns the number of overlapping vectors.
    ///
    /// Equal to `slice.len() - V::width() + 1`.
    #[inline]
    pub fn len(&self) -> usize {
        self.slice.len() - V::width() + 1
    }

    /// Returns the vector offset `index` into the slice of scalars.
    #[inline]
    pub fn get(&self, index: usize) -> Option<V> {
        if index < self.len() {
            Some(unsafe { self.get_unchecked(index) })
        } else {
            None
        }
    }

    /// Returns the vector offset `index` into the slice of scalars.
    ///
    /// # Safety
    /// Index must be less than `len()`, i.e. the underlying slice must be at least `index
    /// + V::width()` long.
    #[inline]
    pub unsafe fn get_unchecked(&self, index: usize) -> V
    where
        V: Vector,
    {
        V::read_ptr(V::Token::new_unchecked(), self.slice.as_ptr().add(index))
    }
}

/// Wrapper for indexing into overlapping mutable vectors.
pub struct OverlappingMut<'a, V>
where
    V: Vector,
{
    slice: &'a mut [V::Scalar],
    phantom: PhantomData<V>,
}

#[allow(clippy::len_without_is_empty)]
impl<'a, V> OverlappingMut<'a, V>
where
    V: Vector,
{
    /// Create a new overlapping vector slice.
    #[inline]
    pub fn new(
        #[allow(unused_variables)] token: impl Into<V::Token>,
        slice: &'a mut [V::Scalar],
    ) -> Self {
        assert!(
            slice.len() >= V::width(),
            "slice must be at least as wide as the vector"
        );
        Self {
            slice,
            phantom: PhantomData,
        }
    }

    /// Returns the number of overlapping vectors.
    ///
    /// Equal to `slice.len() - V::width() + 1`.
    #[inline]
    pub fn len(&self) -> usize {
        self.slice.len() - V::width() + 1
    }

    /// Returns the vector offset `index` into the slice of scalars.
    #[inline]
    pub fn get(&self, index: usize) -> Option<V> {
        if index < self.len() {
            Some(unsafe { self.get_unchecked(index) })
        } else {
            None
        }
    }

    /// Returns the vector offset `index` into the slice of scalars.
    ///
    /// # Safety
    /// Index must be less than `len()`, i.e. the underlying slice must be at least `index
    /// + V::width()` long.
    #[inline]
    pub unsafe fn get_unchecked(&self, index: usize) -> V {
        V::read_ptr(V::Token::new_unchecked(), self.slice.as_ptr().add(index))
    }

    /// Returns the mutable vector offset `index` into the slice of scalars.
    #[inline]
    pub fn get_mut(&'a mut self, index: usize) -> Option<RefMut<'a, V>> {
        if index < self.len() {
            Some(unsafe { self.get_unchecked_mut(index) })
        } else {
            None
        }
    }

    /// Returns the mutable vector offset `index` into the slice of scalars.
    ///
    /// # Safety
    /// Index must be less than `len()`, i.e. the underlying slice must be at least `index
    /// + V::width()` long.
    #[inline]
    pub unsafe fn get_unchecked_mut(&'a mut self, index: usize) -> RefMut<'a, V> {
        RefMut::new(
            V::Token::new_unchecked(),
            self.slice.as_mut_ptr().add(index),
        )
    }
}
