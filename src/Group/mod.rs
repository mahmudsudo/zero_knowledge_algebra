use crate::Monad::axiom::Monad;

pub trait Group<A: Sized + PartialEq> : Monad<A>{
    fn identity(&self) -> A;
}
