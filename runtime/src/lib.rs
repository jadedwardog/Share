//! Runtime module: Substrate integration, elections, logging
#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    construct_runtime, parameter_types,
    traits::{ConstU32, Everything, PalletInfo},
};
use frame_system as system;
use sp_core::H256;
use sp_runtime::{
    generic,
    traits::{BlakeTwo256, IdentityLookup},
    MultiSignature,
};


pub mod pallet_governance;


use frame_support::{
    construct_runtime, parameter_types,
    traits::{ConstU32},
    weights::Weight,
};
use frame_system as system;
use sp_runtime::traits::IdentityLookup;
use sp_core::H256;

pub type AccountId = u64;
pub type BlockNumber = u32;
pub type Index = u32;
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;

construct_runtime!(
    pub enum Runtime {
        System: frame_system,
        Governance: pallet_governance,
    }
);

parameter_types! {
    pub const BlockHashCount: u32 = 2400;
    pub const MaxConsumers: u32 = 16;
}

parameter_types! {
    pub const BlockHashCount: u32 = 2400;
    pub const MaxConsumers: u32 = 16;
}

impl system::Config for Runtime {
    type BaseCallFilter = Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    type RuntimeEvent = RuntimeEvent;
    type AccountId = AccountId;
    type Lookup = IdentityLookup<AccountId>;
    type Index = Index;
    type BlockNumber = BlockNumber;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type Header = Header;
    type Nonce = u32;
    type BlockHashCount = BlockHashCount;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ();
    type MaxConsumers = MaxConsumers;
}

impl pallet_governance::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type MinCouncilSize = ConstU32<10>;
}


construct_runtime!(
    pub enum Runtime {
        System: system,
        Governance: pallets::governance,
    }
);

