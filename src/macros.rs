macro_rules! define_endian_types {
    ($($en_ty:ident $base_ty:ident $from_en:ident $to_en:ident $fn_to_oppo:ident $oppo_ty:ident),*$(,)?) => {

        $( /* Repeats for each $en_ty */
        /* Type definition */
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        #[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
        #[repr(transparent)]
        pub struct $en_ty($base_ty);
        /* const method conversions */
        impl $en_ty {
            #[doc = concat!("Creates a new [`", stringify!($en_ty), "`] with the provided [`", stringify!($base_ty), "`]'s value.")]
            pub const fn new(n: $base_ty) -> $en_ty {
                $en_ty(<$base_ty>::$to_en(n))
            }
            #[doc = concat!("Gets the value as a native [`", stringify!($base_ty), "`].")]
            pub const fn to_ne(self) -> $base_ty {
                <$base_ty>::$from_en(self.0)
            }
            #[doc = concat!("Swaps byte-order to create a [`", stringify!($oppo_ty), "`].")]
            pub const fn $fn_to_oppo(self) -> $oppo_ty {
                $oppo_ty::new(self.to_ne())
            }
            #[doc = concat!("Gets the internal representation as a native [`", stringify!($base_ty), "`], ingoring proper endian conversions.")]
            pub const fn bits(self) -> $base_ty {
                self.0
            }
            #[doc = concat!("Creates a [`", stringify!($en_ty),"`] with the provided internal representation, ignoring proper endian conversions.")]
            pub const fn from_bits(bits: $base_ty) -> $en_ty {
                Self(bits)
            }
        }
        /* Type from native */
        impl From<$en_ty> for $base_ty {
            fn from(nen: $en_ty) -> $base_ty {
                nen.to_ne()
            }
        }
        impl From<&$en_ty> for $base_ty {
            fn from(nen: &$en_ty) -> $base_ty {
                nen.to_ne()
            }
        }
        /* Type into native */
        impl From<$base_ty> for $en_ty {
            fn from(n: $base_ty) -> $en_ty {
                $en_ty(<$base_ty>::$to_en(n))
            }
        }
        impl From<&$base_ty> for $en_ty {
            fn from(n: &$base_ty) -> $en_ty {
                n.into()
            }
        }
        /* Bitwise AND */
        impl core::ops::BitAnd for $en_ty {
            type Output = Self;
            fn bitand(self, rhs: Self) -> Self {
                Self::from_bits(self.bits() & rhs.bits())
            }
        }
        impl core::ops::BitAndAssign for $en_ty {
            fn bitand_assign(&mut self, rhs: Self) {
                *self = Self::from_bits(self.bits() & rhs.bits())
            }
        }
        impl core::ops::BitAnd<&Self> for $en_ty {
            type Output = Self;
            fn bitand(self, rhs: &Self) -> Self {
                Self::from_bits(self.bits() & rhs.bits())
            }
        }
        impl core::ops::BitAndAssign<&Self> for $en_ty {
            fn bitand_assign(&mut self, rhs: &Self) {
                *self = Self::from_bits(self.bits() & rhs.bits())
            }
        }
        /* Bitwise OR */
        impl core::ops::BitOr for $en_ty {
            type Output = Self;
            fn bitor(self, rhs: Self) -> Self {
                Self::from_bits(self.bits() | rhs.bits())
            }
        }
        impl core::ops::BitOrAssign for $en_ty {
            fn bitor_assign(&mut self, rhs: Self) {
                *self = Self::from_bits(self.bits() | rhs.bits())
            }
        }
        impl core::ops::BitOr<&Self> for $en_ty {
            type Output = Self;
            fn bitor(self, rhs: &Self) -> Self {
                Self::from_bits(self.bits() | rhs.bits())
            }
        }
        impl core::ops::BitOrAssign<&Self> for $en_ty {
            fn bitor_assign(&mut self, rhs: &Self) {
                *self = Self::from_bits(self.bits() | rhs.bits())
            }
        }
        /* Bitwise XOR */
        impl core::ops::BitXor for $en_ty {
            type Output = Self;
            fn bitxor(self, rhs: Self) -> Self {
                Self::from_bits(self.bits() ^ rhs.bits())
            }
        }
        impl core::ops::BitXorAssign for $en_ty {
            fn bitxor_assign(&mut self, rhs: Self) {
                *self = Self::from_bits(self.bits() ^ rhs.bits())
            }
        }
        impl core::ops::BitXor<&Self> for $en_ty {
            type Output = Self;
            fn bitxor(self, rhs: &Self) -> Self {
                Self::from_bits(self.bits() ^ rhs.bits())
            }
        }
        impl core::ops::BitXorAssign<&Self> for $en_ty {
            fn bitxor_assign(&mut self, rhs: &Self) {
                *self = Self::from_bits(self.bits() ^ rhs.bits())
            }
        }
        /* Bitwise NOT */
        impl core::ops::Not for $en_ty {
            type Output = Self;
            fn not(self) -> Self {
                Self::from_bits(!self.bits())
            }
        }
        /* Bit access */
        impl $en_ty {
            pub const fn count_ones(self) -> u32 {
                self.bits().count_ones()
            }
            pub const fn count_zeros(self) -> u32 {
                self.bits().count_zeros()
            }
        }
        /* Formats */
        impl core::fmt::Display for $en_ty {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                core::fmt::Display::fmt(&self.to_ne(), f)
            }
        }
        impl core::fmt::Debug for $en_ty {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct(stringify!($en_ty))
                    .field("bits", &self.bits())
                    .field("to_ne", &self.to_ne())
                    .finish()
            }
        }
        )* /* Repeats for each $en_ty */
    };
}
