use crate::Semigroup::axiom::Semigroup;

pub trait Monad<A: PartialEq+Sized> : Semigroup<A>{
    fn identity(&self) -> A ;
}