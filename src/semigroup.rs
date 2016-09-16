#![allow(dead_code, unused)]
use lift::SemiGroup;
use std::hash::Hash;
use std::collections::linked_list::LinkedList;
use std::collections::vec_deque::VecDeque;
use std::collections::{BinaryHeap, BTreeSet, HashSet};

//macro for making semigroups out of types that implement Extend
#[macro_export]
macro_rules! semigroup {
    ($t:ident) => {
        impl<T: Clone> SemiGroup for $t<T> {
            type A = $t<T>;
            fn add(&self, b: &Self::A) -> Self::A {
                let mut ret = $t::new();
                ret.extend(self.iter().cloned());
                ret.extend(b.iter().cloned());
                ret
            }
        }
    }
}

//macro for making semigroup out of extendable things that require ordered inputs
//e.g BTreeSet, BinaryHeap
#[macro_export]
macro_rules! semigroup_ord {
    ($t:ident) => {
        impl<T: Clone + Ord> SemiGroup for $t<T> {
            type A = $t<T>;
            fn add(&self, b: &Self::A) -> Self::A {
                let mut ret = $t::new();
                ret.extend(self.iter().cloned());
                ret.extend(b.iter().cloned());
                ret
            }
        }
    }
}

//Implementataion of SemiGroup for String
impl SemiGroup for String {
    type A = String;

    fn add(&self, b: &Self::A) -> Self::A {
        let mut ret = String::from("");
        ret.push_str(&self);
        ret.push_str(&b);
        ret
    }
}

//Implementation for SemiGroup for HashSet
impl<T: Clone + Hash + Eq> SemiGroup for HashSet<T> {
    type A = HashSet<T>;

    fn add(&self, b: &Self::A) -> Self::A {
        let mut ret = HashSet::new();
        ret.extend(self.iter().cloned());
        ret.extend(b.iter().cloned());
        ret
    }
}

//Implementation of SemiGroup for Vec<T>
semigroup!(Vec);

//Implementation of SemiGroup for LinkedList<T>
semigroup!(LinkedList);

//Implementation of SemiGroup for VecDeque<T>
semigroup!(VecDeque);

//Implementation of SemiGroup for BinaryHeap<T>
semigroup_ord!(BinaryHeap);

//Implemenatation of SemiGroup for BTreeSet<T>
semigroup_ord!(BTreeSet);

#[cfg(test)]
mod test {
    use lift::{SemiGroup};
    
    #[test]
    fn test_vec() {
        let one = vec!(1,2);
        let two = vec!(3,4);

        assert_eq!(one.add(&two), vec!(1,2,3,4));
        assert_eq!(one, vec!(1,2));
    }
    
    #[test]
    fn test_string() {
        let one = String::from("one");
        let two = String::from("two");

        assert_eq!(one.add(&two), String::from("onetwo")); 
    }
}
