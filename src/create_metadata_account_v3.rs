//! CPI wrapper for Metaplex CreateMetadataAccountV3 instruction

use crate::{CreateMetadataAccountV3Args, MPL_TOKEN_METADATA_ID};
use pinocchio::{
    account_info::AccountInfo,
    cpi::invoke_signed,
    instruction::{AccountMeta, Instruction, Signer},
    ProgramResult,
};

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
            &[
                self.metadata,
                self.mint,
                self.mint_authority,
                self.payer,
                self.update_authority,
                self.system_program,
            ],
            signers,
        )
    }
}
