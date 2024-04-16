#![allow(
    non_camel_case_types,
    unused,
    clippy::redundant_closure,
    clippy::useless_conversion,
    clippy::unit_arg,
    clippy::double_parens,
    non_snake_case,
    clippy::too_many_arguments
)]
// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`@ 1.82.6.

use crate::api::*;
use core::panic::UnwindSafe;
use flutter_rust_bridge::rust2dart::IntoIntoDart;
use flutter_rust_bridge::*;
use std::ffi::c_void;
use std::sync::Arc;

// Section: imports

use crate::error::LwkError;
use crate::network::Network;
use crate::types::Address;
use crate::types::Balance;
use crate::types::OutPoint;
use crate::types::PsetAmounts;
use crate::types::Tx;
use crate::types::TxOut;
use crate::types::TxOutSecrets;

// Section: wire functions

fn wire_new_descriptor__static_method__Api_impl(
    port_: MessagePort,
    network: impl Wire2Api<Network> + UnwindSafe,
    mnemonic: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, String, _>(
        WrapInfo {
            debug_name: "new_descriptor__static_method__Api",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_network = network.wire2api();
            let api_mnemonic = mnemonic.wire2api();
            move |task_callback| Api::new_descriptor(api_network, api_mnemonic)
        },
    )
}
fn wire_blinding_key__static_method__Api_impl(
    port_: MessagePort,
    wallet_id: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, String, _>(
        WrapInfo {
            debug_name: "blinding_key__static_method__Api",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_wallet_id = wallet_id.wire2api();
            move |task_callback| Api::blinding_key(api_wallet_id)
        },
    )
}
fn wire_new_wallet__static_method__Api_impl(
    port_: MessagePort,
    network: impl Wire2Api<Network> + UnwindSafe,
    db_path: impl Wire2Api<String> + UnwindSafe,
    descriptor: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, String, _>(
        WrapInfo {
            debug_name: "new_wallet__static_method__Api",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_network = network.wire2api();
            let api_db_path = db_path.wire2api();
            let api_descriptor = descriptor.wire2api();
            move |task_callback| Api::new_wallet(api_network, api_db_path, api_descriptor)
        },
    )
}
fn wire_sync__static_method__Api_impl(
    port_: MessagePort,
    wallet_id: impl Wire2Api<String> + UnwindSafe,
    electrum_url: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, (), _>(
        WrapInfo {
            debug_name: "sync__static_method__Api",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_wallet_id = wallet_id.wire2api();
            let api_electrum_url = electrum_url.wire2api();
            move |task_callback| Api::sync(api_wallet_id, api_electrum_url)
        },
    )
}
fn wire_wallet_descriptor__static_method__Api_impl(
    port_: MessagePort,
    wallet_id: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, String, _>(
        WrapInfo {
            debug_name: "wallet_descriptor__static_method__Api",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_wallet_id = wallet_id.wire2api();
            move |task_callback| Api::wallet_descriptor(api_wallet_id)
        },
    )
}
fn wire_address_last_unused__static_method__Api_impl(
    port_: MessagePort,
    wallet_id: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, Address, _>(
        WrapInfo {
            debug_name: "address_last_unused__static_method__Api",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_wallet_id = wallet_id.wire2api();
            move |task_callback| Api::address_last_unused(api_wallet_id)
        },
    )
}
fn wire_address__static_method__Api_impl(
    port_: MessagePort,
    wallet_id: impl Wire2Api<String> + UnwindSafe,
    index: impl Wire2Api<u32> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, Address, _>(
        WrapInfo {
            debug_name: "address__static_method__Api",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_wallet_id = wallet_id.wire2api();
            let api_index = index.wire2api();
            move |task_callback| Api::address(api_wallet_id, api_index)
        },
    )
}
fn wire_validate_address__static_method__Api_impl(
    port_: MessagePort,
    address_string: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, (), _>(
        WrapInfo {
            debug_name: "validate_address__static_method__Api",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_address_string = address_string.wire2api();
            move |task_callback| Api::validate_address(api_address_string)
        },
    )
}
fn wire_address_from_script__static_method__Api_impl(
    port_: MessagePort,
    network: impl Wire2Api<Network> + UnwindSafe,
    script: impl Wire2Api<String> + UnwindSafe,
    blinding_key: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, Address, _>(
        WrapInfo {
            debug_name: "address_from_script__static_method__Api",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_network = network.wire2api();
            let api_script = script.wire2api();
            let api_blinding_key = blinding_key.wire2api();
            move |task_callback| Api::address_from_script(api_network, api_script, api_blinding_key)
        },
    )
}
fn wire_balance__static_method__Api_impl(
    port_: MessagePort,
    wallet_id: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, Vec<Balance>, _>(
        WrapInfo {
            debug_name: "balance__static_method__Api",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_wallet_id = wallet_id.wire2api();
            move |task_callback| Api::balance(api_wallet_id)
        },
    )
}
fn wire_txs__static_method__Api_impl(
    port_: MessagePort,
    wallet_id: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, Vec<Tx>, _>(
        WrapInfo {
            debug_name: "txs__static_method__Api",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_wallet_id = wallet_id.wire2api();
            move |task_callback| Api::txs(api_wallet_id)
        },
    )
}
fn wire_build_lbtc_tx__static_method__Api_impl(
    port_: MessagePort,
    wallet_id: impl Wire2Api<String> + UnwindSafe,
    sats: impl Wire2Api<u64> + UnwindSafe,
    out_address: impl Wire2Api<String> + UnwindSafe,
    abs_fee: impl Wire2Api<f32> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, String, _>(
        WrapInfo {
            debug_name: "build_lbtc_tx__static_method__Api",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_wallet_id = wallet_id.wire2api();
            let api_sats = sats.wire2api();
            let api_out_address = out_address.wire2api();
            let api_abs_fee = abs_fee.wire2api();
            move |task_callback| {
                Api::build_lbtc_tx(api_wallet_id, api_sats, api_out_address, api_abs_fee)
            }
        },
    )
}
fn wire_build_asset_tx__static_method__Api_impl(
    port_: MessagePort,
    wallet_id: impl Wire2Api<String> + UnwindSafe,
    sats: impl Wire2Api<u64> + UnwindSafe,
    out_address: impl Wire2Api<String> + UnwindSafe,
    abs_fee: impl Wire2Api<f32> + UnwindSafe,
    asset_id: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, String, _>(
        WrapInfo {
            debug_name: "build_asset_tx__static_method__Api",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_wallet_id = wallet_id.wire2api();
            let api_sats = sats.wire2api();
            let api_out_address = out_address.wire2api();
            let api_abs_fee = abs_fee.wire2api();
            let api_asset_id = asset_id.wire2api();
            move |task_callback| {
                Api::build_asset_tx(
                    api_wallet_id,
                    api_sats,
                    api_out_address,
                    api_abs_fee,
                    api_asset_id,
                )
            }
        },
    )
}
fn wire_decode_tx__static_method__Api_impl(
    port_: MessagePort,
    wallet_id: impl Wire2Api<String> + UnwindSafe,
    pset: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, PsetAmounts, _>(
        WrapInfo {
            debug_name: "decode_tx__static_method__Api",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_wallet_id = wallet_id.wire2api();
            let api_pset = pset.wire2api();
            move |task_callback| Api::decode_tx(api_wallet_id, api_pset)
        },
    )
}
fn wire_sign_tx__static_method__Api_impl(
    port_: MessagePort,
    wallet_id: impl Wire2Api<String> + UnwindSafe,
    network: impl Wire2Api<Network> + UnwindSafe,
    pset: impl Wire2Api<String> + UnwindSafe,
    mnemonic: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, Vec<u8>, _>(
        WrapInfo {
            debug_name: "sign_tx__static_method__Api",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_wallet_id = wallet_id.wire2api();
            let api_network = network.wire2api();
            let api_pset = pset.wire2api();
            let api_mnemonic = mnemonic.wire2api();
            move |task_callback| Api::sign_tx(api_wallet_id, api_network, api_pset, api_mnemonic)
        },
    )
}
fn wire_broadcast_tx__static_method__Api_impl(
    port_: MessagePort,
    electrum_url: impl Wire2Api<String> + UnwindSafe,
    tx_bytes: impl Wire2Api<Vec<u8>> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, String, _>(
        WrapInfo {
            debug_name: "broadcast_tx__static_method__Api",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_electrum_url = electrum_url.wire2api();
            let api_tx_bytes = tx_bytes.wire2api();
            move |task_callback| Api::broadcast_tx(api_electrum_url, api_tx_bytes)
        },
    )
}
// Section: wrapper structs

// Section: static checks

// Section: allocate functions

// Section: related functions

// Section: impl Wire2Api

pub trait Wire2Api<T> {
    fn wire2api(self) -> T;
}

impl<T, S> Wire2Api<Option<T>> for *mut S
where
    *mut S: Wire2Api<T>,
{
    fn wire2api(self) -> Option<T> {
        (!self.is_null()).then(|| self.wire2api())
    }
}

impl Wire2Api<f32> for f32 {
    fn wire2api(self) -> f32 {
        self
    }
}
impl Wire2Api<i32> for i32 {
    fn wire2api(self) -> i32 {
        self
    }
}
impl Wire2Api<Network> for i32 {
    fn wire2api(self) -> Network {
        match self {
            0 => Network::Mainnet,
            1 => Network::Testnet,
            _ => unreachable!("Invalid variant for Network: {}", self),
        }
    }
}
impl Wire2Api<u32> for u32 {
    fn wire2api(self) -> u32 {
        self
    }
}
impl Wire2Api<u64> for u64 {
    fn wire2api(self) -> u64 {
        self
    }
}
impl Wire2Api<u8> for u8 {
    fn wire2api(self) -> u8 {
        self
    }
}

// Section: impl IntoDart

impl support::IntoDart for Address {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.standard.into_into_dart().into_dart(),
            self.confidential.into_into_dart().into_dart(),
            self.index.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for Address {}
impl rust2dart::IntoIntoDart<Address> for Address {
    fn into_into_dart(self) -> Self {
        self
    }
}

impl support::IntoDart for Balance {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.asset_id.into_into_dart().into_dart(),
            self.value.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for Balance {}
impl rust2dart::IntoIntoDart<Balance> for Balance {
    fn into_into_dart(self) -> Self {
        self
    }
}

impl support::IntoDart for LwkError {
    fn into_dart(self) -> support::DartAbi {
        vec![self.msg.into_into_dart().into_dart()].into_dart()
    }
}
impl support::IntoDartExceptPrimitive for LwkError {}
impl rust2dart::IntoIntoDart<LwkError> for LwkError {
    fn into_into_dart(self) -> Self {
        self
    }
}

impl support::IntoDart for OutPoint {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.txid.into_into_dart().into_dart(),
            self.vout.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for OutPoint {}
impl rust2dart::IntoIntoDart<OutPoint> for OutPoint {
    fn into_into_dart(self) -> Self {
        self
    }
}

impl support::IntoDart for PsetAmounts {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.fee.into_into_dart().into_dart(),
            self.balances.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for PsetAmounts {}
impl rust2dart::IntoIntoDart<PsetAmounts> for PsetAmounts {
    fn into_into_dart(self) -> Self {
        self
    }
}

impl support::IntoDart for Tx {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.timestamp.into_into_dart().into_dart(),
            self.kind.into_into_dart().into_dart(),
            self.balances.into_into_dart().into_dart(),
            self.txid.into_into_dart().into_dart(),
            self.outputs.into_into_dart().into_dart(),
            self.inputs.into_into_dart().into_dart(),
            self.fee.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for Tx {}
impl rust2dart::IntoIntoDart<Tx> for Tx {
    fn into_into_dart(self) -> Self {
        self
    }
}

impl support::IntoDart for TxOut {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.script_pubkey.into_into_dart().into_dart(),
            self.outpoint.into_into_dart().into_dart(),
            self.height.into_dart(),
            self.unblinded.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for TxOut {}
impl rust2dart::IntoIntoDart<TxOut> for TxOut {
    fn into_into_dart(self) -> Self {
        self
    }
}

impl support::IntoDart for TxOutSecrets {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.value.into_into_dart().into_dart(),
            self.value_bf.into_into_dart().into_dart(),
            self.asset.into_into_dart().into_dart(),
            self.asset_bf.into_into_dart().into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for TxOutSecrets {}
impl rust2dart::IntoIntoDart<TxOutSecrets> for TxOutSecrets {
    fn into_into_dart(self) -> Self {
        self
    }
}

// Section: executor

support::lazy_static! {
    pub static ref FLUTTER_RUST_BRIDGE_HANDLER: support::DefaultHandler = Default::default();
}

#[cfg(not(target_family = "wasm"))]
#[path = "bridge_generated.io.rs"]
mod io;
#[cfg(not(target_family = "wasm"))]
pub use self::io::*;
