//! Vertical floating-point `sin_pi`
#![allow(unused)]

// FIXME 64-bit 1 elem vectors sin_pi

use crate::*;

crate trait SinPi {
    fn sin_pi(self) -> Self;
}

macro_rules! impl_vsin {
    ($vid:ident: $llvm_fn:ident) => {
        impl SinPi for $vid {
            #[inline]
            fn sin_pi(self) -> Self {
                unsafe { mem::transmute($llvm_fn(mem::transmute(self))) }
            }
        }
    };
}

macro_rules! impl_def {
    ($vid:ident, $PI:path) => {
        impl SinPi for $vid {
            #[inline]
            fn sin_pi(self) -> Self {
                (self * Self::splat($PI)).sin()
            }
        }
    };
}

cfg_if! {
    if #[cfg(all(target_arch = "x86_64", feature = "sleef-sys"))] {
        use ::sleef_sys::*;
        impl_def!(f32x2, core::f32::consts::PI);
        impl_def!(f32x16, core::f32::consts::PI);
        impl_def!(f64x8, core::f64::consts::PI);
        cfg_if! {
            if #[cfg(target_feature = "avx2")] {
                impl_vsin!(f32x4: Sleef_sinpif4_u05avx2128);
                impl_vsin!(f32x8: Sleef_sinpif8_u05avx2);
                impl_vsin!(f64x2: Sleef_sinpid2_u05avx2128);
                impl_vsin!(f64x4: Sleef_sinpid4_u05avx2);
            } else if #[cfg(target_feature = "avx")] {
                impl_vsin!(f32x4: Sleef_sinpif4_u05sse4);
                impl_vsin!(f32x8: Sleef_sinpif8_u05avx);
                impl_vsin!(f64x2: Sleef_sinpid2_u05sse4);
                impl_vsin!(f64x4: Sleef_sinpid4_u05avx);
            } else if #[cfg(target_feature = "sse4.2")] {
                impl_vsin!(f32x4: Sleef_sinpif4_u05sse4);
                impl_def!(f32x8, core::f32::consts::PI);
                impl_vsin!(f64x2: Sleef_sinpid2_u05sse4);
                impl_def!(f64x4, core::f64::consts::PI);
            } else {
                impl_def!(f32x4, core::f32::consts::PI);
                impl_def!(f32x8, core::f32::consts::PI);
                impl_def!(f64x2, core::f64::consts::PI);
                impl_def!(f64x4, core::f64::consts::PI);
            }
        }
    } else {
        impl_def!(f32x2, core::f32::consts::PI);
        impl_def!(f32x4, core::f32::consts::PI);
        impl_def!(f32x8, core::f32::consts::PI);
        impl_def!(f32x16, core::f32::consts::PI);

        impl_def!(f64x2, core::f64::consts::PI);
        impl_def!(f64x4, core::f64::consts::PI);
        impl_def!(f64x8, core::f64::consts::PI);
    }
}
