macro_rules! arithmetic_ops {
    {
        @new $type:ty, $feature:expr, $trait:ident, $func:ident, $op:expr
    } => {
        impl core::ops::$trait<$type> for $type {
            type Output = Self;
            #[allow(unused_unsafe)]
            #[inline]
            fn $func(self, rhs: Self) -> Self {
                Self(unsafe { $op(self.0, rhs.0) })
            }
        }

        impl core::ops::$trait<<$type as Vector>::Scalar> for $type {
            type Output = Self;
            #[inline]
            fn $func(self, rhs: <$type as Vector>::Scalar) -> Self {
                self.$func(<$type>::splat(unsafe { $feature }, rhs))
            }
        }
    };
    {
        @assign $type:ty, $feature:expr, $trait:ident, $func:ident, $op:expr
    } => {
        impl core::ops::$trait<$type> for $type {
            #[allow(unused_unsafe)]
            #[inline]
            fn $func(&mut self, rhs: Self) {
                self.0 = unsafe { $op(self.0, rhs.0) };
            }
        }

        impl core::ops::$trait<<$type as Vector>::Scalar> for $type {
            #[inline]
            fn $func(&mut self, rhs: <$type as Vector>::Scalar) {
                self.$func(<$type>::splat(unsafe { $feature }, rhs))
            }
        }
    };
    {
        feature: $feature:expr,
        for $type:ty:
            add -> $add_expr:expr,
            sub -> $sub_expr:expr,
            mul -> $mul_expr:expr,
            div -> $div_expr:expr
    } => {
        arithmetic_ops!{@new $type, $feature, Add, add, $add_expr}
        arithmetic_ops!{@new $type, $feature, Sub, sub, $sub_expr}
        arithmetic_ops!{@new $type, $feature, Mul, mul, $mul_expr}
        arithmetic_ops!{@new $type, $feature, Div, div, $div_expr}
        arithmetic_ops!{@assign $type, $feature, AddAssign, add_assign, $add_expr}
        arithmetic_ops!{@assign $type, $feature, SubAssign, sub_assign, $sub_expr}
        arithmetic_ops!{@assign $type, $feature, MulAssign, mul_assign, $mul_expr}
        arithmetic_ops!{@assign $type, $feature, DivAssign, div_assign, $div_expr}
    };
}

macro_rules! as_slice {
    {
        $type:ty
    } => {
        impl AsRef<[<$type as crate::vector::Vector>::Scalar]> for $type {
            fn as_ref(&self) -> &[<$type as crate::vector::Vector>::Scalar] {
                use crate::vector::Vector;
                self.as_slice()
            }
        }

        impl AsMut<[<$type as crate::vector::Vector>::Scalar]> for $type {
            fn as_mut(&mut self) -> &mut [<$type as crate::vector::Vector>::Scalar] {
                use crate::vector::Vector;
                self.as_slice_mut()
            }
        }

        impl core::ops::Deref for $type {
            type Target = [<Self as crate::vector::Vector>::Scalar];
            fn deref(&self) -> &Self::Target {
                self.as_slice()
            }
        }

        impl core::ops::DerefMut for $type {
            fn deref_mut(&mut self) -> &mut <Self as core::ops::Deref>::Target {
                self.as_slice_mut()
            }
        }
    }
}
