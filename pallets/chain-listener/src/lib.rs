#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
#[cfg(test)]
mod tests;


#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{pallet_prelude::*, weights::Weight};
    use frame_system::{pallet_prelude::*, offchain::*};
    use sp_runtime::offchain::*;
    use sp_consensus_aura::ed25519::AuthorityId;
   
    use sp_core::Public;

	#[pallet::config]
	pub trait Config: frame_system::Config + CreateSignedTransaction<Call<Self>> {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type WeightInfo: WeightInfo;
		type AuthorityId: Public;
        
    }

    #[pallet::pallet]
	
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
       
        fn offchain_worker(block_number: BlockNumberFor<T>) {
            // off-chain worker logic
            log::info!("Hello from offchain worker: {:?}", block_number);
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        // Add your extrinsics here
    }

    #[pallet::error]
    pub enum Error<T> {
        // Add your error variants here
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        // Add your event variants here
    }

    #[pallet::type_value]
    pub fn DefaultForRuntimeEvent() -> () {
        ()
    }

	pub trait WeightInfo {
		fn some_extrinsic() -> Weight {
			Weight::zero()
		}
	}
	
	impl WeightInfo for () {
		fn some_extrinsic() -> Weight {
			Weight::zero()
		}
	}
}