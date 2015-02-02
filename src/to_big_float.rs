use BigFloat;
use UpdateBigFloat;
use Precision;

pub trait ToBigFloat {
    fn to_big_float(self) -> BigFloat;
    fn to_big_float_with_prec(self, precision: Precision) -> BigFloat;
}

impl<T: UpdateBigFloat> ToBigFloat for T {
    #[inline]
    fn to_big_float(self) -> BigFloat {
        BigFloat::new().from(self)
    }

    #[inline]
    fn to_big_float_with_prec(self, precision: Precision) -> BigFloat {
        BigFloat::new().with_prec(precision).from(self)
    }
}
