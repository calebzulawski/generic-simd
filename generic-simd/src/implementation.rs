macro_rules! arithmetic_ops {
    {
        @new $type:ty, $feature:expr, $trait:ident, $func:ident, default
    } => {
        impl core::ops::$trait<$type> for $type {
            type Output = Self;
            #[allow(unused_unsafe)]
            #[inline]
            fn $func(mut self, rhs: Self) -> Self {
                for (a, b) in self.iter_mut().zip(b.iter()) {
                    *a = $trait::$func(*a, b);
                }
                self
            }
        }

        impl core::ops::$trait<<$type as $crate::vector::Vector>::Scalar> for $type {
            type Output = Self;
            #[inline]
            fn $func(mut self, rhs: <$type as $crate::vector::Vector>::Scalar) -> Self {
                for a in self.iter_mut() {
                    *a = $trait::$func(*a, rhs);
                }
                self
            }
        }
    };
    {
        @assign $type:ty, $feature:expr, $trait:ident, $func:ident, default
    } => {
        impl core::ops::$trait<$type> for $type {
            #[allow(unused_unsafe)]
            #[inline]
            fn $func(&mut self, rhs: Self) {
                for (a, b) in self.iter_mut().zip(b.iter()) {
                    $trait::$func(a, b);
                }
            }
        }

        impl core::ops::$trait<<$type as $crate::vector::Vector>::Scalar> for $type {
            #[inline]
            fn $func(&mut self, rhs: <$type as $crate::vector::Vector>::Scalar) {
                for a in self.iter_mut() {
                    $trait::$func(a, rhs);
                }
            }
        }
    };
    {
        @new $type:ty, $feature:expr, $trait:ident, $func:ident, $op:path
    } => {
        impl core::ops::$trait<$type> for $type {
            type Output = Self;
            #[allow(unused_unsafe)]
            #[inline]
            fn $func(self, rhs: Self) -> Self {
                Self(unsafe { $op(self.0, rhs.0) })
            }
        }

        impl core::ops::$trait<<$type as $crate::vector::Vector>::Scalar> for $type {
            type Output = Self;
            #[inline]
            fn $func(self, rhs: <$type as $crate::vector::Vector>::Scalar) -> Self {
                self.$func(<$type>::splat(unsafe { $feature }, rhs))
            }
        }
    };
    {
        @assign $type:ty, $feature:expr, $trait:ident, $func:ident, $op:path
    } => {
        impl core::ops::$trait<$type> for $type {
            #[allow(unused_unsafe)]
            #[inline]
            fn $func(&mut self, rhs: Self) {
                self.0 = unsafe { $op(self.0, rhs.0) };
            }
        }

        impl core::ops::$trait<<$type as $crate::vector::Vector>::Scalar> for $type {
            #[inline]
            fn $func(&mut self, rhs: <$type as $crate::vector::Vector>::Scalar) {
                self.$func(<$type>::splat(unsafe { $feature }, rhs))
            }
        }
    };
    {
        feature: $feature:expr,
        for $type:ty:
            add -> $add_expr:path,
            sub -> $sub_expr:path,
            mul -> $mul_expr:path,
            div -> $div_expr:path
    } => {
        impl core::iter::Sum<$type> for Option<$type> {
            #[inline]
            fn sum<I>(mut iter: I) -> Self
            where
                I: Iterator<Item = $type>,
            {
                if let Some(mut sum) = iter.next() {
                    while let Some(v) = iter.next() {
                        sum += v;
                    }
                    Some(sum)
                } else {
                    None
                }
            }
        }

        impl core::iter::Sum<$type> for <$type as $crate::vector::Vector>::Scalar {
            #[inline]
            fn sum<I>(iter: I) -> Self
            where
                I: Iterator<Item = $type>,
            {
                if let Some(sums) = iter.sum::<Option<$type>>() {
                    sums.iter().sum()
                } else {
                    Default::default()
                }
            }
        }

        impl core::iter::Product<$type> for Option<$type> {
            #[inline]
            fn product<I>(mut iter: I) -> Self
            where
                I: Iterator<Item = $type>,
            {
                if let Some(mut sum) = iter.next() {
                    while let Some(v) = iter.next() {
                        sum *= v;
                    }
                    Some(sum)
                } else {
                    None
                }
            }
        }

        impl core::iter::Product<$type> for <$type as $crate::vector::Vector>::Scalar {
            #[inline]
            fn product<I>(iter: I) -> Self
            where
                I: Iterator<Item = $type>,
            {
                if let Some(sums) = iter.sum::<Option<$type>>() {
                    sums.iter().product()
                } else {
                    Default::default()
                }
            }
        }

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
            #[inline]
            fn as_ref(&self) -> &[<$type as crate::vector::Vector>::Scalar] {
                use crate::vector::Vector;
                self.as_slice()
            }
        }

        impl AsMut<[<$type as crate::vector::Vector>::Scalar]> for $type {
            #[inline]
            fn as_mut(&mut self) -> &mut [<$type as crate::vector::Vector>::Scalar] {
                use crate::vector::Vector;
                self.as_slice_mut()
            }
        }

        impl core::ops::Deref for $type {
            type Target = [<Self as crate::vector::Vector>::Scalar];
            #[inline]
            fn deref(&self) -> &Self::Target {
                self.as_slice()
            }
        }

        impl core::ops::DerefMut for $type {
            #[inline]
            fn deref_mut(&mut self) -> &mut <Self as core::ops::Deref>::Target {
                self.as_slice_mut()
            }
        }
    }
}
