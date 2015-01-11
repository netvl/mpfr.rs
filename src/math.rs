pub trait MathExt {
    fn sqr(self) -> Self;
    fn sqrt(self) -> Self;
    fn sqrt_rec(self) -> Self;
    fn cbrt(self) -> Self;
    fn root(self, k: u32) -> Self;
}

