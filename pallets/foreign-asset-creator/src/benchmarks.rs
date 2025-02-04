// Copyright Moonsong Labs
// This file is part of Moonkit.

// Moonkit is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Moonkit is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Moonkit.  If not, see <http://www.gnu.org/licenses/>.

#![cfg(feature = "runtime-benchmarks")]

use crate::{AssetId, Call, Config, Pallet};
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite};
use frame_system::RawOrigin;
use sp_arithmetic::traits::AtLeast16BitUnsigned;
use staging_xcm::latest::prelude::*;
benchmarks! {
	// This where clause allows us to create ForeignAssetTypes
	where_clause { where T::ForeignAsset: From<Location>, AssetId<T>: AtLeast16BitUnsigned }
	create_foreign_asset {
		const USER_SEED: u32 = 1;
		let manager = account("manager",  0, USER_SEED);
		let foreign_asset = T::ForeignAsset::default();
		let amount = 1u32.into();
		let asset_id: AssetId<T> = 1u16.into();

	}: _(RawOrigin::Root, foreign_asset.clone(), asset_id.clone(), manager, true, amount)
	verify {
		assert_eq!(Pallet::<T>::foreign_asset_for_id(asset_id), Some(foreign_asset));
	}

	change_existing_asset_type {
		const USER_SEED: u32 = 1;
		let manager: T::AccountId = account("manager",  0, USER_SEED);

		let foreign_asset:  T::ForeignAsset = Location::new(0, [GeneralIndex(0u128)]).into();
		let asset_id: AssetId<T> = (0u16).into();
		let amount = 1u32.into();
		Pallet::<T>::create_foreign_asset(
			RawOrigin::Root.into(),
			foreign_asset.clone(),
			asset_id.clone(),
			manager.clone(),
			true,
			amount,
		)?;

		let new_foreign_asset = T::ForeignAsset::default();
		let asset_type_to_be_changed: T::ForeignAsset = Location::new(
			0,
			[GeneralIndex((0) as u128)]
		).into();
		let asset_id_to_be_changed: AssetId<T> = (0u16).into();
	}: _(RawOrigin::Root, asset_id_to_be_changed.clone(), new_foreign_asset.clone())
	verify {
		assert_eq!(Pallet::<T>::foreign_asset_for_id(asset_id_to_be_changed), Some(new_foreign_asset.clone()));
	}

	remove_existing_asset_type {
		const USER_SEED: u32 = 1;
		let manager: T::AccountId = account("manager",  0, USER_SEED);

			let foreign_asset:  T::ForeignAsset = Location::new(0, [GeneralIndex(0u128)]).into();
			let asset_id: AssetId<T> = 0u16.into();
			let amount = 1u32.into();
			Pallet::<T>::create_foreign_asset(
				RawOrigin::Root.into(),
				foreign_asset.clone(),
				asset_id.clone(),
				manager.clone(),
				true,
				amount,
			)?;

		let asset_id_to_be_removed: AssetId<T> = 0u16.into();
	}: _(RawOrigin::Root, asset_id_to_be_removed.clone())
	verify {
		assert!(Pallet::<T>::foreign_asset_for_id(asset_id_to_be_removed).is_none());
	}

	destroy_foreign_asset {
		const USER_SEED: u32 = 1;
		let manager: T::AccountId = account("manager",  0, USER_SEED);

			let foreign_asset:  T::ForeignAsset = Location::new(0, [GeneralIndex(0u128)]).into();
			let asset_id: AssetId<T> = 0u16.into();
			let amount = 1u32.into();
			Pallet::<T>::create_foreign_asset(
				RawOrigin::Root.into(),
				foreign_asset.clone(),
				asset_id.clone(),
				manager.clone(),
				true,
				amount,
			)?;

		let asset_id_to_be_destroyed: AssetId<T> = 0u16.into();
	}: _(RawOrigin::Root, asset_id_to_be_destroyed.clone())
	verify {
		assert!(Pallet::<T>::foreign_asset_for_id(asset_id_to_be_destroyed).is_none());
	}
}

#[cfg(test)]
mod tests {
	use crate::mock::Test;
	use sp_io::TestExternalities;
	use sp_runtime::BuildStorage;

	pub fn new_test_ext() -> TestExternalities {
		let t = frame_system::GenesisConfig::<Test>::default()
			.build_storage()
			.unwrap();
		TestExternalities::new(t)
	}
}

impl_benchmark_test_suite!(
	Pallet,
	crate::benchmarks::tests::new_test_ext(),
	crate::mock::Test
);
