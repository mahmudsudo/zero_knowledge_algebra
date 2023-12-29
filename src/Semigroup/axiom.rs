use crate::magma::axiom::Magma;
pub trait Semigroup<A :PartialEq + Sized> : Magma<A>{
 fn associative() -> bool;
}