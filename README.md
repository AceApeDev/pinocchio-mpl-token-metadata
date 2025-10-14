# Pinocchio MPL Token Metadata

A minimal, zero-copy SDK for Metaplex Token Metadata program designed for Solana programs using Pinocchio.

## Features

- 🚀 **Zero-copy**: No dynamic allocations, works with static buffers
- 📦 **Minimal**: Only what you need - no unnecessary dependencies
- 🎯 **Simple API**: Easy-to-use builder pattern
- ⚡ **Efficient**: Manual serialization optimized for BPF

## Usage

```rust
use pinocchio_mpl_token_metadata::{CreateMetadataAccountV3, CreateMetadataAccountV3Args};

CreateMetadataAccountV3 {
    metadata: metadata_info,
    mint: mint_info,
    mint_authority: mint_authority_info,
    payer: payer_info,
    update_authority: update_authority_info,
    system_program,
    args: CreateMetadataAccountV3Args {
        name: "Token Name",
        symbol: "SYMBOL",
        uri: "https://example.com/metadata.json",
        seller_fee_basis_points: 0,
        is_mutable: true,
    },
}
.invoke_signed(&[signer])?;
```

## Supported Instructions

Currently supports:

- ✅ `CreateMetadataAccountV3`

Need more instructions? PRs are welcome!

## References

- [mpl-token-metadata](https://github.com/metaplex-foundation/mpl-token-metadata/tree/main/clients/rust/)
- [pinocchio](https://github.com/febo/pinocchio)
