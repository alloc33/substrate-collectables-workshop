use super::*;
use frame::prelude::*;
use frame::primitives::BlakeTwo256;
use frame::traits::Hash;

impl<T: Config> Pallet<T> {
	pub fn gen_dna() -> [u8; 32] {
		let parent_hash = frame_system::Pallet::<T>::parent_hash();
		let block_number = frame_system::Pallet::<T>::block_number();
		let extrinsic_index = frame_system::Pallet::<T>::extrinsic_index();
		let kitties_count = CountForKitties::<T>::get();

		let unique_payload = (parent_hash, block_number, extrinsic_index, kitties_count);

		BlakeTwo256::hash_of(&unique_payload).into()
	}

	pub fn mint(owner: T::AccountId, dna: [u8; 32]) -> DispatchResult {
		let kitty = Kitty { dna, owner: owner.clone() };
		// Check if the kitty does not already exist in our storage map
		ensure!(!Kitties::<T>::contains_key(dna), Error::<T>::DuplicateKitty);

		let current_count: u32 = CountForKitties::<T>::get();
		let new_count = current_count.checked_add(1).ok_or(Error::<T>::TooManyKitties)?;
		Kitties::<T>::insert(dna, kitty);
		CountForKitties::<T>::set(new_count);
		Self::deposit_event(Event::<T>::Created { owner });
		Ok(())
	}
}
