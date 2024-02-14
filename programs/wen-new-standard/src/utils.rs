use anchor_lang::{
    prelude::Result,
    solana_program::{
        account_info::AccountInfo,
        program::invoke,
        pubkey::Pubkey,
        rent::Rent,
        system_instruction::transfer,
        sysvar::{self, instructions::load_current_index_checked, Sysvar},
    },
    Lamports,
};
use anchor_spl::token_interface::{
    spl_token_2022::{
        extension::{BaseStateWithExtensions, Extension, StateWithExtensions},
        solana_zk_token_sdk::zk_token_proof_instruction::Pod,
        state::Mint,
    },
    spl_token_metadata_interface::state::TokenMetadata,
};
use spl_tlv_account_resolution::account::ExtraAccountMeta;

use crate::{APPROVE_ACCOUNT_SEED, META_LIST_ACCOUNT_SEED};

pub fn update_account_lamports_to_minimum_balance<'info>(
    account: AccountInfo<'info>,
    payer: AccountInfo<'info>,
    system_program: AccountInfo<'info>,
) -> Result<()> {
    let extra_lamports = Rent::get()?.minimum_balance(account.data_len()) - account.get_lamports();
    if extra_lamports > 0 {
        invoke(
            &transfer(payer.key, account.key, extra_lamports),
            &[payer, account, system_program],
        )?;
    }
    Ok(())
}

pub fn get_mint_metadata(account: &mut AccountInfo) -> Result<TokenMetadata> {
    let mint_data = account.data.borrow();
    let mint_with_extension = StateWithExtensions::<Mint>::unpack(&mint_data)?;
    let extension_data = mint_with_extension.get_variable_len_extension::<TokenMetadata>()?;
    Ok(extension_data)
}

pub fn get_extension_data<T: Extension + Pod>(account: &mut AccountInfo) -> Result<T> {
    let mint_data = account.data.borrow();
    let mint_with_extension = StateWithExtensions::<Mint>::unpack(&mint_data)?;
    let extension_data = *mint_with_extension.get_extension::<T>()?;
    Ok(extension_data)
}

pub fn get_extra_meta_list_account_pda(mint: Pubkey) -> Pubkey {
    Pubkey::find_program_address(&[META_LIST_ACCOUNT_SEED, mint.as_ref()], &crate::id()).0
}

pub fn get_approve_account_pda(mint: Pubkey) -> Pubkey {
    Pubkey::find_program_address(&[APPROVE_ACCOUNT_SEED, mint.as_ref()], &crate::id()).0
}

/// Returns true if the transfer is a cpi call
/// currently executing `Transaction`
pub fn is_cpi(instruction_sysvar_account_info: &AccountInfo) -> Result<bool> {
    let current_index = load_current_index_checked(instruction_sysvar_account_info)?;
    if current_index == 0 {
        return Ok(false);
    }
    Ok(true)
}

pub fn get_meta_list(approve_account: Option<Pubkey>) -> Vec<ExtraAccountMeta> {
    let mut meta_list = vec![
        // instructions program
        ExtraAccountMeta {
            discriminator: 0,
            address_config: sysvar::instructions::id().to_bytes(),
            is_signer: false.into(),
            is_writable: false.into(),
        },
    ];
    if let Some(approve_account) = approve_account {
        meta_list.push(ExtraAccountMeta {
            discriminator: 0,
            address_config: approve_account.to_bytes(),
            is_signer: false.into(),
            is_writable: true.into(),
        });
    }
    meta_list
}

pub fn get_meta_list_size(approve_account: Option<Pubkey>) -> usize {
    get_meta_list(approve_account).len()
}
