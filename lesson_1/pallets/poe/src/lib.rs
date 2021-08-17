#![cfg_attr(not(feature = "std"), no_std)]

// A module for proof of existence
pub use pallet::*;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        dispatch::DispatchResultWithPostInfo, 
        pallet_prelude::*
    };
    use frame_system::pallet_prelude::*;
    use sp_std::vec::Vec; 
    
    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type ClaimSize: Get<usize>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);
    
    #[pallet::storage]
    #[pallet::getter(fn proofs)]
    pub type Proofs<T: Config> = StorageMap<
        _, 
        Blake2_128Concat, 
        Vec<u8>, 
        (T::AccountId, T::BlockNumber)
    >;   

    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config>{
        ClaimCreated(T::AccountId, Vec<u8>),
        ClaimRevoked(T::AccountId, Vec<u8>),
        ClaimTransferred(T::AccountId, Vec<u8>, T::AccountId),
    }
    
    #[pallet::error]   // <-- Step 4. code block will replace this.
    pub enum Error<T>{
        ProofAlreadyExist,
        ClaimNotExist,
        NotProofOwner,
        ClaimSizeTooBig,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}
    
    #[pallet::call]
    impl<T: Config> Pallet<T>{
        #[pallet::weight(0)]
        pub fn create_claim(
            origin: OriginFor<T>,
            claim: Vec<u8>,
        ) -> DispatchResultWithPostInfo {

            let sender = ensure_signed(origin)?;
             
            ensure!(!Proofs::<T>::contains_key(&claim), Error::<T>::ProofAlreadyExist);

            //check the size of the claim to make sure this one should smaller than ClaimSize's upper value
            ensure!(claim.len() <= T::ClaimSize::get(), Error::<T>::ClaimSizeTooBig);

            // Get the block number from the FRAME System module.
            let current_block = <frame_system::Pallet<T>>::block_number();

            // Store the claim with the sender and block number.
            Proofs::<T>::insert(
                &claim, 
                (sender.clone(), current_block)
            );

            // Emit an event that the claim was created.
            Self::deposit_event(Event::ClaimCreated(sender, claim));

            Ok(().into())
        }
        
        #[pallet::weight(0)]
        pub fn revoke_claim(
            origin: OriginFor<T>,
            claim: Vec<u8>,
        ) -> DispatchResultWithPostInfo {
            
            let sender = ensure_signed(origin)?;

            // Get owner of the claim.
            let (owner, _) = Proofs::<T>::get(&claim).ok_or(Error::<T>::ClaimNotExist)?;

            // Verify that sender of the current call is the claim owner.
            ensure!(sender == owner, Error::<T>::NotProofOwner);

            // Remove claim from storage.
            Proofs::<T>::remove(&claim);

            // Emit an event that the claim was erased.
            Self::deposit_event(Event::ClaimRevoked(sender, claim));

            Ok(().into())
        }
        

        #[pallet::weight(0)]
        pub fn transfer_claim(
            origin: OriginFor<T>,
            claim: Vec<u8>,
            receiver: T::AccountId,
        )-> DispatchResultWithPostInfo{
            let sender = ensure_signed(origin)?;
        
            let (owner, _) = Proofs::<T>::get(&claim).ok_or(Error::<T>::ClaimNotExist)?;

            ensure!(sender == owner, Error::<T>::NotProofOwner);

            let current_block = <frame_system::Pallet<T>>::block_number();

            Proofs::<T>::insert(
                &claim, 
                (receiver.clone(), current_block)
            );
            Self::deposit_event(Event::ClaimTransferred(owner, claim, receiver));

            Ok(().into())
        }

    }
}