# Pinocchio MPL Token Metadata

A minimal, zero-copy SDK for Metaplex Token Metadata program designed for Solana programs using Pinocchio.

## Features

- 🚀 **Zero-copy**: No dynamic allocations, works with static buffers
- 📦 **Minimal**: Only what you need - no unnecessary dependencies
- 🎯 **Simple API**: Easy-to-use builder pattern
- ⚡ **Efficient**: Manual serialization optimized for BPF

## Usage

```rust
use pinocchio_mpl_token_metadata::{CreateMetadataAccountV3, MPL_TOKEN_METADATA_ID};

CreateMetadataAccountV3::new(
    metadata_info,
    mint_info,
    mint_authority_info,
    payer_info,
    update_authority_info,
    system_program,
    "Token Name",
    "SYMBOL",
    "https://example.com/metadata.json",
)
.invoke_signed(&[signer])?;
```

## Supported Instructions

Currently supports:
- ✅ `CreateMetadataAccountV3`

Need more instructions? PRs are welcome!

## References

- [mpl-token-metadata](https://github.com/metaplex-foundation/mpl-token-metadata/tree/main/clients/rust/)
- [pinocchio](https://github.com/febo/pinocchio)