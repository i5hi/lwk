use std::str::FromStr;

use lwk_rpc_model::{
    request::{self, Direction},
    response,
};
use schemars::schema_for;
use serde_json::Value;

#[derive(Debug, thiserror::Error)]
#[error("The rpc method '{name}' does not exist")]
pub struct MethodNotExist {
    name: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(test, derive(enum_iterator::Sequence))]
pub enum Method {
    Schema,
    SignerGenerate,
    Version,
    WalletLoad,
    WalletUnload,
    WalletList,
    SignerLoadSoftware,
    SignerLoadJade,
    SignerLoadExternal,
    SignerDetails,
    SignerUnload,
    SignerList,
    WalletAddress,
    Balance,
    SendMany,
    SinglesigDescriptor,
    MultisigDescriptor,
    RegisterMultisig,
    Xpub,
    Sign,
    Broadcast,
    WalletDetails,
    WalletCombine,
    WalletPsetDetails,
    WalletUtxos,
    WalletTxs,
    WalletSetTxMemo,
    WalletSetAddrMemo,
    Issue,
    Reissue,
    Contract,
    AssetDetails,
    AssetList,
    AssetInsert,
    AssetRemove,
    AssetPublish,
    Scan,
    Stop,
    SignerJadeId,
}
impl Method {
    pub(crate) fn schema(&self, direction: request::Direction) -> Result<Value, serde_json::Error> {
        serde_json::to_value(match direction {
            Direction::Request => match self {
                Method::Schema => schema_for!(request::Schema),
                Method::SignerGenerate => schema_for!(request::Empty),
                Method::Version => schema_for!(request::Empty),
                Method::WalletLoad => schema_for!(request::WalletLoad),
                Method::WalletUnload => schema_for!(request::WalletUnload),
                Method::WalletList => schema_for!(request::Empty),
                Method::SignerLoadSoftware => schema_for!(request::SignerLoadSoftware),
                Method::SignerLoadJade => schema_for!(request::SignerLoadJade),
                Method::SignerLoadExternal => schema_for!(request::SignerLoadExternal),
                Method::SignerDetails => schema_for!(request::SignerDetails),
                Method::SignerUnload => schema_for!(request::SignerUnload),
                Method::SignerList => schema_for!(request::Empty),
                Method::WalletAddress => schema_for!(request::WalletAddress),
                Method::Balance => schema_for!(request::Balance),
                Method::SendMany => schema_for!(request::Send),
                Method::SinglesigDescriptor => schema_for!(request::SinglesigDescriptor),
                Method::MultisigDescriptor => schema_for!(request::MultisigDescriptor),
                Method::RegisterMultisig => schema_for!(request::RegisterMultisig),
                Method::Xpub => schema_for!(request::Xpub),
                Method::Sign => schema_for!(request::Sign),
                Method::Broadcast => schema_for!(request::Broadcast),
                Method::WalletDetails => schema_for!(request::WalletDetails),
                Method::WalletCombine => schema_for!(request::WalletCombine),
                Method::WalletPsetDetails => schema_for!(request::WalletPsetDetails),
                Method::WalletUtxos => schema_for!(request::WalletUtxos),
                Method::WalletTxs => schema_for!(request::WalletTxs),
                Method::WalletSetTxMemo => schema_for!(request::WalletSetTxMemo),
                Method::WalletSetAddrMemo => schema_for!(request::WalletSetAddrMemo),
                Method::Issue => schema_for!(request::Issue),
                Method::Reissue => schema_for!(request::Reissue),
                Method::Contract => schema_for!(request::Contract),
                Method::AssetDetails => schema_for!(request::AssetDetails),
                Method::AssetList => schema_for!(request::Empty),
                Method::AssetInsert => schema_for!(request::AssetInsert),
                Method::AssetRemove => schema_for!(request::AssetRemove),
                Method::Scan => schema_for!(request::Empty),
                Method::Stop => schema_for!(request::Empty),
                Method::SignerJadeId => schema_for!(request::Empty),
                Method::AssetPublish => schema_for!(request::AssetPublish),
            },
            Direction::Response => match self {
                Method::Schema => return serde_json::from_str(include_str!("../schema.json")),
                Method::SignerGenerate => schema_for!(response::SignerGenerate),
                Method::Version => schema_for!(response::Version),
                Method::WalletLoad => schema_for!(response::Wallet),
                Method::WalletUnload => schema_for!(response::WalletUnload),
                Method::WalletList => schema_for!(response::WalletList),
                Method::SignerLoadSoftware => schema_for!(response::Signer),
                Method::SignerLoadJade => schema_for!(response::Signer),
                Method::SignerLoadExternal => schema_for!(response::Signer),
                Method::SignerDetails => schema_for!(response::SignerDetails),
                Method::SignerUnload => schema_for!(response::SignerUnload),
                Method::SignerList => schema_for!(response::SignerList),
                Method::WalletAddress => schema_for!(response::WalletAddress),
                Method::Balance => schema_for!(response::Balance),
                Method::SendMany => schema_for!(response::Pset),
                Method::SinglesigDescriptor => schema_for!(response::SinglesigDescriptor),
                Method::MultisigDescriptor => schema_for!(response::MultisigDescriptor),
                Method::RegisterMultisig => schema_for!(response::Empty),
                Method::Xpub => schema_for!(response::Xpub),
                Method::Sign => schema_for!(response::Pset),
                Method::Broadcast => schema_for!(response::Broadcast),
                Method::WalletDetails => schema_for!(response::WalletDetails),
                Method::WalletCombine => schema_for!(response::WalletCombine),
                Method::WalletPsetDetails => schema_for!(response::WalletPsetDetails),
                Method::WalletUtxos => schema_for!(response::WalletUtxos),
                Method::WalletTxs => schema_for!(response::WalletTxs),
                Method::WalletSetTxMemo => schema_for!(response::Empty),
                Method::WalletSetAddrMemo => schema_for!(response::Empty),
                Method::Issue => schema_for!(response::Pset),
                Method::Reissue => schema_for!(response::Pset),
                Method::Contract => schema_for!(response::Contract),
                Method::AssetDetails => schema_for!(response::AssetDetails),
                Method::AssetList => schema_for!(response::AssetList),
                Method::AssetInsert => schema_for!(response::Empty),
                Method::AssetRemove => schema_for!(request::Empty),
                Method::Scan => schema_for!(response::Empty),
                Method::Stop => schema_for!(request::Empty),
                Method::SignerJadeId => schema_for!(response::JadeId),
                Method::AssetPublish => schema_for!(response::AssetPublish),
            },
        })
    }
}

impl FromStr for Method {
    type Err = MethodNotExist;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "schema" => Method::Schema,
            "signer_generate" => Method::SignerGenerate,
            "version" => Method::Version,
            "wallet_load" => Method::WalletLoad,
            "wallet_unload" => Method::WalletUnload,
            "wallet_list" => Method::WalletList,
            "signer_load_software" => Method::SignerLoadSoftware,
            "signer_load_jade" => Method::SignerLoadJade,
            "signer_load_external" => Method::SignerLoadExternal,
            "signer_details" => Method::SignerDetails,
            "signer_unload" => Method::SignerUnload,
            "signer_list" => Method::SignerList,
            "wallet_address" => Method::WalletAddress,
            "balance" => Method::Balance,
            "send_many" => Method::SendMany,
            "singlesig_descriptor" => Method::SinglesigDescriptor,
            "multisig_descriptor" => Method::MultisigDescriptor,
            "register_multisig" => Method::RegisterMultisig,
            "xpub" => Method::Xpub,
            "sign" => Method::Sign,
            "broadcast" => Method::Broadcast,
            "wallet_details" => Method::WalletDetails,
            "wallet_combine" => Method::WalletCombine,
            "wallet_pset_details" => Method::WalletPsetDetails,
            "wallet_utxos" => Method::WalletUtxos,
            "wallet_txs" => Method::WalletTxs,
            "wallet_set_tx_memo" => Method::WalletSetTxMemo,
            "wallet_set_addr_memo" => Method::WalletSetAddrMemo,
            "issue" => Method::Issue,
            "reissue" => Method::Reissue,
            "contract" => Method::Contract,
            "asset_details" => Method::AssetDetails,
            "asset_list" => Method::AssetList,
            "asset_insert" => Method::AssetInsert,
            "asset_remove" => Method::AssetRemove,
            "signer_jade_id" => Method::SignerJadeId,
            "asset_publish" => Method::AssetPublish,
            "scan" => Method::Scan,
            "stop" => Method::Stop,
            _ => {
                return Err(MethodNotExist {
                    name: s.to_string(),
                })
            }
        })
    }
}

impl std::fmt::Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Method::Schema => "schema",
            Method::SignerGenerate => "signer_generate",
            Method::Version => "version",
            Method::WalletLoad => "wallet_load",
            Method::WalletUnload => "wallet_unload",
            Method::WalletList => "wallet_list",
            Method::SignerLoadSoftware => "signer_load_software",
            Method::SignerLoadJade => "signer_load_jade",
            Method::SignerLoadExternal => "signer_load_external",
            Method::SignerDetails => "signer_details",
            Method::SignerUnload => "signer_unload",
            Method::SignerList => "signer_list",
            Method::WalletAddress => "wallet_address",
            Method::Balance => "balance",
            Method::SendMany => "send_many",
            Method::SinglesigDescriptor => "singlesig_descriptor",
            Method::MultisigDescriptor => "multisig_descriptor",
            Method::RegisterMultisig => "register_multisig",
            Method::Xpub => "xpub",
            Method::Sign => "sign",
            Method::Broadcast => "broadcast",
            Method::WalletDetails => "wallet_details",
            Method::WalletCombine => "wallet_combine",
            Method::WalletPsetDetails => "wallet_pset_details",
            Method::WalletUtxos => "wallet_utxos",
            Method::WalletTxs => "wallet_txs",
            Method::WalletSetTxMemo => "wallet_set_tx_memo",
            Method::WalletSetAddrMemo => "wallet_set_addr_memo",
            Method::Issue => "issue",
            Method::Reissue => "reissue",
            Method::Contract => "contract",
            Method::AssetDetails => "asset_details",
            Method::AssetList => "asset_list",
            Method::AssetInsert => "asset_insert",
            Method::AssetRemove => "asset_remove",
            Method::Scan => "scan",
            Method::Stop => "stop",
            Method::SignerJadeId => "signer_jade_id",
            Method::AssetPublish => "asset_publish",
        };
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod test {
    use enum_iterator::all;

    use super::Method;

    #[test]
    fn method_roundtrip() {
        let all = all::<Method>().collect::<Vec<_>>();
        for m in all {
            assert_eq!(m, m.to_string().parse().unwrap())
        }
    }
}
