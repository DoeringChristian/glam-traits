use crate::traits::{Dot, Length, Lerp, Normalize};

pub fn dot<Rhs, Lhs: Dot<Rhs>>(lhs: Lhs, rhs: Rhs) -> Lhs::Output {
    lhs.dot(rhs)
}

pub fn length<T: Length>(src: T) -> T::Output {
    src.length()
}

pub fn normalize<T: Normalize>(src: T) -> T {
    src.normalize()
}

pub fn lerp<S, T: Lerp<S, Output = T>>(lhs: T, rhs: T, s: S) -> T::Output {
    lhs.lerp(rhs, s)
}

#[cfg(test)]
mod test {
    use super::*;
    use glam::*;
    #[test]
    fn test0() {
        let v0 = Vec4::new(0., 0., 0., 1.);
        let v1 = Vec4::new(0., 0., 1., 0.);
        let f0 = dot(v0, v1);
        assert_eq!(f0, 0.);
    }
}
