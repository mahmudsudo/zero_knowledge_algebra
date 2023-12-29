pub trait  Magma<A : Sized + PartialEq> {
    fn closure(&self, _x: &Self) -> bool {
        true
      }
    fn op(&self, other: &Self, operation : dyn Fn(A,A) -> A) -> Self;
}
