// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`@ 1.82.6.
// ignore_for_file: non_constant_identifier_names, unused_element, duplicate_ignore, directives_ordering, curly_braces_in_flow_control_structures, unnecessary_lambdas, slash_for_doc_comments, prefer_const_literals_to_create_immutables, implicit_dynamic_list_literal, duplicate_import, unused_import, unnecessary_import, prefer_single_quotes, prefer_const_constructors, use_super_parameters, always_use_package_imports, annotate_overrides, invalid_use_of_protected_member, constant_identifier_names, invalid_use_of_internal_member, prefer_is_empty, unnecessary_const

import 'dart:convert';
import 'dart:async';
import 'package:meta/meta.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:uuid/uuid.dart';

abstract class LwkBridge {
  Future<String> newDescriptorStaticMethodApi(
      {required Network network, required String mnemonic, dynamic hint});

  FlutterRustBridgeTaskConstMeta get kNewDescriptorStaticMethodApiConstMeta;

  Future<String> blindingKeyStaticMethodApi(
      {required String walletId, dynamic hint});

  FlutterRustBridgeTaskConstMeta get kBlindingKeyStaticMethodApiConstMeta;

  Future<String> newWalletStaticMethodApi(
      {required Network network,
      required String dbPath,
      required String descriptor,
      dynamic hint});

  FlutterRustBridgeTaskConstMeta get kNewWalletStaticMethodApiConstMeta;

  Future<void> syncStaticMethodApi(
      {required String walletId, required String electrumUrl, dynamic hint});

  FlutterRustBridgeTaskConstMeta get kSyncStaticMethodApiConstMeta;

  Future<String> walletDescriptorStaticMethodApi(
      {required String walletId, dynamic hint});

  FlutterRustBridgeTaskConstMeta get kWalletDescriptorStaticMethodApiConstMeta;

  Future<Address> addressLastUnusedStaticMethodApi(
      {required String walletId, dynamic hint});

  FlutterRustBridgeTaskConstMeta get kAddressLastUnusedStaticMethodApiConstMeta;

  Future<Address> addressStaticMethodApi(
      {required String walletId, required int index, dynamic hint});

  FlutterRustBridgeTaskConstMeta get kAddressStaticMethodApiConstMeta;

  Future<void> validateAddressStaticMethodApi(
      {required String addressString, dynamic hint});

  FlutterRustBridgeTaskConstMeta get kValidateAddressStaticMethodApiConstMeta;

  Future<Address> addressFromScriptStaticMethodApi(
      {required Network network,
      required String script,
      required String blindingKey,
      dynamic hint});

  FlutterRustBridgeTaskConstMeta get kAddressFromScriptStaticMethodApiConstMeta;

  Future<List<Balance>> balanceStaticMethodApi(
      {required String walletId, dynamic hint});

  FlutterRustBridgeTaskConstMeta get kBalanceStaticMethodApiConstMeta;

  Future<List<Tx>> txsStaticMethodApi({required String walletId, dynamic hint});

  FlutterRustBridgeTaskConstMeta get kTxsStaticMethodApiConstMeta;

  Future<String> buildLbtcTxStaticMethodApi(
      {required String walletId,
      required int sats,
      required String outAddress,
      required double absFee,
      dynamic hint});

  FlutterRustBridgeTaskConstMeta get kBuildLbtcTxStaticMethodApiConstMeta;

  Future<String> buildAssetTxStaticMethodApi(
      {required String walletId,
      required int sats,
      required String outAddress,
      required double absFee,
      required String assetId,
      dynamic hint});

  FlutterRustBridgeTaskConstMeta get kBuildAssetTxStaticMethodApiConstMeta;

  Future<PsetAmounts> decodeTxStaticMethodApi(
      {required String walletId, required String pset, dynamic hint});

  FlutterRustBridgeTaskConstMeta get kDecodeTxStaticMethodApiConstMeta;

  Future<Uint8List> signTxStaticMethodApi(
      {required String walletId,
      required Network network,
      required String pset,
      required String mnemonic,
      dynamic hint});

  FlutterRustBridgeTaskConstMeta get kSignTxStaticMethodApiConstMeta;

  Future<String> broadcastTxStaticMethodApi(
      {required String electrumUrl, required Uint8List txBytes, dynamic hint});

  FlutterRustBridgeTaskConstMeta get kBroadcastTxStaticMethodApiConstMeta;
}

class Address {
  final String standard;
  final String confidential;
  final int index;

  const Address({
    required this.standard,
    required this.confidential,
    required this.index,
  });
}

class Balance {
  final String assetId;
  final int value;

  const Balance({
    required this.assetId,
    required this.value,
  });
}

/// Possible errors emitted
class LwkError implements FrbException {
  final String msg;

  const LwkError({
    required this.msg,
  });
}

enum Network {
  Mainnet,
  Testnet,
}

class OutPoint {
  final String txid;
  final int vout;

  const OutPoint({
    required this.txid,
    required this.vout,
  });
}

class PsetAmounts {
  final int fee;
  final List<Balance> balances;

  const PsetAmounts({
    required this.fee,
    required this.balances,
  });
}

class Tx {
  final int timestamp;
  final String kind;
  final List<Balance> balances;
  final String txid;
  final List<TxOut> outputs;
  final List<TxOut> inputs;
  final int fee;

  const Tx({
    required this.timestamp,
    required this.kind,
    required this.balances,
    required this.txid,
    required this.outputs,
    required this.inputs,
    required this.fee,
  });
}

class TxOut {
  final String scriptPubkey;
  final OutPoint outpoint;
  final int? height;
  final TxOutSecrets unblinded;

  const TxOut({
    required this.scriptPubkey,
    required this.outpoint,
    this.height,
    required this.unblinded,
  });
}

class TxOutSecrets {
  final int value;
  final String valueBf;
  final String asset;
  final String assetBf;

  const TxOutSecrets({
    required this.value,
    required this.valueBf,
    required this.asset,
    required this.assetBf,
  });
}
