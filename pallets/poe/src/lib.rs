#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use sp_std::prelude::*;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		// 存证长度
		// 因为是一个常量，所以需要加上
		#[pallet::constant]
		type MaxClaimLength: Get<u32>;
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		/// 当前模块转换成系统Event
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	pub type Proofs<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		BoundedVec<u8, T::MaxClaimLength>, // key 最大值
		(T::AccountId, T::BlockNumber),    //value 存储参数
	>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ClaimCreated(T::AccountId, Vec<u8>), // 创建事件触发
		ClaimRevoked(T::AccountId, Vec<u8>), // 掉销事件触发
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		ProofAleadyExist,
		ClaimTooLong,
		ClaimNotExist,
		NotClaimOwner,
	}

	// 区块保留函数 ，置空
	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	// 定义可调用函数
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn create_claim(origin: OriginFor<T>, claim: Vec<u8>) -> DispatchResult {
			let sender = ensure_signed(origin)?; // 校验 发送方
			let bouned_claim = BoundedVec::<u8, T::MaxClaimLength>::try_from(claim.clone())
				.map_err(|_| Error::<T>::ClaimTooLong)?; // 转换过程中出现错误返回ClaimTooLong
			ensure!(Proofs::<T>::contains_key(&bouned_claim), Error::<T>::ProofAleadyExist); // 校验链上是否存在这个值，存在返回错误
																				 // 插入键值对
			Proofs::<T>::insert(
				&bouned_claim,
				(sender.clone(), frame_system::Pallet::<T>::block_number()),
			);

			Self::deposit_event(Event::ClaimCreated(sender, claim));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		#[pallet::weight(0)]
		pub fn revoke_claim(origin: OriginFor<T>, claim: Vec<u8>) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			let bouned_claim = BoundedVec::<u8, T::MaxClaimLength>::try_from(claim.clone())
				.map_err(|_| Error::<T>::ClaimTooLong)?; // 转换过程中出现错误返回ClaimTooLong

			let owner = if let Ok((owner, _)) =
				Proofs::<T>::get(&bouned_claim).ok_or(Error::<T>::ClaimNotExist)
			{
				owner
			} else {
				todo!()
			};

			ensure!(sender == owner, Error::<T>::NotClaimOwner); // 检查 存证人和 owner 是否一致

			Proofs::<T>::remove(&bouned_claim);

			Self::deposit_event(Event::ClaimRevoked(sender, claim));
			Ok(())
		}
	}
}
