// Implementation of arithmetic operations for BigFloat.
// Basically, all operations are implemented both for BigFloat and &BigFloat as RHS and LHS
// (4 variants total: value + value, value + reference, reference + value, reference + reference).
//
// If one of the operands is a value, then it is used to hold the result. If both of the
// operands are values, LHS gets a priority. If both of the operands are references,
// the LHS is cloned and used to hold the result.
//
// The order is important if participating values have different precisions.
//
// If one of the operands is of primitive type, then either the BigFloat operand will
// be reused for the result or (if it is a reference) it will be cloned.

// Commutative operations (+, *)

macro_rules! impl_commutative_op_val_ref {
    ($tr:ident, $meth:ident, $mpfr:ident) => {
        impl<'a> $tr<&'a BigFloat> for BigFloat {
            type Output = BigFloat;

            fn $meth(mut self, rhs: &'a BigFloat) -> BigFloat {
                unsafe {
                    $mpfr(&mut self.value, &self.value, &rhs.value, grnd());
                }
                self
            }
        }
    }
}

macro_rules! impl_commutative_op_val_val {
    ($tr:ident, $meth:ident) => {
        impl $tr<BigFloat> for BigFloat {
            type Output = BigFloat;

            #[inline]
            fn $meth(self, rhs: BigFloat) -> BigFloat {
                self.$meth(&rhs)
            }
        }
    }
}

macro_rules! impl_commutative_op_ref_val {
    ($tr:ident, $meth:ident) => {
        impl<'r> $tr<BigFloat> for &'r BigFloat {
            type Output = BigFloat;

            #[inline]
            fn $meth(self, rhs: BigFloat) -> BigFloat {
                rhs.$meth(self)
            }
        }
    }
}

macro_rules! impl_commutative_op_ref_ref {
    ($tr:ident, $meth:ident) => {
        impl<'a, 'r> $tr<&'a BigFloat> for &'r BigFloat {
            type Output = BigFloat;

            #[inline]
            fn $meth(self, rhs: &'a BigFloat) -> BigFloat {
                self.clone().$meth(rhs)
            }
        }
    }
}

macro_rules! impl_commutative_op_val_prim {
    ($tr:ident, $meth:ident, $prim:ty, $c_prim:ty, $mpfr:ident) => {
        impl $tr<$prim> for BigFloat {
            type Output = BigFloat;

            fn $meth(mut self, rhs: $prim) -> BigFloat {
                unsafe {
                    $mpfr(&mut self.value, &self.value, rhs as $c_prim, grnd());
                }
                self
            }
        }
    }
}

macro_rules! impl_commutative_op_ref_prim {
    ($tr:ident, $meth:ident, $prim:ty) => {
        impl<'r> $tr<$prim> for &'r BigFloat {
            type Output = BigFloat;

            #[inline]
            fn $meth(self, rhs: $prim) -> BigFloat {
                self.clone() + rhs
            }
        }
    }
}

macro_rules! impl_commutative_op_prim_val {
    ($tr:ident, $meth:ident, $prim:ty) => {
        impl $tr<BigFloat> for $prim {
            type Output = BigFloat;

            #[inline]
            fn $meth(self, rhs: BigFloat) -> BigFloat {
                rhs + self
            }
        }
    }
}

macro_rules! impl_commutative_op_prim_ref {
    ($tr:ident, $meth:ident, $prim:ty) => {
        impl<'r> $tr<&'r BigFloat> for $prim {
            type Output = BigFloat;

            #[inline]
            fn add(self, rhs: &'r BigFloat) -> BigFloat {
                rhs + self
            }
        }
    }
}

macro_rules! impl_commutative_op {
    ($tr:ident, $meth:ident, $mpfr:ident, $mpfr_f64:ident) => {
        impl_commutative_op_val_ref! { $tr, $meth, $mpfr }
        impl_commutative_op_val_val! { $tr, $meth }
        impl_commutative_op_ref_val! { $tr, $meth }
        impl_commutative_op_ref_ref! { $tr, $meth }
        impl_commutative_op_val_prim! { $tr, $meth, f64, c_double, $mpfr_f64 }
        impl_commutative_op_ref_prim! { $tr, $meth, f64 }
        // Does not work now due to trait coherence rules :(
        //impl_commutative_op_prim_val! { $tr, $meth, f64 }
        //impl_commutative_op_prim_ref! { $tr, $meth, f64 }
    }
}

// Non-commutative operations (-, /, **)

macro_rules! impl_noncommutative_op_val_ref {
    ($tr:ident, $meth:ident, $mpfr:ident) => { impl_commutative_op_val_ref! { $tr, $meth, $mpfr } }
}

macro_rules! impl_noncommutative_op_val_val {
    ($tr:ident, $meth:ident) => { impl_commutative_op_val_val! { $tr, $meth } }
}

macro_rules! impl_noncommutative_op_ref_ref {
    ($tr:ident, $meth:ident) => { impl_commutative_op_ref_ref! { $tr, $meth } }
}

macro_rules! impl_noncommutative_op_ref_val {
    ($tr:ident, $meth:ident, $mpfr:ident) => {
        impl<'r> $tr<BigFloat> for &'r BigFloat {
            type Output = BigFloat;

            fn $meth(self, mut rhs: BigFloat) -> BigFloat {
                unsafe {
                    $mpfr(&mut rhs.value, &self.value, &rhs.value, grnd());
                }
                rhs
            }
        }
    }
}

macro_rules! impl_noncommutative_op_val_prim {
    ($tr:ident, $meth:ident, $prim:ty, $c_prim:ty, $mpfr:ident) => {
        impl_commutative_op_val_prim! { $tr, $meth, $prim, $c_prim, $mpfr }
    }
}

macro_rules! impl_noncommutative_op_ref_prim {
    ($tr:ident, $meth:ident, $prim:ty) => {
        impl_commutative_op_ref_prim! { $tr, $meth, $prim }
    }
}

macro_rules! impl_noncommutative_op {
    ($tr:ident, $meth:ident, $mpfr:ident, $mpfr_f64:ident) => {
        impl_noncommutative_op_val_ref! { $tr, $meth, $mpfr }
        impl_noncommutative_op_val_val! { $tr, $meth }
        impl_noncommutative_op_ref_val! { $tr, $meth, $mpfr }
        impl_noncommutative_op_ref_ref! { $tr, $meth }
        impl_noncommutative_op_val_prim! { $tr, $meth, f64, c_double, $mpfr_f64 }
    }
}
