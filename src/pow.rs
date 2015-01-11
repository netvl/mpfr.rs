pub trait Pow<RHS = Self> {
    type Output;
    fn pow(self, rhs: RHS) -> Self::Output;
}
