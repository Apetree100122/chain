/*
 * This file is part of the Nodle Chain distributed at https://github.com/NodleCode/chain
 * Copyright (C) 2022  Nodle International
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use grandpa_primitives::AuthorityId as GrandpaId;
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use runtime_main::{
    constants::*, wasm_binary_unwrap, AuthorityDiscoveryConfig, BabeConfig, BalancesConfig,
    FinancialMembershipConfig, GrandpaConfig, ImOnlineConfig, KnownCustomerMembershipConfig,
    RootMembershipConfig, SessionConfig, SessionKeys, SystemConfig, TechnicalMembershipConfig,
    ValidatorsSetConfig, VestingConfig, WnodlConfig, WnodlOracleMembershipConfig,
};
use sc_chain_spec::{ChainSpecExtension, Properties};
use sc_service::ChainType;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_core::{sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};

pub use primitives::{AccountId, Balance, Block, BlockNumber, Signature};
pub use runtime_main::GenesisConfig;

type AccountPublic = <Signature as Verify>::Signer;

/// Node `ChainSpec` extensions.
///
/// Additional parameters for some Substrate core modules,
/// customizable from the chain spec.
#[derive(Default, Clone, Serialize, Deserialize, ChainSpecExtension)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
    /// Block numbers with known hashes.
    pub fork_blocks: sc_client_api::ForkBlocks<Block>,
    /// Known bad block hashes.
    pub bad_blocks: sc_client_api::BadBlocks<Block>,
    /// The light sync state extension used by the sync-state rpc.
    pub light_sync_state: sc_sync_state_rpc::LightSyncStateExtension,
}

/// Specialized `ChainSpec`.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig, Extensions>;

fn session_keys(
    grandpa: GrandpaId,
    babe: BabeId,
    im_online: ImOnlineId,
    authority_discovery: AuthorityDiscoveryId,
) -> SessionKeys {
    SessionKeys {
        grandpa,
        babe,
        im_online,
        authority_discovery,
    }
}

fn build_local_properties() -> Properties {
    let mut props = Properties::new();
    props.insert("tokenDecimals".to_string(), json!(11));
    props.insert("tokenSymbol".to_string(), json!("NODL"));

    props
}

/// Helper function to generate a crypto pair from seed
fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

/// Helper function to generate an account ID from seed
fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
    AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
    AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to generate stash, controller and session key from seed
fn authority_keys_from_seed(
    seed: &str,
) -> (
    AccountId,
    AccountId,
    GrandpaId,
    BabeId,
    ImOnlineId,
    AuthorityDiscoveryId,
) {
    (
        get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", seed)),
        get_account_id_from_seed::<sr25519::Public>(seed),
        get_from_seed::<GrandpaId>(seed),
        get_from_seed::<BabeId>(seed),
        get_from_seed::<ImOnlineId>(seed),
        get_from_seed::<AuthorityDiscoveryId>(seed),
    )
}

/// Helper function to create GenesisConfig for testing
pub fn testnet_genesis(
    initial_authorities: Vec<(
        AccountId,
        AccountId,
        GrandpaId,
        BabeId,
        ImOnlineId,
        AuthorityDiscoveryId,
    )>,
    roots: Vec<AccountId>,
    oracles: Vec<AccountId>,
    endowed_accounts: Option<Vec<AccountId>>,
    grants: Option<Vec<(AccountId, Vec<(BlockNumber, BlockNumber, u32, Balance)>)>>,
) -> GenesisConfig {
    let endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(|| {
        vec![
            get_account_id_from_seed::<sr25519::Public>("Alice"),
            get_account_id_from_seed::<sr25519::Public>("Bob"),
            get_account_id_from_seed::<sr25519::Public>("Charlie"),
            get_account_id_from_seed::<sr25519::Public>("Dave"),
            get_account_id_from_seed::<sr25519::Public>("Eve"),
            get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
            get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
            get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
            get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
            get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
        ]
    });

    let vested_grants = grants.unwrap_or_else(|| {
        vec![(
            // Ferdie has a network launch grant:
            // 1. after 1000 blocks a cliff of 1000 NODL unlocks
            // 2. for the next 100000 blocks a grant of 100 NODL unlocks every 1000 blocks
            get_account_id_from_seed::<sr25519::Public>("Ferdie"),
            vec![
                (1_000, 1, 1, 1000 * NODL),      // Cliff
                (2_000, 1_000, 100, 100 * NODL), // Vesting
            ],
        )]
    });

    const ENDOWMENT: Balance = 10_000 * NODL;
    const MIN_WRAP_AMOUNT: Balance = 1;
    const MAX_WRAP_AMOUNT: Balance = 1000 * NODL;

    GenesisConfig {
        // Core
        system: SystemConfig {
            code: wasm_binary_unwrap().to_vec(),
        },
        balances: BalancesConfig {
            balances: endowed_accounts
                .iter()
                .cloned()
                .map(|k| (k, ENDOWMENT))
                .chain(oracles.iter().map(|x| (x.clone(), ENDOWMENT)))
                .chain(roots.iter().map(|x| (x.clone(), ENDOWMENT)))
                .fold(vec![], |mut acc, (account, endowment)| {
                    if acc
                        .iter()
                        .find(|(who, _endowment)| who == &account)
                        .is_some()
                    {
                        // Increase endowment
                        acc = acc
                            .iter()
                            .cloned()
                            .map(|(cur_account, cur_endowment)| (cur_account, cur_endowment))
                            .collect::<Vec<(AccountId, Balance)>>();
                    } else {
                        acc.push((account, endowment));
                    }

                    acc
                }),
        },
        vesting: VestingConfig {
            vesting: vested_grants,
        },

        // Consensus
        session: SessionConfig {
            keys: initial_authorities
                .iter()
                .map(|x| {
                    (
                        x.0.clone(),
                        x.0.clone(),
                        session_keys(x.2.clone(), x.3.clone(), x.4.clone(), x.5.clone()),
                    )
                })
                .collect::<Vec<_>>(),
        },
        babe: BabeConfig {
            authorities: vec![],
            epoch_config: Some(runtime_main::constants::BABE_GENESIS_EPOCH_CONFIG),
        },
        im_online: ImOnlineConfig { keys: vec![] },
        authority_discovery: AuthorityDiscoveryConfig { keys: vec![] },
        grandpa: GrandpaConfig {
            authorities: vec![],
        },
        validators_set: ValidatorsSetConfig {
            members: initial_authorities
                .iter()
                .map(|x| x.0.clone())
                .collect::<Vec<_>>(),
            phantom: Default::default(),
        },

        // Governance
        // Technical Committee
        technical_committee: Default::default(),
        technical_membership: TechnicalMembershipConfig {
            members: roots.clone(),
            phantom: Default::default(),
        },
        // Financial Committee
        financial_committee: Default::default(),
        financial_membership: FinancialMembershipConfig {
            members: roots.clone(),
            phantom: Default::default(),
        },
        company_reserve: Default::default(),
        international_reserve: Default::default(),
        usa_reserve: Default::default(),
        // Root Committee
        root_committee: Default::default(),
        root_membership: RootMembershipConfig {
            members: roots.clone(),
            phantom: Default::default(),
        },

        // Allocations
        allocations_oracles: Default::default(),

        wnodl_oracle_membership: WnodlOracleMembershipConfig {
            members: oracles.clone(),
            phantom: Default::default(),
        },
        known_customer_membership: KnownCustomerMembershipConfig {
            members: endowed_accounts.clone(),
            phantom: Default::default(),
        },
        wnodl: WnodlConfig {
            min_wrapping: MIN_WRAP_AMOUNT,
            max_wrapping: MAX_WRAP_AMOUNT,
        },
    }
}

fn development_config_genesis() -> GenesisConfig {
    testnet_genesis(
        vec![authority_keys_from_seed("Alice")],
        vec![
            get_account_id_from_seed::<sr25519::Public>("Alice"),
            get_account_id_from_seed::<sr25519::Public>("Bob"),
            get_account_id_from_seed::<sr25519::Public>("Charlie"),
            get_account_id_from_seed::<sr25519::Public>("Dave"),
        ],
        vec![get_account_id_from_seed::<sr25519::Public>("Ferdie")],
        None,
        None,
    )
}

/// Development config (single validator Alice)
pub fn development_config() -> ChainSpec {
    ChainSpec::from_genesis(
        "Development",
        "dev",
        ChainType::Development,
        development_config_genesis,
        vec![],
        None,
        None,
        None,
        Some(build_local_properties()),
        Default::default(),
    )
}

fn local_testnet_genesis() -> GenesisConfig {
    testnet_genesis(
        vec![
            authority_keys_from_seed("Alice"),
            authority_keys_from_seed("Bob"),
        ],
        vec![
            get_account_id_from_seed::<sr25519::Public>("Alice"),
            get_account_id_from_seed::<sr25519::Public>("Bob"),
            get_account_id_from_seed::<sr25519::Public>("Charlie"),
        ],
        vec![get_account_id_from_seed::<sr25519::Public>("Ferdie")],
        None,
        None,
    )
}

/// Local testnet config (multivalidator Alice + Bob)
pub fn local_testnet_config() -> ChainSpec {
    ChainSpec::from_genesis(
        "Local Testnet",
        "local_testnet",
        ChainType::Local,
        local_testnet_genesis,
        vec![],
        None,
        None,
        None,
        Some(build_local_properties()),
        Default::default(),
    )
}

/// Arcadia config, from json chainspec
pub fn arcadia_config() -> ChainSpec {
    ChainSpec::from_json_bytes(&include_bytes!("../res/arcadia.json")[..]).unwrap()
}

/// Main config, from json chainspec
pub fn main_config() -> ChainSpec {
    ChainSpec::from_json_bytes(&include_bytes!("../res/main.json")[..]).unwrap()
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use sp_runtime::BuildStorage;

    #[test]
    fn test_create_development_chain_spec() {
        development_config().build_storage().unwrap();
    }

    #[test]
    fn test_create_local_testnet_chain_spec() {
        local_testnet_config().build_storage().unwrap();
    }

    #[test]
    fn test_create_arcadia_chain_spec() {
        arcadia_config().build_storage().unwrap();
    }

    #[test]
    fn test_create_main_chain_spec() {
        main_config().build_storage().unwrap();
    }
}