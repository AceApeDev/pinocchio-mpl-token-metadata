//! CPI wrapper for Metaplex CreateMetadataAccountV3 instruction

use pinocchio::{
    ProgramResult,
    account_info::AccountInfo,
    instruction::{AccountMeta, Instruction, Signer},
    cpi::invoke_signed,
};
use crate::{MPL_TOKEN_METADATA_ID, CreateMetadataAccountV3Args};

/// Builder for CreateMetadataAccountV3 CPI
pub struct CreateMetadataAccountV3<'a> {
    pub metadata: &'a AccountInfo,
    pub mint: &'a AccountInfo,
    pub mint_authority: &'a AccountInfo,
    pub payer: &'a AccountInfo,
    pub update_authority: &'a AccountInfo,
    pub system_program: &'a AccountInfo,

    pub args: CreateMetadataAccountV3Args<'a>,
}

impl<'a> CreateMetadataAccountV3<'a> {
    /// Create a new instance with simple parameters
    #[inline(always)]
    pub fn new(
        metadata: &'a AccountInfo,
        mint: &'a AccountInfo,
        mint_authority: &'a AccountInfo,
        payer: &'a AccountInfo,
        update_authority: &'a AccountInfo,
        system_program: &'a AccountInfo,
        name: &'a str,
        symbol: &'a str,
        uri: &'a str,
    ) -> Self {
        Self {
            metadata,
            mint,
            mint_authority,
            payer,
            update_authority,
            system_program,
            args: CreateMetadataAccountV3Args::new(name, symbol, uri),
        }
    }

    /// Set custom args
    #[inline(always)]
    pub fn with_args(mut self, args: CreateMetadataAccountV3Args<'a>) -> Self {
        self.args = args;
        self
    }

    /// Invoke the CPI without signers
    #[inline(always)]
    pub fn invoke(&self) -> ProgramResult {
        self.invoke_signed(&[])
    }

    /// Invoke the CPI with PDA signers
    pub fn invoke_signed(&self, signers: &[Signer]) -> ProgramResult {
        // Serialize instruction data into static buffer
        let mut data_buf = [0u8; 512];
        let data_len = self.args.to_instruction_data(&mut data_buf);

        // Build account metas (statically)
        let accounts: [AccountMeta; 6] = [
            AccountMeta::writable(self.metadata.key()),
            AccountMeta::readonly(self.mint.key()),
            AccountMeta::readonly_signer(self.mint_authority.key()),
            AccountMeta::writable_signer(self.payer.key()),
            AccountMeta::readonly(self.update_authority.key()),
            AccountMeta::readonly(self.system_program.key()),
        ];

        // Build instruction
        let instruction = Instruction {
            program_id: &MPL_TOKEN_METADATA_ID,
            accounts: &accounts,
            data: &data_buf[..data_len],
        };

        // Invoke CPI
        invoke_signed(
            &instruction,
            &[self.metadata, self.mint, self.mint_authority, self.payer, self.update_authority, self.system_program],
            signers,
        )
    }
}
