// SPDX-License-Identifier: CC0-1.0

//! Proof-of-work related integer types.

use core::fmt;

/// Encoding of 256-bit target as 32-bit float.
///
/// This is used to encode a target into the block header. Satoshi made this part of consensus code
/// in the original version of Bitcoin, likely copying an idea from OpenSSL.
///
/// OpenSSL's bignum (BN) type has an encoding, which is even called "compact" as in bitcoin, which
/// is exactly this format.
///
/// # Note on order/equality
///
/// Usage of the ordering and equality traits for this type may be surprising. Converting between
/// `CompactTarget` and `Target` is lossy *in both directions* (there are multiple `CompactTarget`
/// values that map to the same `Target` value). Ordering and equality for this type are defined in
/// terms of the underlying `u32`.
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CompactTarget(u32);

impl CompactTarget {
    /// Constructs a new [`CompactTarget`] from a consensus encoded `u32`.
    #[inline]
    pub fn from_consensus(bits: u32) -> Self { Self(bits) }

    /// Returns the consensus encoded `u32` representation of this [`CompactTarget`].
    #[inline]
    pub fn to_consensus(self) -> u32 { self.0 }
}

impl fmt::Display for CompactTarget {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { fmt::LowerHex::fmt(self, f) }
}

impl fmt::Debug for CompactTarget {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CompactTarget({:08x})", self.to_consensus())
    }
}

impl fmt::LowerHex for CompactTarget {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if f.alternate() {
            write!(f, "{:#08x}", self.to_consensus())
        } else {
            write!(f, "{:08x}", self.to_consensus())
        }
    }
}
#[cfg(feature = "alloc")]
internals::impl_to_hex_from_lower_hex!(CompactTarget, |_: &CompactTarget| 8);

impl fmt::UpperHex for CompactTarget {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if f.alternate() {
            write!(f, "{:#08X}", self.to_consensus())
        } else {
            write!(f, "{:08X}", self.to_consensus())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compact_target_ordering() {
        let lower = CompactTarget::from_consensus(0x1d00_fffe);
        let lower_copy = CompactTarget::from_consensus(0x1d00_fffe);
        let higher = CompactTarget::from_consensus(0x1d00_ffff);

        assert!(lower < higher);
        assert!(lower == lower_copy);
    }

    #[test]
    fn compact_target_formatting() {
        let compact_target = CompactTarget::from_consensus(0x1d00_ffff);
        assert_eq!(format!("{}", compact_target), "1d00ffff");
        assert_eq!(format!("{:x}", compact_target), "1d00ffff");
        assert_eq!(format!("{:X}", compact_target), "1D00FFFF");
        assert_eq!(format!("{:#x}", compact_target), "0x1d00ffff");
        assert_eq!(format!("{:#X}", compact_target), "0x1D00FFFF");
        assert_eq!(format!("{:?}", compact_target), "CompactTarget(1d00ffff)");
        assert_eq!(compact_target.to_consensus(), 0x1d00_ffff);
    }
}
