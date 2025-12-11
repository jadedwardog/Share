#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet_governance {
    use frame_support::{pallet_prelude::*, traits::StorageVersion};
    use frame_system::pallet_prelude::*;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn council_members)]
    pub type CouncilMembers<T: Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        MemberAdded(T::AccountId),
        MemberRemoved(T::AccountId),
        QuorumReached(u32),
    }

    #[pallet::error]
    pub enum Error<T> {
        AlreadyMember,
        NotMember,
        QuorumNotMet,
    }

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        #[pallet::constant]
        type MinCouncilSize: Get<u32>;
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000)]
        pub fn add_member(origin: OriginFor<T>, who: T::AccountId) -> DispatchResult {
            ensure_root(origin)?;
            let mut members = CouncilMembers::<T>::get();
            ensure!(!members.contains(&who), Error::<T>::AlreadyMember);
            members.push(who.clone());
            CouncilMembers::<T>::put(&members);
            Self::deposit_event(Event::MemberAdded(who));
            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn remove_member(origin: OriginFor<T>, who: T::AccountId) -> DispatchResult {
            ensure_root(origin)?;
            let mut members = CouncilMembers::<T>::get();
            ensure!(members.contains(&who), Error::<T>::NotMember);
            members.retain(|m| m != &who);
            CouncilMembers::<T>::put(&members);
            Self::deposit_event(Event::MemberRemoved(who));
            Ok(())
        }

        #[pallet::weight(5_000)]
        pub fn check_quorum(origin: OriginFor<T>) -> DispatchResult {
            ensure_root(origin)?;
            let members = CouncilMembers::<T>::get();
            let count = members.len() as u32;
            ensure!(count >= T::MinCouncilSize::get(), Error::<T>::QuorumNotMet);
            Self::deposit_event(Event::QuorumReached(count));
            Ok(())
        }
    }

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub initial_members: Vec<T::AccountId>,
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            CouncilMembers::<T>::put(&self.initial_members);
        }
    }
}
