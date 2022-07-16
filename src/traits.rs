pub trait Dot<Rhs = Self> {
    type Output;
    fn dot(self, rhs: Rhs) -> Self::Output;
}

macro_rules! impl_dot {
    ($lhs:ty, $rhs:ty => $out:ty) => {
        impl Dot<$rhs> for $lhs {
            type Output = $out;

            #[inline]
            fn dot(self, rhs: $rhs) -> Self::Output {
                <$lhs>::dot(self, rhs)
            }
        }
    };
}

impl_dot!(glam::Vec2, glam::Vec2 => f32);
impl_dot!(glam::Vec3, glam::Vec3 => f32);
impl_dot!(glam::Vec4, glam::Vec4 => f32);

pub trait Cross<Rhs = Self> {
    type Output;
    fn cross(self, rhs: Rhs) -> Self::Output;
}

macro_rules! impl_cross {
    ($lhs:ty, $rhs:ty => $out:ty) => {
        impl Cross<$rhs> for $lhs {
            type Output = $out;

            #[inline]
            fn cross(self, rhs: $rhs) -> Self::Output {
                <$lhs>::cross(self, rhs)
            }
        }
    };
}

impl_cross!(glam::Vec3, glam::Vec3 => glam::Vec3);

pub trait Length {
    type Output;
    fn length_squared(self) -> Self::Output;
    fn length(self) -> Self::Output;
}

macro_rules! impl_length {
    ($lhs:ty => $out:ty) => {
        impl Length for $lhs {
            type Output = $out;

            #[inline]
            fn length(self) -> Self::Output {
                <$lhs>::length(self)
            }
            #[inline]
            fn length_squared(self) -> Self::Output {
                <$lhs>::length_squared(self)
            }
        }
    };
}

impl_length!(glam::Vec2 => f32);
impl_length!(glam::Vec3 => f32);
impl_length!(glam::Vec4 => f32);

pub trait Normalize {
    fn normalize(self) -> Self;
}
macro_rules! impl_normalize {
    ($lhs:ty) => {
        impl Normalize for $lhs {
            #[inline]
            fn normalize(self) -> Self {
                <$lhs>::normalize(self)
            }
        }
    };
}

impl_normalize!(glam::Vec2);
impl_normalize!(glam::Vec3);
impl_normalize!(glam::Vec4);

pub trait Lerp<S> {
    type Output;
    fn lerp(self, rhs: Self, s: S) -> Self;
}
macro_rules! impl_lerp {
    ($lhs:ty, $scalar:ty => $out:ty) => {
        impl Lerp<$scalar> for $lhs {
            type Output = $out;
            #[inline]
            fn lerp(self, rhs: Self, s: $scalar) -> Self::Output {
                <$lhs>::lerp(self, rhs, s)
            }
        }
    };
}

impl_lerp!(glam::Vec2, f32 => glam::Vec2);
impl_lerp!(glam::Vec3, f32 => glam::Vec3);
impl_lerp!(glam::Vec4, f32 => glam::Vec4);
