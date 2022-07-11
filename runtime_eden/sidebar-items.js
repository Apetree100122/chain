initSidebarItems({"constant":[["VERSION","This runtime version. This should not be thought of as classic Semver (major/minor/tiny). This triplet have different semantics and mis-interpretation could cause problems. In particular: bug fixes should result in an increment of `spec_version` and possibly `authoring_version`, absolutely not `impl_version` since they change the semantics of the runtime."],["WASM_BINARY",""],["WASM_BINARY_BLOATY",""]],"enum":[["Call",""],["Event",""],["OriginCaller",""]],"fn":[["native_version","The version infromation used to identify this runtime when compiled natively."],["wasm_binary_unwrap","Wasm binary unwrapped. If built with `SKIP_WASM_BUILD`, the function panics."]],"mod":[["api",""],["constants",""]],"struct":[["GenesisConfig",""],["Origin","The runtime origin type representing the origin of a call."],["PalletInfo","Provides an implementation of `PalletInfo` to provide information about the pallet setup in the runtime."],["ParaId","Unique identifier of a parachain."],["Runtime",""],["RuntimeApi",""],["RuntimeApiImpl","Implements all runtime apis for the client side."],["SessionKeys",""]],"trait":[["BuildStorage","Complex storage builder stuff."]],"type":[["Address","The address format for describing accounts."],["AllPallets","All pallets included in the runtime as a nested tuple of types."],["AllPalletsReversedWithSystemFirst","All pallets included in the runtime as a nested tuple of types in reversed order. With the system pallet first."],["AllPalletsWithSystem","All pallets included in the runtime as a nested tuple of types."],["AllPalletsWithSystemReversed","All pallets included in the runtime as a nested tuple of types in reversed order."],["AllPalletsWithoutSystem","All pallets included in the runtime as a nested tuple of types. Excludes the System pallet."],["AllPalletsWithoutSystemReversed","All pallets included in the runtime as a nested tuple of types in reversed order. Excludes the System pallet."],["Allocations",""],["AllocationsOracles",""],["AllocationsOraclesConfig",""],["Aura",""],["AuraConfig",""],["AuraExt",""],["AuraExtConfig",""],["AuraId","An Aura authority identifier using S/R 25519 as its crypto."],["Authorship",""],["Balances",""],["BalancesConfig",""],["Block","Block type as expected by this runtime."],["BlockId","BlockId type as expected by this runtime."],["CheckedExtrinsic","Extrinsic type that has already been checked."],["CompanyReserve",""],["CompanyReserveConfig",""],["CumulusXcm",""],["Executive",""],["Header","Block header type as expected by this runtime."],["InternationalReserve",""],["InternationalReserveConfig",""],["Mandate",""],["Multisig",""],["ParachainInfo",""],["ParachainInfoConfig",""],["ParachainSystem",""],["ParachainSystemConfig",""],["Poa",""],["Preimage",""],["RandomnessCollectiveFlip",""],["Scheduler",""],["Session",""],["SessionConfig",""],["SignedBlock","A Block signed with a Justification"],["SignedExtra","The SignedExtension to the basic transaction logic."],["SignedPayload","The payload being signed in transactions."],["System",""],["SystemConfig",""],["TechnicalCommittee",""],["TechnicalCommitteeConfig",""],["TechnicalMembership",""],["TechnicalMembershipConfig",""],["Timestamp",""],["TransactionPayment",""],["TransactionPaymentConfig",""],["UncheckedExtrinsic","Unchecked extrinsic type as expected by this runtime."],["Uniques",""],["UsaReserve",""],["UsaReserveConfig",""],["Utility",""],["ValidatorsSet",""],["ValidatorsSetConfig",""],["Vesting",""],["VestingConfig",""]]});