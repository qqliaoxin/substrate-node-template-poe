rustup install nightly-2023-01-01
rustup target add wasm32-unknown-unknown --toolchain nightly-2023-01-01
cargo +nightly-2023-01-01 build --release


列出3个常用的宏、3个常用的存储数据结构
说明：使用 Substrate-node-template 的版本为 polkadot-v0.9.30（git clone -b polkadot-v0.9.30 --depth 1 https://github.com/substrate-developer-hub/substrate-node-template.git）
常用的宏
pallet::config
pallet::storage
pallet::event
存储数据结构
pub type Something<T> = StorageValue<_, u32>;
pub type SomeNumber<T> = StorageValue<_, u8>;
pub type SomeBool<T> = StorageValue<_, bool>;
pub type SomeMoment<T:Config> = StorageValue<_, T:Moment>;
pub type SomeMap<T> = StorageMap<_, Blake2_128Concat, u8, Vec<u8>;
pub type SomeBoubleMap<T> = StorageBoubleMap<_, Blake2_128Concat,T:AccountId, Blake2_128Concat, u32, Vec<u8>;




实现存证模块的功能，包括：创建存证；撤销存证。
说明：使用 Substrate-node-template 的版本为 polkadot-v0.9.30（git clone -b polkadot-v0.9.30 --depth 1 https://github.com/substrate-developer-hub/substrate-node-template.git），提交的Github链接必须包含：⚠️代码运行的截图图片+⚠️全部代码


为存证模块添加新的功能，转移存证，接收两个参数，一个是包含的哈希值，另一个是存证的接收账户地址。
说明：使用 Substrate-node-template 的版本为 polkadot-v0.9.30（git clone -b polkadot-v0.9.30 --depth 1 https://github.com/substrate-developer-hub/substrate-node-template.git），提交的Github链接必须包含：⚠️代码运行的截图图片+⚠️全部代码


https://appbhteffsi3308.h5.xiaoeknow.com/p/course/column/p_638027c6e4b0fc5d1209ecac