initSidebarItems({"constant":[["VERSION","This runtime version. This should not be thought of as classic Semver (major/minor/tiny). This triplet have different semantics and mis-interpretation could cause problems. In particular: bug fixes should result in an increment of `spec_version` and possibly `authoring_version`, absolutely not `impl_version` since they change the semantics of the runtime."],["WASM_BINARY",""],["WASM_BINARY_BLOATY",""]],"enum":[["Call",""],["Event",""],["OriginCaller",""]],"fn":[["make_committees_match_memberships",""],["native_version","The version infromation used to identify this runtime when compiled natively."],["wasm_binary_unwrap","Wasm binary unwrapped. If built with `SKIP_WASM_BUILD`, the function panics."]],"mod":[["api",""],["constants",""]],"struct":[["EpochDuration",""],["GenesisConfig",""],["MultiPalletMigration","Migrate from `Instance1Membership` to the new pallet prefix `TechnicalMembership`"],["Origin","The runtime origin type representing the origin of a call."],["PalletInfo","Provides an implementation of `PalletInfo` to provide information about the pallet setup in the runtime."],["Runtime",""],["RuntimeApi",""],["RuntimeApiImpl","Implements all runtime apis for the client side."],["SessionKeys",""]],"trait":[["BuildStorage","Complex storage builder stuff."]],"type":[["Address","The address format for describing accounts."],["AllModules","All modules included in the runtime as a nested tuple of types. Excludes the System pallet."],["AllModulesWithSystem","All modules included in the runtime as a nested tuple of types."],["AllPallets","All pallets included in the runtime as a nested tuple of types. Excludes the System pallet."],["AllPalletsWithSystem","All pallets included in the runtime as a nested tuple of types."],["Allocations",""],["AllocationsOracles",""],["AllocationsOraclesConfig",""],["Amendments",""],["AuthorityDiscovery",""],["AuthorityDiscoveryConfig",""],["Authorship",""],["Babe",""],["BabeConfig",""],["Balances",""],["BalancesConfig",""],["Block","Block type as expected by this runtime."],["BlockId","BlockId type as expected by this runtime."],["CheckedExtrinsic","Extrinsic type that has already been checked."],["CompanyReserve",""],["CompanyReserveConfig",""],["Contracts",""],["EmergencyShutdown",""],["Executive",""],["FinancialCommittee",""],["FinancialCommitteeConfig",""],["FinancialMembership",""],["FinancialMembershipConfig",""],["Grandpa",""],["GrandpaConfig",""],["Header","Block header type as expected by this runtime."],["Historical",""],["ImOnline",""],["ImOnlineConfig",""],["InternationalReserve",""],["InternationalReserveConfig",""],["Mandate",""],["Multisig",""],["Offences",""],["PkiRootOfTrust",""],["PkiTcr",""],["Poa",""],["RandomnessCollectiveFlip",""],["RootCommittee",""],["RootCommitteeConfig",""],["RootMembership",""],["RootMembershipConfig",""],["Scheduler",""],["Session",""],["SessionConfig",""],["SignedBlock","A Block signed with a Justification"],["SignedExtra","The SignedExtension to the basic transaction logic."],["SignedPayload","The payload being signed in transactions."],["System",""],["SystemConfig",""],["TechnicalCommittee",""],["TechnicalCommitteeConfig",""],["TechnicalMembership",""],["TechnicalMembershipConfig",""],["Timestamp",""],["TransactionPayment",""],["UncheckedExtrinsic","Unchecked extrinsic type as expected by this runtime."],["UsaReserve",""],["UsaReserveConfig",""],["Utility",""],["ValidatorsSet",""],["ValidatorsSetConfig",""],["Vesting",""],["VestingConfig",""]]});