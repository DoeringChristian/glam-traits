macro_rules! vec2 {
    ($($e:expr),*) => {
        $crate::glam::Vec2::from(($($e),*))
    }
}
macro_rules! vec3 {
    ($($e:expr),*) => {
        $crate::glam::Vec3::from(($($e),*))
    }
}
macro_rules! vec4 {
    ($($e:expr),*) => {
        $crate::glam::Vec4::from(($($e),*))
    }
}

macro_rules! quat{
    ($($e:expr),*) => {
        $crate::glam::Quat::from(($($e),*))
    }
}

#[cfg(test)]
mod test {
    use glam::*;
    #[test]
    fn test() {
        let v0 = vec2!(0., 1.);
        assert_eq!(v0, vec2(0., 1.));

        let v0 = vec3!(v0, 2.);
        assert_eq!(v0, vec3(0., 1., 2.));

        let v0 = vec4!(v0, 3.);
        assert_eq!(v0, vec4(0., 1., 2., 3.));
    }
}
