pub trait Specification<T> {
    fn is_satisfied_by(&self, candidate: &T) -> bool;
}

pub struct CompositeSpecification<T> {
    child_specifications: Vec<Box<dyn Specification<T>>>,
}

#[allow(dead_code)]
#[allow(unused_variables)]
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
                child_specifications: Vec::new(),
            },
        }
    }

    pub fn add_child_specifications(&mut self, child: Box<dyn Specification<T>>) {
        self.specification.add_child_specifications(child);
    }

    pub fn children(&self) -> &Vec<Box<dyn Specification<T>>> {
        &self.specification.children()
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

pub struct OrSpecification<T> {
    specification: CompositeSpecification<T>,
}

impl<T> OrSpecification<T> {
    pub fn new() -> OrSpecification<T> {
        OrSpecification {
            specification: CompositeSpecification {
                child_specifications: Vec::new(),
            },
        }
    }

    pub fn add_child_specifications(&mut self, child: Box<dyn Specification<T>>) {
        self.specification.add_child_specifications(child);
    }

    pub fn children(&self) -> &Vec<Box<dyn Specification<T>>> {
        &self.specification.children()
    }
}

impl<T> Specification<T> for OrSpecification<T> {
    fn is_satisfied_by(&self, candidate: &T) -> bool {
        if self.children().len() <= 0 {
            return false;
        }

        for child in self.children() {
            if child.is_satisfied_by(candidate) {
                return true;
            }
        }

        false
    }
}

pub struct XorSpecification<T> {
    specification: CompositeSpecification<T>,
}

impl<T> XorSpecification<T> {
    pub fn new() -> XorSpecification<T> {
        XorSpecification {
            specification: CompositeSpecification {
                child_specifications: Vec::new(),
            },
        }
    }

    pub fn add_child_specifications(&mut self, child: Box<dyn Specification<T>>) {
        self.specification.add_child_specifications(child);
    }

    pub fn children(&self) -> &Vec<Box<dyn Specification<T>>> {
        &self.specification.children()
    }
}

impl<T> Specification<T> for XorSpecification<T> {
    fn is_satisfied_by(&self, candidate: &T) -> bool {
        if self.children().len() <= 0 {
            return false;
        }

        let mut flag = false;
        for child in self.children() {
            match child.is_satisfied_by(candidate) {
                true => {
                    if flag {
                        return false;
                    } else {
                        flag = true;
                    }
                }
                false => continue,
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

    #[allow(dead_code)]
    #[allow(unused_variables)]
    impl<T> Specification<T> for TrueSpecification<T> {
        fn is_satisfied_by(&self, candidate: &T) -> bool {
            true
        }
    }

    #[allow(dead_code)]
    #[allow(unused_variables)]
    impl<T> Specification<T> for FalseSpecification<T> {
        fn is_satisfied_by(&self, candidate: &T) -> bool {
            false
        }
    }

    #[test]
    fn and() {
        // Arrange
        let mut spec = AndSpecification::<TestMessage>::new();
        spec.add_child_specifications(Box::new(TrueSpecification::<TestMessage>::new()));
        spec.add_child_specifications(Box::new(TrueSpecification::<TestMessage>::new()));
        let message = TestMessage {};
        let result = spec.is_satisfied_by(&Box::new(message));

        assert_eq!(true, result);
    }

    #[test]
    fn or() {
        let mut spec = OrSpecification::<TestMessage>::new();
        spec.add_child_specifications(Box::new(TrueSpecification::<TestMessage>::new()));
        spec.add_child_specifications(Box::new(FalseSpecification::<TestMessage>::new()));
        let message = TestMessage {};
        let result = spec.is_satisfied_by(&Box::new(message));

        assert_eq!(true, result);

        let mut spec = OrSpecification::<TestMessage>::new();
        spec.add_child_specifications(Box::new(TrueSpecification::<TestMessage>::new()));
        spec.add_child_specifications(Box::new(TrueSpecification::<TestMessage>::new()));
        let message = TestMessage {};
        let result = spec.is_satisfied_by(&Box::new(message));

        assert_eq!(true, result);
    }

    #[test]
    fn xor() {
        let mut spec = XorSpecification::<TestMessage>::new();
        spec.add_child_specifications(Box::new(TrueSpecification::<TestMessage>::new()));
        spec.add_child_specifications(Box::new(FalseSpecification::<TestMessage>::new()));
        let result = spec.is_satisfied_by(&Box::new(TestMessage {}));
        assert_eq!(true, result);

        spec.add_child_specifications(Box::new(TrueSpecification::<TestMessage>::new()));
        let result = spec.is_satisfied_by(&Box::new(TestMessage {}));
        assert_eq!(false, result);
    }
}
