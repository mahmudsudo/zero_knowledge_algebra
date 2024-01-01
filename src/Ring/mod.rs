use crate::Group::Group;
use crate::Monad::axiom::Monad;

pub trait Ring<A : Sized + PartialEq> : Group<A> + Monad<A> {
    fn op1(&self, other: &Self, operation : dyn Fn(A,A) -> A) -> Self;
    fn op2(&self, other: &Self, operation : dyn Fn(A,A) -> A) -> Self;
    fn distribute(&self, op1 : dyn Fn(A,A) -> A,op2 : dyn Fn(A,A) -> A) -> bool ;
}  