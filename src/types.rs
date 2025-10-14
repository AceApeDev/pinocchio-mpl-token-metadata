//! Types for Metaplex Token Metadata
//!
//! This module provides types for creating token metadata with manual Borsh-like serialization.

/// Arguments for CreateMetadataAccountV3 instruction
pub struct CreateMetadataAccountV3Args<'a> {
    pub name: &'a str,
    pub symbol: &'a str,
    pub uri: &'a str,
    pub seller_fee_basis_points: u16,
    pub is_mutable: bool,
}

impl<'a> CreateMetadataAccountV3Args<'a> {
    /// Create new args with basic parameters
    #[inline(always)]
    pub fn new(name: &'a str, symbol: &'a str, uri: &'a str) -> Self {
        Self {
            name,
            symbol,
            uri,
            seller_fee_basis_points: 0,
            is_mutable: true,
        }
    }

    /// Set seller fee basis points
    #[inline(always)]
    pub fn with_seller_fee(mut self, seller_fee_basis_points: u16) -> Self {
        self.seller_fee_basis_points = seller_fee_basis_points;
        self
    }

    /// Set mutability
    #[inline(always)]
    pub fn with_mutable(mut self, is_mutable: bool) -> Self {
        self.is_mutable = is_mutable;
        self
    }

    /// Serialize to instruction data into a static buffer
    /// Returns (buffer, length)
    /// Format: discriminator(1) + name(4+len) + symbol(4+len) + uri(4+len) +
    ///         seller_fee(2) + creators(opt) + collection(opt) + uses(opt) + is_mutable(1) + collection_details(opt)
    pub fn to_instruction_data(&self, buf: &mut [u8]) -> usize {
        let mut offset = 0;

        // Discriminator for CreateMetadataAccountV3
        buf[offset] = 33;
        offset += 1;

        // DataV2:
        // - name: String (u32 length + bytes)
        offset = Self::write_string(buf, offset, self.name);

        // - symbol: String
        offset = Self::write_string(buf, offset, self.symbol);

        // - uri: String
        offset = Self::write_string(buf, offset, self.uri);

        // - seller_fee_basis_points: u16
        buf[offset..offset + 2].copy_from_slice(&self.seller_fee_basis_points.to_le_bytes());
        offset += 2;

        // - creators: Option<Vec<Creator>> = None
        buf[offset] = 0;
        offset += 1;

        // - collection: Option<Collection> = None
        buf[offset] = 0;
        offset += 1;

        // - uses: Option<Uses> = None
        buf[offset] = 0;
        offset += 1;

        // - is_mutable: bool (as u8)
        buf[offset] = self.is_mutable as u8;
        offset += 1;

        // - collection_details: Option<CollectionDetails> = None
        buf[offset] = 0;
        offset += 1;

        offset
    }

    #[inline(always)]
    fn write_string(buf: &mut [u8], offset: usize, s: &str) -> usize {
        let bytes = s.as_bytes();
        buf[offset..offset + 4].copy_from_slice(&(bytes.len() as u32).to_le_bytes());
        buf[offset + 4..offset + 4 + bytes.len()].copy_from_slice(bytes);
        offset + 4 + bytes.len()
    }
}
