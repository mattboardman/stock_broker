pub trait Specification<T> {
    fn is_satisfied_by(&self, candidate: &T) -> bool;
}

pub struct CompositeSpecification<T> {
    child_specifications: Vec<Box<dyn Specification<T>>>,
}

impl<T> CompositeSpecification<T> {
    fn add_child_specifications(&mut self, child: Box<dyn Specification<T>>) {
        self.child_specifications.push(child);
    }

    fn children(&self) -> &Vec<Box<dyn Specification<T>>> {
        &self.child_specifications
    }
}

pub struct AndSpecification<T> {
    specification: CompositeSpecification<T>,
}

impl<T> AndSpecification<T> {
    pub fn new() -> AndSpecification<T> {
        AndSpecification {
            specification: CompositeSpecification {
                child_specifications: Vec::default(),
            },
        }
    }

    pub fn add_child_specifications(&mut self, child: Box<dyn Specification<T>>) {
        self.specification.add_child_specifications(child);
    }

    pub fn children(&self) -> &Vec<Box<dyn Specification<T>>> {
        &self.specification.child_specifications
    }
}

impl<T> Specification<T> for AndSpecification<T> {
    fn is_satisfied_by(&self, candidate: &T) -> bool {
        if self.children().len() <= 0 {
            return false;
        }

        for child in self.children() {
            if !child.is_satisfied_by(candidate) {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::marker::PhantomData;

    struct TrueSpecification<T> {
        phantom: PhantomData<T>,
    }

    struct FalseSpecification<T> {
        phantom: PhantomData<T>,
    }

    struct TestMessage {}

    impl<T> TrueSpecification<T> {
        fn new() -> TrueSpecification<T> {
            TrueSpecification::<T> {
                phantom: PhantomData,
            }
        }
    }

    impl<T> FalseSpecification<T> {
        fn new() -> FalseSpecification<T> {
            FalseSpecification::<T> {
                phantom: PhantomData,
            }
        }
    }

    impl<T> Specification<T> for TrueSpecification<T> {
        fn is_satisfied_by(&self, candidate: &T) -> bool {
            true
        }
    }

    impl<T> Specification<T> for FalseSpecification<T> {
        fn is_satisfied_by(&self, candidate: &T) -> bool {
            false
        }
    }

    #[test]
    fn basics() {
        // Arrange
        let mut spec = AndSpecification::<TestMessage>::new();
        spec.add_child_specifications(Box::new(TrueSpecification::<TestMessage>::new()));
        spec.add_child_specifications(Box::new(TrueSpecification::<TestMessage>::new()));
        let message = TestMessage {};
        let result = spec.is_satisfied_by(&Box::new(message));

        assert_eq!(true, result);
    }
}
