macro_rules! arithmetic_ops {
    {
        @new $type:ty, $trait:ident, $func:ident, $intrinsic:ident
    } => {
        impl core::ops::$trait<$type> for $type {
            type Output = Self;
            fn $func(self, rhs: Self) -> Self {
                Self(unsafe { $intrinsic(self.0, rhs.0) })
            }
        }
    };
    {
        @assign $type:ty, $trait:ident, $func:ident, $intrinsic:ident
    } => {
        impl core::ops::$trait<$type> for $type {
            fn $func(&mut self, rhs: Self) {
                self.0 = unsafe { $intrinsic(self.0, rhs.0) };
            }
        }
    };
    {
        for $type:ident:
            add -> $add_intrin:ident,
            sub -> $sub_intrin:ident,
            mul -> $mul_intrin:ident,
            div -> $div_intrin:ident
    } => {
        arithmetic_ops!{@new $type, Add, add, $add_intrin}
        arithmetic_ops!{@new $type, Sub, sub, $sub_intrin}
        arithmetic_ops!{@new $type, Mul, mul, $mul_intrin}
        arithmetic_ops!{@new $type, Div, div, $div_intrin}
        arithmetic_ops!{@assign $type, AddAssign, add_assign, $add_intrin}
        arithmetic_ops!{@assign $type, SubAssign, sub_assign, $sub_intrin}
        arithmetic_ops!{@assign $type, MulAssign, mul_assign, $mul_intrin}
        arithmetic_ops!{@assign $type, DivAssign, div_assign, $div_intrin}
    };
}

macro_rules! as_slice {
    {
        $type:ty
    } => {
        impl AsRef<[<$type as crate::vector::VectorCore>::Scalar]> for $type {
            fn as_ref(&self) -> &[<$type as crate::vector::VectorCore>::Scalar] {
                use crate::vector::VectorCore;
                self.as_slice()
            }
        }

        impl AsMut<[<$type as crate::vector::VectorCore>::Scalar]> for $type {
            fn as_mut(&mut self) -> &mut [<$type as crate::vector::VectorCore>::Scalar] {
                use crate::vector::VectorCore;
                self.as_slice_mut()
            }
        }

        impl<I> core::ops::Index<I> for $type
            where I: core::slice::SliceIndex<[<Self as crate::vector::VectorCore>::Scalar]>
        {
            type Output = <I as core::slice::SliceIndex<[<Self as crate::vector::VectorCore>::Scalar]>>::Output;
            fn index(&self, index: I) -> &Self::Output {
                &self.as_ref()[index]
            }
        }

        impl<I> core::ops::IndexMut<I> for $type
            where I: core::slice::SliceIndex<[<Self as crate::vector::VectorCore>::Scalar]>
        {
            fn index_mut(&mut self, index: I) -> &mut Self::Output {
                &mut self.as_mut()[index]
            }
        }
    }
}
