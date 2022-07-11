#[allow(dead_code, unused_imports, non_camel_case_types)]
pub mod api {
    use super::api as root_mod;
    pub static PALLETS: [&str; 10usize] = [
        "System",
        "RandomnessCollectiveFlip",
        "Timestamp",
        "Aura",
        "Grandpa",
        "Balances",
        "TransactionPayment",
        "Sudo",
        "TemplateModule",
        "Ibc",
    ];
    #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
    pub enum Event {
        #[codec(index = 0)]
        System(system::Event),
        #[codec(index = 4)]
        Grandpa(grandpa::Event),
        #[codec(index = 5)]
        Balances(balances::Event),
        #[codec(index = 7)]
        Sudo(sudo::Event),
        #[codec(index = 8)]
        TemplateModule(template_module::Event),
        #[codec(index = 9)]
        Ibc(ibc::Event),
    }
    pub mod system {
        use super::root_mod;
        use super::runtime_types;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct FillBlock {
                pub ratio: runtime_types::sp_arithmetic::per_things::Perbill,
            }
            impl ::subxt::Call for FillBlock {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "fill_block";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct Remark {
                pub remark: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for Remark {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "remark";
            }
            #[derive(
                :: subxt :: codec :: CompactAs,
                :: subxt :: codec :: Decode,
                :: subxt :: codec :: Encode,
                Debug,
            )]
            pub struct SetHeapPages {
                pub pages: ::core::primitive::u64,
            }
            impl ::subxt::Call for SetHeapPages {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "set_heap_pages";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct SetCode {
                pub code: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for SetCode {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "set_code";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct SetCodeWithoutChecks {
                pub code: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for SetCodeWithoutChecks {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "set_code_without_checks";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct SetStorage {
                pub items: ::std::vec::Vec<(
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::std::vec::Vec<::core::primitive::u8>,
                )>,
            }
            impl ::subxt::Call for SetStorage {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "set_storage";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct KillStorage {
                pub keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
            }
            impl ::subxt::Call for KillStorage {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "kill_storage";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct KillPrefix {
                pub prefix: ::std::vec::Vec<::core::primitive::u8>,
                pub subkeys: ::core::primitive::u32,
            }
            impl ::subxt::Call for KillPrefix {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "kill_prefix";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct RemarkWithEvent {
                pub remark: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for RemarkWithEvent {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "remark_with_event";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<X>,
            }
            impl<'a, T, X> TransactionApi<'a, T, X>
            where
                T: ::subxt::Config,
                X: ::subxt::extrinsic::ExtrinsicParams<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                #[doc = "A dispatch that will fill the block weight up to the given ratio."]
                pub fn fill_block(
                    &self,
                    ratio: runtime_types::sp_arithmetic::per_things::Perbill,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        FillBlock,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<FillBlock>()?
                        == [
                            228u8, 117u8, 251u8, 95u8, 47u8, 56u8, 32u8, 177u8, 191u8, 72u8, 75u8,
                            23u8, 193u8, 175u8, 227u8, 218u8, 127u8, 94u8, 114u8, 110u8, 215u8,
                            61u8, 162u8, 102u8, 73u8, 89u8, 218u8, 148u8, 59u8, 73u8, 59u8, 149u8,
                        ]
                    {
                        let call = FillBlock { ratio };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Make some on-chain remark."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- `O(1)`"]
                #[doc = "# </weight>"]
                pub fn remark(
                    &self,
                    remark: ::std::vec::Vec<::core::primitive::u8>,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<'a, T, X, Remark, DispatchError, root_mod::Event>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<Remark>()?
                        == [
                            186u8, 79u8, 33u8, 199u8, 216u8, 115u8, 19u8, 146u8, 220u8, 174u8,
                            98u8, 61u8, 179u8, 230u8, 40u8, 70u8, 22u8, 251u8, 77u8, 62u8, 133u8,
                            80u8, 186u8, 70u8, 135u8, 172u8, 178u8, 241u8, 69u8, 106u8, 235u8,
                            140u8,
                        ]
                    {
                        let call = Remark { remark };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Set the number of pages in the WebAssembly environment's heap."]
                pub fn set_heap_pages(
                    &self,
                    pages: ::core::primitive::u64,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        SetHeapPages,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<SetHeapPages>()?
                        == [
                            77u8, 138u8, 122u8, 55u8, 179u8, 101u8, 60u8, 137u8, 173u8, 39u8, 28u8,
                            36u8, 237u8, 243u8, 232u8, 162u8, 76u8, 176u8, 135u8, 58u8, 60u8,
                            177u8, 105u8, 136u8, 94u8, 53u8, 26u8, 31u8, 41u8, 156u8, 228u8, 241u8,
                        ]
                    {
                        let call = SetHeapPages { pages };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Set the new runtime code."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- `O(C + S)` where `C` length of `code` and `S` complexity of `can_set_code`"]
                #[doc = "- 1 call to `can_set_code`: `O(S)` (calls `sp_io::misc::runtime_version` which is"]
                #[doc = "  expensive)."]
                #[doc = "- 1 storage write (codec `O(C)`)."]
                #[doc = "- 1 digest item."]
                #[doc = "- 1 event."]
                #[doc = "The weight of this function is dependent on the runtime, but generally this is very"]
                #[doc = "expensive. We will treat this as a full block."]
                #[doc = "# </weight>"]
                pub fn set_code(
                    &self,
                    code: ::std::vec::Vec<::core::primitive::u8>,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        SetCode,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<SetCode>()?
                        == [
                            35u8, 75u8, 103u8, 203u8, 91u8, 141u8, 77u8, 95u8, 37u8, 157u8, 107u8,
                            240u8, 54u8, 242u8, 245u8, 205u8, 104u8, 165u8, 177u8, 37u8, 86u8,
                            197u8, 28u8, 202u8, 121u8, 159u8, 18u8, 204u8, 237u8, 117u8, 141u8,
                            131u8,
                        ]
                    {
                        let call = SetCode { code };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Set the new runtime code without doing any checks of the given `code`."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- `O(C)` where `C` length of `code`"]
                #[doc = "- 1 storage write (codec `O(C)`)."]
                #[doc = "- 1 digest item."]
                #[doc = "- 1 event."]
                #[doc = "The weight of this function is dependent on the runtime. We will treat this as a full"]
                #[doc = "block. # </weight>"]
                pub fn set_code_without_checks(
                    &self,
                    code: ::std::vec::Vec<::core::primitive::u8>,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        SetCodeWithoutChecks,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<SetCodeWithoutChecks>()?
                        == [
                            150u8, 148u8, 119u8, 129u8, 77u8, 216u8, 135u8, 187u8, 127u8, 24u8,
                            238u8, 15u8, 227u8, 229u8, 191u8, 217u8, 106u8, 129u8, 149u8, 79u8,
                            154u8, 78u8, 53u8, 159u8, 89u8, 69u8, 103u8, 197u8, 93u8, 161u8, 134u8,
                            17u8,
                        ]
                    {
                        let call = SetCodeWithoutChecks { code };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Set some items of storage."]
                pub fn set_storage(
                    &self,
                    items: ::std::vec::Vec<(
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::std::vec::Vec<::core::primitive::u8>,
                    )>,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        SetStorage,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<SetStorage>()?
                        == [
                            197u8, 12u8, 119u8, 205u8, 152u8, 103u8, 211u8, 170u8, 146u8, 253u8,
                            25u8, 56u8, 180u8, 146u8, 74u8, 75u8, 38u8, 108u8, 212u8, 154u8, 23u8,
                            22u8, 148u8, 175u8, 107u8, 186u8, 222u8, 13u8, 149u8, 132u8, 204u8,
                            217u8,
                        ]
                    {
                        let call = SetStorage { items };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Kill some items from storage."]
                pub fn kill_storage(
                    &self,
                    keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        KillStorage,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<KillStorage>()?
                        == [
                            154u8, 115u8, 185u8, 20u8, 126u8, 90u8, 222u8, 131u8, 199u8, 57u8,
                            184u8, 226u8, 43u8, 245u8, 161u8, 176u8, 194u8, 123u8, 139u8, 97u8,
                            97u8, 94u8, 47u8, 64u8, 204u8, 96u8, 190u8, 94u8, 216u8, 237u8, 69u8,
                            51u8,
                        ]
                    {
                        let call = KillStorage { keys };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Kill all storage items with a key that starts with the given prefix."]
                #[doc = ""]
                #[doc = "**NOTE:** We rely on the Root origin to provide us the number of subkeys under"]
                #[doc = "the prefix we are removing to accurately calculate the weight of this function."]
                pub fn kill_prefix(
                    &self,
                    prefix: ::std::vec::Vec<::core::primitive::u8>,
                    subkeys: ::core::primitive::u32,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        KillPrefix,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<KillPrefix>()?
                        == [
                            214u8, 101u8, 191u8, 241u8, 1u8, 241u8, 144u8, 116u8, 246u8, 199u8,
                            159u8, 249u8, 155u8, 164u8, 220u8, 221u8, 75u8, 33u8, 204u8, 3u8,
                            255u8, 201u8, 187u8, 238u8, 181u8, 213u8, 41u8, 105u8, 234u8, 120u8,
                            202u8, 115u8,
                        ]
                    {
                        let call = KillPrefix { prefix, subkeys };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Make some on-chain remark and emit event."]
                pub fn remark_with_event(
                    &self,
                    remark: ::std::vec::Vec<::core::primitive::u8>,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        RemarkWithEvent,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<RemarkWithEvent>()?
                        == [
                            171u8, 82u8, 75u8, 237u8, 69u8, 197u8, 223u8, 125u8, 123u8, 51u8,
                            241u8, 35u8, 202u8, 210u8, 227u8, 109u8, 1u8, 241u8, 255u8, 63u8, 33u8,
                            115u8, 156u8, 239u8, 97u8, 76u8, 193u8, 35u8, 74u8, 199u8, 43u8, 255u8,
                        ]
                    {
                        let call = RemarkWithEvent { remark };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
            }
        }
        pub type Event = runtime_types::frame_system::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "An extrinsic completed successfully."]
            pub struct ExtrinsicSuccess {
                pub dispatch_info: runtime_types::frame_support::weights::DispatchInfo,
            }
            impl ::subxt::Event for ExtrinsicSuccess {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "ExtrinsicSuccess";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "An extrinsic failed."]
            pub struct ExtrinsicFailed {
                pub dispatch_error: runtime_types::sp_runtime::DispatchError,
                pub dispatch_info: runtime_types::frame_support::weights::DispatchInfo,
            }
            impl ::subxt::Event for ExtrinsicFailed {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "ExtrinsicFailed";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "`:code` was updated."]
            pub struct CodeUpdated;
            impl ::subxt::Event for CodeUpdated {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "CodeUpdated";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "A new account was created."]
            pub struct NewAccount {
                pub account: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for NewAccount {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "NewAccount";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "An account was reaped."]
            pub struct KilledAccount {
                pub account: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for KilledAccount {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "KilledAccount";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "On on-chain remark happened."]
            pub struct Remarked {
                pub sender: ::subxt::sp_core::crypto::AccountId32,
                pub hash: ::subxt::sp_core::H256,
            }
            impl ::subxt::Event for Remarked {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "Remarked";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Account<'a>(pub &'a ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Account<'_> {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "Account";
                type Value = runtime_types::frame_system::AccountInfo<
                    ::core::primitive::u32,
                    runtime_types::pallet_balances::AccountData<::core::primitive::u128>,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct ExtrinsicCount;
            impl ::subxt::StorageEntry for ExtrinsicCount {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "ExtrinsicCount";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct BlockWeight;
            impl ::subxt::StorageEntry for BlockWeight {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "BlockWeight";
                type Value =
                    runtime_types::frame_support::weights::PerDispatchClass<::core::primitive::u64>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct AllExtrinsicsLen;
            impl ::subxt::StorageEntry for AllExtrinsicsLen {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "AllExtrinsicsLen";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct BlockHash<'a>(pub &'a ::core::primitive::u32);
            impl ::subxt::StorageEntry for BlockHash<'_> {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "BlockHash";
                type Value = ::subxt::sp_core::H256;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct ExtrinsicData<'a>(pub &'a ::core::primitive::u32);
            impl ::subxt::StorageEntry for ExtrinsicData<'_> {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "ExtrinsicData";
                type Value = ::std::vec::Vec<::core::primitive::u8>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct Number;
            impl ::subxt::StorageEntry for Number {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "Number";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ParentHash;
            impl ::subxt::StorageEntry for ParentHash {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "ParentHash";
                type Value = ::subxt::sp_core::H256;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Digest;
            impl ::subxt::StorageEntry for Digest {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "Digest";
                type Value = runtime_types::sp_runtime::generic::digest::Digest;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Events;
            impl ::subxt::StorageEntry for Events {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "Events";
                type Value = ::std::vec::Vec<
                    runtime_types::frame_system::EventRecord<
                        runtime_types::node_template_runtime::Event,
                        ::subxt::sp_core::H256,
                    >,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct EventCount;
            impl ::subxt::StorageEntry for EventCount {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "EventCount";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct EventTopics<'a>(pub &'a ::subxt::sp_core::H256);
            impl ::subxt::StorageEntry for EventTopics<'_> {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "EventTopics";
                type Value = ::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32)>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct LastRuntimeUpgrade;
            impl ::subxt::StorageEntry for LastRuntimeUpgrade {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "LastRuntimeUpgrade";
                type Value = runtime_types::frame_system::LastRuntimeUpgradeInfo;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct UpgradedToU32RefCount;
            impl ::subxt::StorageEntry for UpgradedToU32RefCount {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "UpgradedToU32RefCount";
                type Value = ::core::primitive::bool;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct UpgradedToTripleRefCount;
            impl ::subxt::StorageEntry for UpgradedToTripleRefCount {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "UpgradedToTripleRefCount";
                type Value = ::core::primitive::bool;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ExecutionPhase;
            impl ::subxt::StorageEntry for ExecutionPhase {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "ExecutionPhase";
                type Value = runtime_types::frame_system::Phase;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                #[doc = " The full account information for a particular account ID."]
                pub async fn account(
                    &self,
                    _0: &::subxt::sp_core::crypto::AccountId32,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_system::AccountInfo<
                        ::core::primitive::u32,
                        runtime_types::pallet_balances::AccountData<::core::primitive::u128>,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<Account>()?
                        == [
                            224u8, 184u8, 2u8, 14u8, 38u8, 177u8, 223u8, 98u8, 223u8, 15u8, 130u8,
                            23u8, 212u8, 69u8, 61u8, 165u8, 171u8, 61u8, 171u8, 57u8, 88u8, 71u8,
                            168u8, 172u8, 54u8, 91u8, 109u8, 231u8, 169u8, 167u8, 195u8, 46u8,
                        ]
                    {
                        let entry = Account(_0);
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " The full account information for a particular account ID."]
                pub async fn account_iter(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Account<'a>>, ::subxt::BasicError>
                {
                    if self.client.metadata().storage_hash::<Account>()?
                        == [
                            224u8, 184u8, 2u8, 14u8, 38u8, 177u8, 223u8, 98u8, 223u8, 15u8, 130u8,
                            23u8, 212u8, 69u8, 61u8, 165u8, 171u8, 61u8, 171u8, 57u8, 88u8, 71u8,
                            168u8, 172u8, 54u8, 91u8, 109u8, 231u8, 169u8, 167u8, 195u8, 46u8,
                        ]
                    {
                        self.client.storage().iter(block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " Total extrinsics count for the current block."]
                pub async fn extrinsic_count(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<ExtrinsicCount>()?
                        == [
                            223u8, 60u8, 201u8, 120u8, 36u8, 44u8, 180u8, 210u8, 242u8, 53u8,
                            222u8, 154u8, 123u8, 176u8, 249u8, 8u8, 225u8, 28u8, 232u8, 4u8, 136u8,
                            41u8, 151u8, 82u8, 189u8, 149u8, 49u8, 166u8, 139u8, 9u8, 163u8, 231u8,
                        ]
                    {
                        let entry = ExtrinsicCount;
                        self.client.storage().fetch(&entry, block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " The current weight for the block."]
                pub async fn block_weight(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::weights::PerDispatchClass<::core::primitive::u64>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<BlockWeight>()?
                        == [
                            2u8, 236u8, 190u8, 174u8, 244u8, 98u8, 194u8, 168u8, 89u8, 208u8, 7u8,
                            45u8, 175u8, 171u8, 177u8, 121u8, 215u8, 190u8, 184u8, 195u8, 49u8,
                            133u8, 44u8, 1u8, 181u8, 215u8, 89u8, 84u8, 255u8, 16u8, 57u8, 152u8,
                        ]
                    {
                        let entry = BlockWeight;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " Total length (in bytes) for all extrinsics put together, for the current block."]
                pub async fn all_extrinsics_len(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<AllExtrinsicsLen>()?
                        == [
                            202u8, 145u8, 209u8, 225u8, 40u8, 220u8, 174u8, 74u8, 93u8, 164u8,
                            254u8, 248u8, 254u8, 192u8, 32u8, 117u8, 96u8, 149u8, 53u8, 145u8,
                            219u8, 64u8, 234u8, 18u8, 217u8, 200u8, 203u8, 141u8, 145u8, 28u8,
                            134u8, 60u8,
                        ]
                    {
                        let entry = AllExtrinsicsLen;
                        self.client.storage().fetch(&entry, block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " Map of block numbers to block hashes."]
                pub async fn block_hash(
                    &self,
                    _0: &::core::primitive::u32,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::sp_core::H256, ::subxt::BasicError>
                {
                    if self.client.metadata().storage_hash::<BlockHash>()?
                        == [
                            24u8, 99u8, 146u8, 142u8, 205u8, 166u8, 4u8, 32u8, 218u8, 213u8, 24u8,
                            236u8, 45u8, 116u8, 145u8, 204u8, 27u8, 141u8, 169u8, 249u8, 111u8,
                            141u8, 37u8, 136u8, 45u8, 73u8, 167u8, 217u8, 118u8, 206u8, 246u8,
                            120u8,
                        ]
                    {
                        let entry = BlockHash(_0);
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " Map of block numbers to block hashes."]
                pub async fn block_hash_iter(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, BlockHash<'a>>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<BlockHash>()?
                        == [
                            24u8, 99u8, 146u8, 142u8, 205u8, 166u8, 4u8, 32u8, 218u8, 213u8, 24u8,
                            236u8, 45u8, 116u8, 145u8, 204u8, 27u8, 141u8, 169u8, 249u8, 111u8,
                            141u8, 37u8, 136u8, 45u8, 73u8, 167u8, 217u8, 118u8, 206u8, 246u8,
                            120u8,
                        ]
                    {
                        self.client.storage().iter(block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " Extrinsics data for the current block (maps an extrinsic's index to its data)."]
                pub async fn extrinsic_data(
                    &self,
                    _0: &::core::primitive::u32,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<ExtrinsicData>()?
                        == [
                            210u8, 224u8, 211u8, 186u8, 118u8, 210u8, 185u8, 194u8, 238u8, 211u8,
                            254u8, 73u8, 67u8, 184u8, 31u8, 229u8, 168u8, 125u8, 98u8, 23u8, 241u8,
                            59u8, 49u8, 86u8, 126u8, 9u8, 114u8, 163u8, 160u8, 62u8, 50u8, 67u8,
                        ]
                    {
                        let entry = ExtrinsicData(_0);
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " Extrinsics data for the current block (maps an extrinsic's index to its data)."]
                pub async fn extrinsic_data_iter(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ExtrinsicData<'a>>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<ExtrinsicData>()?
                        == [
                            210u8, 224u8, 211u8, 186u8, 118u8, 210u8, 185u8, 194u8, 238u8, 211u8,
                            254u8, 73u8, 67u8, 184u8, 31u8, 229u8, 168u8, 125u8, 98u8, 23u8, 241u8,
                            59u8, 49u8, 86u8, 126u8, 9u8, 114u8, 163u8, 160u8, 62u8, 50u8, 67u8,
                        ]
                    {
                        self.client.storage().iter(block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " The current block number being processed. Set by `execute_block`."]
                pub async fn number(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    if self.client.metadata().storage_hash::<Number>()?
                        == [
                            228u8, 96u8, 102u8, 190u8, 252u8, 130u8, 239u8, 172u8, 126u8, 235u8,
                            246u8, 139u8, 208u8, 15u8, 88u8, 245u8, 141u8, 232u8, 43u8, 204u8,
                            36u8, 87u8, 211u8, 141u8, 187u8, 68u8, 236u8, 70u8, 193u8, 235u8,
                            164u8, 191u8,
                        ]
                    {
                        let entry = Number;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " Hash of the previous block."]
                pub async fn parent_hash(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::sp_core::H256, ::subxt::BasicError>
                {
                    if self.client.metadata().storage_hash::<ParentHash>()?
                        == [
                            194u8, 221u8, 147u8, 22u8, 68u8, 141u8, 32u8, 6u8, 202u8, 39u8, 164u8,
                            184u8, 69u8, 126u8, 190u8, 101u8, 215u8, 27u8, 127u8, 157u8, 200u8,
                            69u8, 170u8, 139u8, 232u8, 27u8, 254u8, 181u8, 183u8, 105u8, 111u8,
                            177u8,
                        ]
                    {
                        let entry = ParentHash;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " Digest of the current block, also part of the block header."]
                pub async fn digest(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::sp_runtime::generic::digest::Digest,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<Digest>()?
                        == [
                            10u8, 176u8, 13u8, 228u8, 226u8, 42u8, 210u8, 151u8, 107u8, 212u8,
                            136u8, 15u8, 38u8, 182u8, 225u8, 12u8, 250u8, 56u8, 193u8, 243u8,
                            219u8, 113u8, 95u8, 233u8, 21u8, 229u8, 125u8, 146u8, 92u8, 250u8,
                            32u8, 168u8,
                        ]
                    {
                        let entry = Digest;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " Events deposited for the current block."]
                #[doc = ""]
                #[doc = " NOTE: The item is unbound and should therefore never be read on chain."]
                #[doc = " It could otherwise inflate the PoV size of a block."]
                #[doc = ""]
                #[doc = " Events have a large in-memory size. Box the events to not go out-of-memory"]
                #[doc = " just in case someone still reads them from within the runtime."]
                pub async fn events(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<
                        runtime_types::frame_system::EventRecord<
                            runtime_types::node_template_runtime::Event,
                            ::subxt::sp_core::H256,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<Events>()?
                        == [
                            155u8, 21u8, 173u8, 86u8, 58u8, 97u8, 39u8, 114u8, 202u8, 201u8, 172u8,
                            101u8, 16u8, 144u8, 129u8, 39u8, 125u8, 151u8, 105u8, 103u8, 233u8,
                            124u8, 213u8, 183u8, 205u8, 186u8, 2u8, 130u8, 223u8, 200u8, 157u8,
                            228u8,
                        ]
                    {
                        let entry = Events;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " The number of events in the `Events<T>` list."]
                pub async fn event_count(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    if self.client.metadata().storage_hash::<EventCount>()?
                        == [
                            236u8, 93u8, 90u8, 177u8, 250u8, 211u8, 138u8, 187u8, 26u8, 208u8,
                            203u8, 113u8, 221u8, 233u8, 227u8, 9u8, 249u8, 25u8, 202u8, 185u8,
                            161u8, 144u8, 167u8, 104u8, 127u8, 187u8, 38u8, 18u8, 52u8, 61u8, 66u8,
                            112u8,
                        ]
                    {
                        let entry = EventCount;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " Mapping between a topic (represented by T::Hash) and a vector of indexes"]
                #[doc = " of events in the `<Events<T>>` list."]
                #[doc = ""]
                #[doc = " All topic vectors have deterministic storage locations depending on the topic. This"]
                #[doc = " allows light-clients to leverage the changes trie storage tracking mechanism and"]
                #[doc = " in case of changes fetch the list of events of interest."]
                #[doc = ""]
                #[doc = " The value has the type `(T::BlockNumber, EventIndex)` because if we used only just"]
                #[doc = " the `EventIndex` then in case if the topic has the same contents on the next block"]
                #[doc = " no notification will be triggered thus the event might be lost."]
                pub async fn event_topics(
                    &self,
                    _0: &::subxt::sp_core::H256,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32)>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<EventTopics>()?
                        == [
                            231u8, 73u8, 172u8, 223u8, 210u8, 145u8, 151u8, 102u8, 73u8, 23u8,
                            140u8, 55u8, 97u8, 40u8, 219u8, 239u8, 229u8, 177u8, 72u8, 41u8, 93u8,
                            178u8, 7u8, 209u8, 57u8, 86u8, 153u8, 252u8, 86u8, 152u8, 245u8, 179u8,
                        ]
                    {
                        let entry = EventTopics(_0);
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " Mapping between a topic (represented by T::Hash) and a vector of indexes"]
                #[doc = " of events in the `<Events<T>>` list."]
                #[doc = ""]
                #[doc = " All topic vectors have deterministic storage locations depending on the topic. This"]
                #[doc = " allows light-clients to leverage the changes trie storage tracking mechanism and"]
                #[doc = " in case of changes fetch the list of events of interest."]
                #[doc = ""]
                #[doc = " The value has the type `(T::BlockNumber, EventIndex)` because if we used only just"]
                #[doc = " the `EventIndex` then in case if the topic has the same contents on the next block"]
                #[doc = " no notification will be triggered thus the event might be lost."]
                pub async fn event_topics_iter(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, EventTopics<'a>>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<EventTopics>()?
                        == [
                            231u8, 73u8, 172u8, 223u8, 210u8, 145u8, 151u8, 102u8, 73u8, 23u8,
                            140u8, 55u8, 97u8, 40u8, 219u8, 239u8, 229u8, 177u8, 72u8, 41u8, 93u8,
                            178u8, 7u8, 209u8, 57u8, 86u8, 153u8, 252u8, 86u8, 152u8, 245u8, 179u8,
                        ]
                    {
                        self.client.storage().iter(block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " Stores the `spec_version` and `spec_name` of when the last runtime upgrade happened."]
                pub async fn last_runtime_upgrade(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<runtime_types::frame_system::LastRuntimeUpgradeInfo>,
                    ::subxt::BasicError,
                > {
                    if self
                        .client
                        .metadata()
                        .storage_hash::<LastRuntimeUpgrade>()?
                        == [
                            219u8, 153u8, 158u8, 38u8, 45u8, 65u8, 151u8, 137u8, 53u8, 76u8, 11u8,
                            181u8, 218u8, 248u8, 125u8, 190u8, 100u8, 240u8, 173u8, 75u8, 179u8,
                            137u8, 198u8, 197u8, 248u8, 185u8, 118u8, 58u8, 42u8, 165u8, 125u8,
                            119u8,
                        ]
                    {
                        let entry = LastRuntimeUpgrade;
                        self.client.storage().fetch(&entry, block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " True if we have upgraded so that `type RefCount` is `u32`. False (default) if not."]
                pub async fn upgraded_to_u32_ref_count(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::bool, ::subxt::BasicError>
                {
                    if self
                        .client
                        .metadata()
                        .storage_hash::<UpgradedToU32RefCount>()?
                        == [
                            171u8, 88u8, 244u8, 92u8, 122u8, 67u8, 27u8, 18u8, 59u8, 175u8, 175u8,
                            178u8, 20u8, 150u8, 213u8, 59u8, 222u8, 141u8, 32u8, 107u8, 3u8, 114u8,
                            83u8, 250u8, 180u8, 233u8, 152u8, 54u8, 187u8, 99u8, 131u8, 204u8,
                        ]
                    {
                        let entry = UpgradedToU32RefCount;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " True if we have upgraded so that AccountInfo contains three types of `RefCount`. False"]
                #[doc = " (default) if not."]
                pub async fn upgraded_to_triple_ref_count(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::bool, ::subxt::BasicError>
                {
                    if self
                        .client
                        .metadata()
                        .storage_hash::<UpgradedToTripleRefCount>()?
                        == [
                            90u8, 33u8, 56u8, 86u8, 90u8, 101u8, 89u8, 133u8, 203u8, 56u8, 201u8,
                            210u8, 244u8, 232u8, 150u8, 18u8, 51u8, 105u8, 14u8, 230u8, 103u8,
                            155u8, 246u8, 99u8, 53u8, 207u8, 225u8, 128u8, 186u8, 76u8, 40u8,
                            185u8,
                        ]
                    {
                        let entry = UpgradedToTripleRefCount;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " The execution phase of the block."]
                pub async fn execution_phase(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<runtime_types::frame_system::Phase>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<ExecutionPhase>()?
                        == [
                            174u8, 13u8, 230u8, 220u8, 239u8, 161u8, 172u8, 122u8, 188u8, 95u8,
                            141u8, 118u8, 91u8, 158u8, 111u8, 145u8, 243u8, 173u8, 226u8, 212u8,
                            187u8, 118u8, 94u8, 132u8, 221u8, 244u8, 61u8, 148u8, 217u8, 30u8,
                            238u8, 225u8,
                        ]
                    {
                        let entry = ExecutionPhase;
                        self.client.storage().fetch(&entry, block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> ConstantsApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                #[doc = " Block & extrinsics weights: base values and limits."]
                pub fn block_weights(
                    &self,
                ) -> ::core::result::Result<
                    runtime_types::frame_system::limits::BlockWeights,
                    ::subxt::BasicError,
                > {
                    if self
                        .client
                        .metadata()
                        .constant_hash("System", "BlockWeights")?
                        == [
                            217u8, 234u8, 30u8, 130u8, 41u8, 60u8, 153u8, 179u8, 70u8, 50u8, 191u8,
                            186u8, 122u8, 139u8, 169u8, 116u8, 164u8, 253u8, 204u8, 106u8, 25u8,
                            239u8, 108u8, 31u8, 117u8, 255u8, 11u8, 212u8, 151u8, 60u8, 68u8,
                            181u8,
                        ]
                    {
                        let pallet = self.client.metadata().pallet("System")?;
                        let constant = pallet.constant("BlockWeights")?;
                        let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                        Ok(value)
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " The maximum length of a block (in bytes)."]
                pub fn block_length(
                    &self,
                ) -> ::core::result::Result<
                    runtime_types::frame_system::limits::BlockLength,
                    ::subxt::BasicError,
                > {
                    if self
                        .client
                        .metadata()
                        .constant_hash("System", "BlockLength")?
                        == [
                            120u8, 249u8, 182u8, 103u8, 246u8, 214u8, 149u8, 44u8, 42u8, 64u8, 2u8,
                            56u8, 157u8, 184u8, 43u8, 195u8, 214u8, 251u8, 207u8, 207u8, 249u8,
                            105u8, 203u8, 108u8, 179u8, 93u8, 93u8, 246u8, 40u8, 175u8, 160u8,
                            114u8,
                        ]
                    {
                        let pallet = self.client.metadata().pallet("System")?;
                        let constant = pallet.constant("BlockLength")?;
                        let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                        Ok(value)
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " Maximum number of block number to block hash mappings to keep (oldest pruned first)."]
                pub fn block_hash_count(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    if self
                        .client
                        .metadata()
                        .constant_hash("System", "BlockHashCount")?
                        == [
                            123u8, 126u8, 182u8, 103u8, 71u8, 187u8, 233u8, 8u8, 47u8, 226u8,
                            159u8, 139u8, 0u8, 59u8, 190u8, 135u8, 189u8, 77u8, 190u8, 81u8, 39u8,
                            198u8, 224u8, 219u8, 70u8, 143u8, 6u8, 132u8, 196u8, 61u8, 117u8,
                            194u8,
                        ]
                    {
                        let pallet = self.client.metadata().pallet("System")?;
                        let constant = pallet.constant("BlockHashCount")?;
                        let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                        Ok(value)
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " The weight of runtime database operations the runtime can invoke."]
                pub fn db_weight(
                    &self,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::weights::RuntimeDbWeight,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().constant_hash("System", "DbWeight")?
                        == [
                            203u8, 8u8, 106u8, 152u8, 74u8, 132u8, 2u8, 132u8, 244u8, 106u8, 147u8,
                            12u8, 93u8, 80u8, 61u8, 158u8, 172u8, 178u8, 228u8, 125u8, 213u8,
                            102u8, 75u8, 210u8, 64u8, 185u8, 204u8, 84u8, 10u8, 164u8, 204u8, 62u8,
                        ]
                    {
                        let pallet = self.client.metadata().pallet("System")?;
                        let constant = pallet.constant("DbWeight")?;
                        let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                        Ok(value)
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " Get the chain's current version."]
                pub fn version(
                    &self,
                ) -> ::core::result::Result<
                    runtime_types::sp_version::RuntimeVersion,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().constant_hash("System", "Version")?
                        == [
                            58u8, 250u8, 50u8, 248u8, 238u8, 235u8, 107u8, 162u8, 23u8, 194u8, 8u8,
                            122u8, 221u8, 90u8, 163u8, 245u8, 246u8, 112u8, 164u8, 58u8, 124u8,
                            94u8, 205u8, 220u8, 23u8, 80u8, 153u8, 76u8, 141u8, 158u8, 91u8, 253u8,
                        ]
                    {
                        let pallet = self.client.metadata().pallet("System")?;
                        let constant = pallet.constant("Version")?;
                        let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                        Ok(value)
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " The designated SS85 prefix of this chain."]
                #[doc = ""]
                #[doc = " This replaces the \"ss58Format\" property declared in the chain spec. Reason is"]
                #[doc = " that the runtime should know about the prefix in order to make use of it as"]
                #[doc = " an identifier of the chain."]
                pub fn ss58_prefix(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u16, ::subxt::BasicError>
                {
                    if self
                        .client
                        .metadata()
                        .constant_hash("System", "SS58Prefix")?
                        == [
                            197u8, 217u8, 49u8, 68u8, 82u8, 238u8, 120u8, 50u8, 91u8, 58u8, 6u8,
                            156u8, 40u8, 1u8, 241u8, 213u8, 141u8, 74u8, 83u8, 115u8, 117u8, 41u8,
                            119u8, 50u8, 140u8, 136u8, 163u8, 185u8, 34u8, 190u8, 60u8, 97u8,
                        ]
                    {
                        let pallet = self.client.metadata().pallet("System")?;
                        let constant = pallet.constant("SS58Prefix")?;
                        let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                        Ok(value)
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
            }
        }
    }
    pub mod randomness_collective_flip {
        use super::root_mod;
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub struct RandomMaterial;
            impl ::subxt::StorageEntry for RandomMaterial {
                const PALLET: &'static str = "RandomnessCollectiveFlip";
                const STORAGE: &'static str = "RandomMaterial";
                type Value = runtime_types::sp_runtime::bounded::bounded_vec::BoundedVec<
                    ::subxt::sp_core::H256,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                #[doc = " Series of block headers from the last 81 blocks that acts as random seed material. This"]
                #[doc = " is arranged as a ring buffer with `block_number % 81` being the index into the `Vec` of"]
                #[doc = " the oldest hash."]
                pub async fn random_material(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::sp_runtime::bounded::bounded_vec::BoundedVec<
                        ::subxt::sp_core::H256,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<RandomMaterial>()?
                        == [
                            60u8, 176u8, 119u8, 155u8, 161u8, 136u8, 144u8, 88u8, 26u8, 57u8,
                            142u8, 34u8, 5u8, 37u8, 115u8, 11u8, 90u8, 222u8, 147u8, 194u8, 82u8,
                            194u8, 70u8, 227u8, 175u8, 198u8, 235u8, 24u8, 7u8, 87u8, 203u8, 182u8,
                        ]
                    {
                        let entry = RandomMaterial;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
            }
        }
    }
    pub mod timestamp {
        use super::root_mod;
        use super::runtime_types;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct Set {
                #[codec(compact)]
                pub now: ::core::primitive::u64,
            }
            impl ::subxt::Call for Set {
                const PALLET: &'static str = "Timestamp";
                const FUNCTION: &'static str = "set";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<X>,
            }
            impl<'a, T, X> TransactionApi<'a, T, X>
            where
                T: ::subxt::Config,
                X: ::subxt::extrinsic::ExtrinsicParams<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                #[doc = "Set the current time."]
                #[doc = ""]
                #[doc = "This call should be invoked exactly once per block. It will panic at the finalization"]
                #[doc = "phase, if this call hasn't been invoked by that time."]
                #[doc = ""]
                #[doc = "The timestamp should be greater than the previous one by the amount specified by"]
                #[doc = "`MinimumPeriod`."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be `Inherent`."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- `O(1)` (Note that implementations of `OnTimestampSet` must also be `O(1)`)"]
                #[doc = "- 1 storage read and 1 storage mutation (codec `O(1)`). (because of `DidUpdate::take` in"]
                #[doc = "  `on_finalize`)"]
                #[doc = "- 1 event handler `on_timestamp_set`. Must be `O(1)`."]
                #[doc = "# </weight>"]
                pub fn set(
                    &self,
                    now: ::core::primitive::u64,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<'a, T, X, Set, DispatchError, root_mod::Event>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<Set>()?
                        == [
                            191u8, 73u8, 102u8, 150u8, 65u8, 157u8, 172u8, 194u8, 7u8, 72u8, 1u8,
                            35u8, 54u8, 99u8, 245u8, 139u8, 40u8, 136u8, 245u8, 53u8, 167u8, 100u8,
                            143u8, 244u8, 160u8, 5u8, 18u8, 130u8, 77u8, 160u8, 227u8, 51u8,
                        ]
                    {
                        let call = Set { now };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Now;
            impl ::subxt::StorageEntry for Now {
                const PALLET: &'static str = "Timestamp";
                const STORAGE: &'static str = "Now";
                type Value = ::core::primitive::u64;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct DidUpdate;
            impl ::subxt::StorageEntry for DidUpdate {
                const PALLET: &'static str = "Timestamp";
                const STORAGE: &'static str = "DidUpdate";
                type Value = ::core::primitive::bool;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                #[doc = " Current time for the current block."]
                pub async fn now(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    if self.client.metadata().storage_hash::<Now>()?
                        == [
                            148u8, 53u8, 50u8, 54u8, 13u8, 161u8, 57u8, 150u8, 16u8, 83u8, 144u8,
                            221u8, 59u8, 75u8, 158u8, 130u8, 39u8, 123u8, 106u8, 134u8, 202u8,
                            185u8, 83u8, 85u8, 60u8, 41u8, 120u8, 96u8, 210u8, 34u8, 2u8, 250u8,
                        ]
                    {
                        let entry = Now;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " Did the timestamp get updated in this block?"]
                pub async fn did_update(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::bool, ::subxt::BasicError>
                {
                    if self.client.metadata().storage_hash::<DidUpdate>()?
                        == [
                            70u8, 13u8, 92u8, 186u8, 80u8, 151u8, 167u8, 90u8, 158u8, 232u8, 175u8,
                            13u8, 103u8, 135u8, 2u8, 78u8, 16u8, 6u8, 39u8, 158u8, 167u8, 85u8,
                            27u8, 47u8, 122u8, 73u8, 127u8, 26u8, 35u8, 168u8, 72u8, 204u8,
                        ]
                    {
                        let entry = DidUpdate;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> ConstantsApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                #[doc = " The minimum period between blocks. Beware that this is different to the *expected*"]
                #[doc = " period that the block production apparatus provides. Your chosen consensus system will"]
                #[doc = " generally work with this to determine a sensible block time. e.g. For Aura, it will be"]
                #[doc = " double this period on default settings."]
                pub fn minimum_period(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    if self
                        .client
                        .metadata()
                        .constant_hash("Timestamp", "MinimumPeriod")?
                        == [
                            141u8, 242u8, 40u8, 24u8, 83u8, 43u8, 33u8, 194u8, 156u8, 149u8, 219u8,
                            61u8, 10u8, 123u8, 120u8, 247u8, 228u8, 22u8, 25u8, 24u8, 214u8, 188u8,
                            54u8, 135u8, 240u8, 162u8, 41u8, 216u8, 3u8, 58u8, 238u8, 39u8,
                        ]
                    {
                        let pallet = self.client.metadata().pallet("Timestamp")?;
                        let constant = pallet.constant("MinimumPeriod")?;
                        let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                        Ok(value)
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
            }
        }
    }
    pub mod aura {
        use super::root_mod;
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub struct Authorities;
            impl ::subxt::StorageEntry for Authorities {
                const PALLET: &'static str = "Aura";
                const STORAGE: &'static str = "Authorities";
                type Value = runtime_types::sp_runtime::bounded::bounded_vec::BoundedVec<
                    runtime_types::sp_consensus_aura::sr25519::app_sr25519::Public,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct CurrentSlot;
            impl ::subxt::StorageEntry for CurrentSlot {
                const PALLET: &'static str = "Aura";
                const STORAGE: &'static str = "CurrentSlot";
                type Value = runtime_types::sp_consensus_slots::Slot;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                #[doc = " The current authority set."]
                pub async fn authorities(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::sp_runtime::bounded::bounded_vec::BoundedVec<
                        runtime_types::sp_consensus_aura::sr25519::app_sr25519::Public,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<Authorities>()?
                        == [
                            168u8, 101u8, 224u8, 96u8, 254u8, 152u8, 213u8, 141u8, 46u8, 181u8,
                            131u8, 23u8, 218u8, 24u8, 145u8, 111u8, 161u8, 192u8, 253u8, 29u8,
                            128u8, 92u8, 125u8, 159u8, 242u8, 144u8, 253u8, 174u8, 50u8, 190u8,
                            148u8, 193u8,
                        ]
                    {
                        let entry = Authorities;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " The current slot of this block."]
                #[doc = ""]
                #[doc = " This will be set in `on_initialize`."]
                pub async fn current_slot(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::sp_consensus_slots::Slot,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<CurrentSlot>()?
                        == [
                            233u8, 102u8, 77u8, 99u8, 103u8, 50u8, 151u8, 229u8, 46u8, 226u8,
                            181u8, 37u8, 117u8, 204u8, 234u8, 120u8, 116u8, 166u8, 80u8, 188u8,
                            92u8, 154u8, 137u8, 150u8, 79u8, 164u8, 29u8, 203u8, 2u8, 51u8, 123u8,
                            104u8,
                        ]
                    {
                        let entry = CurrentSlot;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
            }
        }
    }
    pub mod grandpa {
        use super::root_mod;
        use super::runtime_types;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct ReportEquivocation {
                pub equivocation_proof: ::std::boxed::Box<
                    runtime_types::sp_finality_grandpa::EquivocationProof<
                        ::subxt::sp_core::H256,
                        ::core::primitive::u32,
                    >,
                >,
                pub key_owner_proof: runtime_types::sp_core::Void,
            }
            impl ::subxt::Call for ReportEquivocation {
                const PALLET: &'static str = "Grandpa";
                const FUNCTION: &'static str = "report_equivocation";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct ReportEquivocationUnsigned {
                pub equivocation_proof: ::std::boxed::Box<
                    runtime_types::sp_finality_grandpa::EquivocationProof<
                        ::subxt::sp_core::H256,
                        ::core::primitive::u32,
                    >,
                >,
                pub key_owner_proof: runtime_types::sp_core::Void,
            }
            impl ::subxt::Call for ReportEquivocationUnsigned {
                const PALLET: &'static str = "Grandpa";
                const FUNCTION: &'static str = "report_equivocation_unsigned";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct NoteStalled {
                pub delay: ::core::primitive::u32,
                pub best_finalized_block_number: ::core::primitive::u32,
            }
            impl ::subxt::Call for NoteStalled {
                const PALLET: &'static str = "Grandpa";
                const FUNCTION: &'static str = "note_stalled";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<X>,
            }
            impl<'a, T, X> TransactionApi<'a, T, X>
            where
                T: ::subxt::Config,
                X: ::subxt::extrinsic::ExtrinsicParams<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                #[doc = "Report voter equivocation/misbehavior. This method will verify the"]
                #[doc = "equivocation proof and validate the given key ownership proof"]
                #[doc = "against the extracted offender. If both are valid, the offence"]
                #[doc = "will be reported."]
                pub fn report_equivocation(
                    &self,
                    equivocation_proof: runtime_types::sp_finality_grandpa::EquivocationProof<
                        ::subxt::sp_core::H256,
                        ::core::primitive::u32,
                    >,
                    key_owner_proof: runtime_types::sp_core::Void,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        ReportEquivocation,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<ReportEquivocation>()?
                        == [
                            255u8, 59u8, 201u8, 1u8, 171u8, 157u8, 232u8, 62u8, 75u8, 212u8, 86u8,
                            247u8, 132u8, 32u8, 114u8, 38u8, 121u8, 205u8, 61u8, 241u8, 16u8,
                            241u8, 178u8, 191u8, 52u8, 33u8, 34u8, 110u8, 18u8, 6u8, 216u8, 130u8,
                        ]
                    {
                        let call = ReportEquivocation {
                            equivocation_proof: ::std::boxed::Box::new(equivocation_proof),
                            key_owner_proof,
                        };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Report voter equivocation/misbehavior. This method will verify the"]
                #[doc = "equivocation proof and validate the given key ownership proof"]
                #[doc = "against the extracted offender. If both are valid, the offence"]
                #[doc = "will be reported."]
                #[doc = ""]
                #[doc = "This extrinsic must be called unsigned and it is expected that only"]
                #[doc = "block authors will call it (validated in `ValidateUnsigned`), as such"]
                #[doc = "if the block author is defined it will be defined as the equivocation"]
                #[doc = "reporter."]
                pub fn report_equivocation_unsigned(
                    &self,
                    equivocation_proof: runtime_types::sp_finality_grandpa::EquivocationProof<
                        ::subxt::sp_core::H256,
                        ::core::primitive::u32,
                    >,
                    key_owner_proof: runtime_types::sp_core::Void,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        ReportEquivocationUnsigned,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self
                        .client
                        .metadata()
                        .call_hash::<ReportEquivocationUnsigned>()?
                        == [
                            193u8, 179u8, 43u8, 34u8, 77u8, 194u8, 203u8, 216u8, 112u8, 101u8,
                            70u8, 127u8, 136u8, 123u8, 163u8, 143u8, 83u8, 137u8, 142u8, 226u8,
                            5u8, 100u8, 225u8, 32u8, 7u8, 195u8, 78u8, 76u8, 85u8, 114u8, 76u8,
                            109u8,
                        ]
                    {
                        let call = ReportEquivocationUnsigned {
                            equivocation_proof: ::std::boxed::Box::new(equivocation_proof),
                            key_owner_proof,
                        };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Note that the current authority set of the GRANDPA finality gadget has stalled."]
                #[doc = ""]
                #[doc = "This will trigger a forced authority set change at the beginning of the next session, to"]
                #[doc = "be enacted `delay` blocks after that. The `delay` should be high enough to safely assume"]
                #[doc = "that the block signalling the forced change will not be re-orged e.g. 1000 blocks."]
                #[doc = "The block production rate (which may be slowed down because of finality lagging) should"]
                #[doc = "be taken into account when choosing the `delay`. The GRANDPA voters based on the new"]
                #[doc = "authority will start voting on top of `best_finalized_block_number` for new finalized"]
                #[doc = "blocks. `best_finalized_block_number` should be the highest of the latest finalized"]
                #[doc = "block of all validators of the new authority set."]
                #[doc = ""]
                #[doc = "Only callable by root."]
                pub fn note_stalled(
                    &self,
                    delay: ::core::primitive::u32,
                    best_finalized_block_number: ::core::primitive::u32,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        NoteStalled,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<NoteStalled>()?
                        == [
                            227u8, 98u8, 249u8, 158u8, 96u8, 124u8, 72u8, 188u8, 27u8, 215u8, 73u8,
                            62u8, 103u8, 79u8, 38u8, 48u8, 212u8, 88u8, 233u8, 187u8, 11u8, 95u8,
                            39u8, 247u8, 55u8, 184u8, 228u8, 102u8, 13u8, 251u8, 52u8, 206u8,
                        ]
                    {
                        let call = NoteStalled {
                            delay,
                            best_finalized_block_number,
                        };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
            }
        }
        pub type Event = runtime_types::pallet_grandpa::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "New authority set has been applied."]
            pub struct NewAuthorities {
                pub authority_set: ::std::vec::Vec<(
                    runtime_types::sp_finality_grandpa::app::Public,
                    ::core::primitive::u64,
                )>,
            }
            impl ::subxt::Event for NewAuthorities {
                const PALLET: &'static str = "Grandpa";
                const EVENT: &'static str = "NewAuthorities";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "Current authority set has been paused."]
            pub struct Paused;
            impl ::subxt::Event for Paused {
                const PALLET: &'static str = "Grandpa";
                const EVENT: &'static str = "Paused";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "Current authority set has been resumed."]
            pub struct Resumed;
            impl ::subxt::Event for Resumed {
                const PALLET: &'static str = "Grandpa";
                const EVENT: &'static str = "Resumed";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct State;
            impl ::subxt::StorageEntry for State {
                const PALLET: &'static str = "Grandpa";
                const STORAGE: &'static str = "State";
                type Value = runtime_types::pallet_grandpa::StoredState<::core::primitive::u32>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct PendingChange;
            impl ::subxt::StorageEntry for PendingChange {
                const PALLET: &'static str = "Grandpa";
                const STORAGE: &'static str = "PendingChange";
                type Value =
                    runtime_types::pallet_grandpa::StoredPendingChange<::core::primitive::u32>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NextForced;
            impl ::subxt::StorageEntry for NextForced {
                const PALLET: &'static str = "Grandpa";
                const STORAGE: &'static str = "NextForced";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Stalled;
            impl ::subxt::StorageEntry for Stalled {
                const PALLET: &'static str = "Grandpa";
                const STORAGE: &'static str = "Stalled";
                type Value = (::core::primitive::u32, ::core::primitive::u32);
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct CurrentSetId;
            impl ::subxt::StorageEntry for CurrentSetId {
                const PALLET: &'static str = "Grandpa";
                const STORAGE: &'static str = "CurrentSetId";
                type Value = ::core::primitive::u64;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct SetIdSession<'a>(pub &'a ::core::primitive::u64);
            impl ::subxt::StorageEntry for SetIdSession<'_> {
                const PALLET: &'static str = "Grandpa";
                const STORAGE: &'static str = "SetIdSession";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                #[doc = " State of the current authority set."]
                pub async fn state(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_grandpa::StoredState<::core::primitive::u32>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<State>()?
                        == [
                            159u8, 75u8, 78u8, 23u8, 98u8, 89u8, 239u8, 230u8, 192u8, 67u8, 139u8,
                            222u8, 151u8, 237u8, 216u8, 20u8, 235u8, 247u8, 180u8, 24u8, 64u8,
                            160u8, 58u8, 15u8, 205u8, 191u8, 120u8, 68u8, 32u8, 5u8, 161u8, 106u8,
                        ]
                    {
                        let entry = State;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " Pending change: (signaled at, scheduled change)."]
                pub async fn pending_change(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_grandpa::StoredPendingChange<::core::primitive::u32>,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<PendingChange>()?
                        == [
                            128u8, 176u8, 209u8, 41u8, 231u8, 111u8, 205u8, 198u8, 154u8, 44u8,
                            228u8, 231u8, 44u8, 110u8, 74u8, 9u8, 31u8, 86u8, 128u8, 244u8, 112u8,
                            21u8, 120u8, 176u8, 50u8, 213u8, 122u8, 46u8, 85u8, 255u8, 40u8, 173u8,
                        ]
                    {
                        let entry = PendingChange;
                        self.client.storage().fetch(&entry, block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " next block number where we can force a change."]
                pub async fn next_forced(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<NextForced>()?
                        == [
                            99u8, 43u8, 245u8, 201u8, 60u8, 9u8, 122u8, 99u8, 188u8, 29u8, 67u8,
                            6u8, 193u8, 133u8, 179u8, 67u8, 202u8, 208u8, 62u8, 179u8, 19u8, 169u8,
                            196u8, 119u8, 107u8, 75u8, 100u8, 3u8, 121u8, 18u8, 80u8, 156u8,
                        ]
                    {
                        let entry = NextForced;
                        self.client.storage().fetch(&entry, block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " `true` if we are currently stalled."]
                pub async fn stalled(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<Stalled>()?
                        == [
                            219u8, 8u8, 37u8, 78u8, 150u8, 55u8, 0u8, 57u8, 201u8, 170u8, 186u8,
                            189u8, 56u8, 161u8, 44u8, 15u8, 53u8, 178u8, 224u8, 208u8, 231u8,
                            109u8, 14u8, 209u8, 57u8, 205u8, 237u8, 153u8, 231u8, 156u8, 24u8,
                            185u8,
                        ]
                    {
                        let entry = Stalled;
                        self.client.storage().fetch(&entry, block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " The number of changes (both in terms of keys and underlying economic responsibilities)"]
                #[doc = " in the \"set\" of Grandpa validators from genesis."]
                pub async fn current_set_id(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    if self.client.metadata().storage_hash::<CurrentSetId>()?
                        == [
                            129u8, 7u8, 62u8, 101u8, 199u8, 60u8, 56u8, 33u8, 54u8, 158u8, 20u8,
                            178u8, 244u8, 145u8, 189u8, 197u8, 157u8, 163u8, 116u8, 36u8, 105u8,
                            52u8, 149u8, 244u8, 108u8, 94u8, 109u8, 111u8, 244u8, 137u8, 7u8,
                            108u8,
                        ]
                    {
                        let entry = CurrentSetId;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " A mapping from grandpa set ID to the index of the *most recent* session for which its"]
                #[doc = " members were responsible."]
                #[doc = ""]
                #[doc = " TWOX-NOTE: `SetId` is not under user control."]
                pub async fn set_id_session(
                    &self,
                    _0: &::core::primitive::u64,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<SetIdSession>()?
                        == [
                            91u8, 175u8, 145u8, 127u8, 242u8, 81u8, 13u8, 231u8, 110u8, 11u8,
                            166u8, 169u8, 103u8, 146u8, 123u8, 133u8, 157u8, 15u8, 33u8, 234u8,
                            108u8, 13u8, 88u8, 115u8, 254u8, 9u8, 145u8, 199u8, 102u8, 47u8, 53u8,
                            134u8,
                        ]
                    {
                        let entry = SetIdSession(_0);
                        self.client.storage().fetch(&entry, block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " A mapping from grandpa set ID to the index of the *most recent* session for which its"]
                #[doc = " members were responsible."]
                #[doc = ""]
                #[doc = " TWOX-NOTE: `SetId` is not under user control."]
                pub async fn set_id_session_iter(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, SetIdSession<'a>>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<SetIdSession>()?
                        == [
                            91u8, 175u8, 145u8, 127u8, 242u8, 81u8, 13u8, 231u8, 110u8, 11u8,
                            166u8, 169u8, 103u8, 146u8, 123u8, 133u8, 157u8, 15u8, 33u8, 234u8,
                            108u8, 13u8, 88u8, 115u8, 254u8, 9u8, 145u8, 199u8, 102u8, 47u8, 53u8,
                            134u8,
                        ]
                    {
                        self.client.storage().iter(block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> ConstantsApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                #[doc = " Max Authorities in use"]
                pub fn max_authorities(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    if self
                        .client
                        .metadata()
                        .constant_hash("Grandpa", "MaxAuthorities")?
                        == [
                            80u8, 201u8, 93u8, 114u8, 100u8, 205u8, 172u8, 38u8, 174u8, 71u8,
                            187u8, 161u8, 148u8, 83u8, 7u8, 73u8, 176u8, 100u8, 128u8, 71u8, 233u8,
                            163u8, 89u8, 171u8, 100u8, 247u8, 111u8, 44u8, 173u8, 82u8, 34u8,
                            159u8,
                        ]
                    {
                        let pallet = self.client.metadata().pallet("Grandpa")?;
                        let constant = pallet.constant("MaxAuthorities")?;
                        let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                        Ok(value)
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
            }
        }
    }
    pub mod balances {
        use super::root_mod;
        use super::runtime_types;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct Transfer {
                pub dest:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                #[codec(compact)]
                pub value: ::core::primitive::u128,
            }
            impl ::subxt::Call for Transfer {
                const PALLET: &'static str = "Balances";
                const FUNCTION: &'static str = "transfer";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct SetBalance {
                pub who:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                #[codec(compact)]
                pub new_free: ::core::primitive::u128,
                #[codec(compact)]
                pub new_reserved: ::core::primitive::u128,
            }
            impl ::subxt::Call for SetBalance {
                const PALLET: &'static str = "Balances";
                const FUNCTION: &'static str = "set_balance";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct ForceTransfer {
                pub source:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub dest:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                #[codec(compact)]
                pub value: ::core::primitive::u128,
            }
            impl ::subxt::Call for ForceTransfer {
                const PALLET: &'static str = "Balances";
                const FUNCTION: &'static str = "force_transfer";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct TransferKeepAlive {
                pub dest:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                #[codec(compact)]
                pub value: ::core::primitive::u128,
            }
            impl ::subxt::Call for TransferKeepAlive {
                const PALLET: &'static str = "Balances";
                const FUNCTION: &'static str = "transfer_keep_alive";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct TransferAll {
                pub dest:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub keep_alive: ::core::primitive::bool,
            }
            impl ::subxt::Call for TransferAll {
                const PALLET: &'static str = "Balances";
                const FUNCTION: &'static str = "transfer_all";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct ForceUnreserve {
                pub who:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Call for ForceUnreserve {
                const PALLET: &'static str = "Balances";
                const FUNCTION: &'static str = "force_unreserve";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<X>,
            }
            impl<'a, T, X> TransactionApi<'a, T, X>
            where
                T: ::subxt::Config,
                X: ::subxt::extrinsic::ExtrinsicParams<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                #[doc = "Transfer some liquid free balance to another account."]
                #[doc = ""]
                #[doc = "`transfer` will set the `FreeBalance` of the sender and receiver."]
                #[doc = "If the sender's account is below the existential deposit as a result"]
                #[doc = "of the transfer, the account will be reaped."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be `Signed` by the transactor."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- Dependent on arguments but not critical, given proper implementations for input config"]
                #[doc = "  types. See related functions below."]
                #[doc = "- It contains a limited number of reads and writes internally and no complex"]
                #[doc = "  computation."]
                #[doc = ""]
                #[doc = "Related functions:"]
                #[doc = ""]
                #[doc = "  - `ensure_can_withdraw` is always called internally but has a bounded complexity."]
                #[doc = "  - Transferring balances to accounts that did not exist before will cause"]
                #[doc = "    `T::OnNewAccount::on_new_account` to be called."]
                #[doc = "  - Removing enough funds from an account will trigger `T::DustRemoval::on_unbalanced`."]
                #[doc = "  - `transfer_keep_alive` works the same way as `transfer`, but has an additional check"]
                #[doc = "    that the transfer will not kill the origin account."]
                #[doc = "---------------------------------"]
                #[doc = "- Origin account is already in memory, so no DB operations for them."]
                #[doc = "# </weight>"]
                pub fn transfer(
                    &self,
                    dest: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    value: ::core::primitive::u128,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        Transfer,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<Transfer>()?
                        == [
                            250u8, 8u8, 164u8, 186u8, 80u8, 220u8, 134u8, 247u8, 142u8, 121u8,
                            34u8, 22u8, 169u8, 39u8, 6u8, 93u8, 72u8, 47u8, 44u8, 107u8, 9u8, 98u8,
                            203u8, 190u8, 136u8, 55u8, 251u8, 78u8, 216u8, 150u8, 98u8, 118u8,
                        ]
                    {
                        let call = Transfer { dest, value };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Set the balances of a given account."]
                #[doc = ""]
                #[doc = "This will alter `FreeBalance` and `ReservedBalance` in storage. it will"]
                #[doc = "also alter the total issuance of the system (`TotalIssuance`) appropriately."]
                #[doc = "If the new free or reserved balance is below the existential deposit,"]
                #[doc = "it will reset the account nonce (`frame_system::AccountNonce`)."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call is `root`."]
                pub fn set_balance(
                    &self,
                    who: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    new_free: ::core::primitive::u128,
                    new_reserved: ::core::primitive::u128,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        SetBalance,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<SetBalance>()?
                        == [
                            232u8, 6u8, 27u8, 131u8, 163u8, 72u8, 148u8, 197u8, 14u8, 239u8, 94u8,
                            1u8, 32u8, 94u8, 17u8, 14u8, 123u8, 82u8, 39u8, 233u8, 77u8, 20u8,
                            40u8, 139u8, 222u8, 137u8, 103u8, 18u8, 126u8, 63u8, 200u8, 149u8,
                        ]
                    {
                        let call = SetBalance {
                            who,
                            new_free,
                            new_reserved,
                        };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Exactly as `transfer`, except the origin must be root and the source account may be"]
                #[doc = "specified."]
                #[doc = "# <weight>"]
                #[doc = "- Same as transfer, but additional read and write because the source account is not"]
                #[doc = "  assumed to be in the overlay."]
                #[doc = "# </weight>"]
                pub fn force_transfer(
                    &self,
                    source: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    dest: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    value: ::core::primitive::u128,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        ForceTransfer,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<ForceTransfer>()?
                        == [
                            120u8, 66u8, 111u8, 84u8, 176u8, 241u8, 214u8, 118u8, 219u8, 75u8,
                            127u8, 222u8, 45u8, 33u8, 204u8, 147u8, 126u8, 214u8, 101u8, 190u8,
                            37u8, 37u8, 159u8, 166u8, 61u8, 143u8, 22u8, 32u8, 15u8, 83u8, 221u8,
                            230u8,
                        ]
                    {
                        let call = ForceTransfer {
                            source,
                            dest,
                            value,
                        };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Same as the [`transfer`] call, but with a check that the transfer will not kill the"]
                #[doc = "origin account."]
                #[doc = ""]
                #[doc = "99% of the time you want [`transfer`] instead."]
                #[doc = ""]
                #[doc = "[`transfer`]: struct.Pallet.html#method.transfer"]
                pub fn transfer_keep_alive(
                    &self,
                    dest: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    value: ::core::primitive::u128,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        TransferKeepAlive,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<TransferKeepAlive>()?
                        == [
                            111u8, 233u8, 125u8, 71u8, 223u8, 141u8, 112u8, 94u8, 157u8, 11u8,
                            88u8, 7u8, 239u8, 145u8, 247u8, 183u8, 245u8, 87u8, 157u8, 35u8, 49u8,
                            91u8, 54u8, 103u8, 101u8, 76u8, 110u8, 94u8, 81u8, 170u8, 153u8, 209u8,
                        ]
                    {
                        let call = TransferKeepAlive { dest, value };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Transfer the entire transferable balance from the caller account."]
                #[doc = ""]
                #[doc = "NOTE: This function only attempts to transfer _transferable_ balances. This means that"]
                #[doc = "any locked, reserved, or existential deposits (when `keep_alive` is `true`), will not be"]
                #[doc = "transferred by this function. To ensure that this function results in a killed account,"]
                #[doc = "you might need to prepare the account by removing any reference counters, storage"]
                #[doc = "deposits, etc..."]
                #[doc = ""]
                #[doc = "The dispatch origin of this call must be Signed."]
                #[doc = ""]
                #[doc = "- `dest`: The recipient of the transfer."]
                #[doc = "- `keep_alive`: A boolean to determine if the `transfer_all` operation should send all"]
                #[doc = "  of the funds the account has, causing the sender account to be killed (false), or"]
                #[doc = "  transfer everything except at least the existential deposit, which will guarantee to"]
                #[doc = "  keep the sender account alive (true). # <weight>"]
                #[doc = "- O(1). Just like transfer, but reading the user's transferable balance first."]
                #[doc = "  #</weight>"]
                pub fn transfer_all(
                    &self,
                    dest: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    keep_alive: ::core::primitive::bool,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        TransferAll,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<TransferAll>()?
                        == [
                            240u8, 165u8, 185u8, 144u8, 24u8, 149u8, 15u8, 46u8, 60u8, 147u8, 19u8,
                            187u8, 96u8, 24u8, 150u8, 53u8, 151u8, 232u8, 200u8, 164u8, 176u8,
                            167u8, 8u8, 23u8, 63u8, 135u8, 68u8, 110u8, 5u8, 21u8, 35u8, 78u8,
                        ]
                    {
                        let call = TransferAll { dest, keep_alive };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Unreserve some balance from a user by force."]
                #[doc = ""]
                #[doc = "Can only be called by ROOT."]
                pub fn force_unreserve(
                    &self,
                    who: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    amount: ::core::primitive::u128,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        ForceUnreserve,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<ForceUnreserve>()?
                        == [
                            106u8, 42u8, 48u8, 136u8, 41u8, 155u8, 214u8, 112u8, 99u8, 122u8,
                            202u8, 250u8, 95u8, 60u8, 182u8, 13u8, 25u8, 149u8, 212u8, 212u8,
                            247u8, 191u8, 130u8, 95u8, 84u8, 252u8, 252u8, 197u8, 244u8, 149u8,
                            103u8, 67u8,
                        ]
                    {
                        let call = ForceUnreserve { who, amount };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
            }
        }
        pub type Event = runtime_types::pallet_balances::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "An account was created with some free balance."]
            pub struct Endowed {
                pub account: ::subxt::sp_core::crypto::AccountId32,
                pub free_balance: ::core::primitive::u128,
            }
            impl ::subxt::Event for Endowed {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Endowed";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "An account was removed whose balance was non-zero but below ExistentialDeposit,"]
            #[doc = "resulting in an outright loss."]
            pub struct DustLost {
                pub account: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Event for DustLost {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "DustLost";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "Transfer succeeded."]
            pub struct Transfer {
                pub from: ::subxt::sp_core::crypto::AccountId32,
                pub to: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Event for Transfer {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Transfer";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "A balance was set by root."]
            pub struct BalanceSet {
                pub who: ::subxt::sp_core::crypto::AccountId32,
                pub free: ::core::primitive::u128,
                pub reserved: ::core::primitive::u128,
            }
            impl ::subxt::Event for BalanceSet {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "BalanceSet";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "Some balance was reserved (moved from free to reserved)."]
            pub struct Reserved {
                pub who: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Event for Reserved {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Reserved";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "Some balance was unreserved (moved from reserved to free)."]
            pub struct Unreserved {
                pub who: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Event for Unreserved {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Unreserved";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "Some balance was moved from the reserve of the first account to the second account."]
            #[doc = "Final argument indicates the destination balance type."]
            pub struct ReserveRepatriated {
                pub from: ::subxt::sp_core::crypto::AccountId32,
                pub to: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
                pub destination_status:
                    runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
            }
            impl ::subxt::Event for ReserveRepatriated {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "ReserveRepatriated";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "Some amount was deposited (e.g. for transaction fees)."]
            pub struct Deposit {
                pub who: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Event for Deposit {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Deposit";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "Some amount was withdrawn from the account (e.g. for transaction fees)."]
            pub struct Withdraw {
                pub who: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Event for Withdraw {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Withdraw";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "Some amount was removed from the account (e.g. for misbehavior)."]
            pub struct Slashed {
                pub who: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Event for Slashed {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Slashed";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct TotalIssuance;
            impl ::subxt::StorageEntry for TotalIssuance {
                const PALLET: &'static str = "Balances";
                const STORAGE: &'static str = "TotalIssuance";
                type Value = ::core::primitive::u128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Account<'a>(pub &'a ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Account<'_> {
                const PALLET: &'static str = "Balances";
                const STORAGE: &'static str = "Account";
                type Value = runtime_types::pallet_balances::AccountData<::core::primitive::u128>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct Locks<'a>(pub &'a ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Locks<'_> {
                const PALLET: &'static str = "Balances";
                const STORAGE: &'static str = "Locks";
                type Value = runtime_types::sp_runtime::bounded::weak_bounded_vec::WeakBoundedVec<
                    runtime_types::pallet_balances::BalanceLock<::core::primitive::u128>,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct Reserves<'a>(pub &'a ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Reserves<'_> {
                const PALLET: &'static str = "Balances";
                const STORAGE: &'static str = "Reserves";
                type Value = runtime_types::sp_runtime::bounded::bounded_vec::BoundedVec<
                    runtime_types::pallet_balances::ReserveData<
                        [::core::primitive::u8; 8usize],
                        ::core::primitive::u128,
                    >,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct StorageVersion;
            impl ::subxt::StorageEntry for StorageVersion {
                const PALLET: &'static str = "Balances";
                const STORAGE: &'static str = "StorageVersion";
                type Value = runtime_types::pallet_balances::Releases;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                #[doc = " The total units issued in the system."]
                pub async fn total_issuance(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    if self.client.metadata().storage_hash::<TotalIssuance>()?
                        == [
                            1u8, 206u8, 252u8, 237u8, 6u8, 30u8, 20u8, 232u8, 164u8, 115u8, 51u8,
                            156u8, 156u8, 206u8, 241u8, 187u8, 44u8, 84u8, 25u8, 164u8, 235u8,
                            20u8, 86u8, 242u8, 124u8, 23u8, 28u8, 140u8, 26u8, 73u8, 231u8, 51u8,
                        ]
                    {
                        let entry = TotalIssuance;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " The Balances pallet example of storing the balance of an account."]
                #[doc = ""]
                #[doc = " # Example"]
                #[doc = ""]
                #[doc = " ```nocompile"]
                #[doc = "  impl pallet_balances::Config for Runtime {"]
                #[doc = "    type AccountStore = StorageMapShim<Self::Account<Runtime>, frame_system::Provider<Runtime>, AccountId, Self::AccountData<Balance>>"]
                #[doc = "  }"]
                #[doc = " ```"]
                #[doc = ""]
                #[doc = " You can also store the balance of an account in the `System` pallet."]
                #[doc = ""]
                #[doc = " # Example"]
                #[doc = ""]
                #[doc = " ```nocompile"]
                #[doc = "  impl pallet_balances::Config for Runtime {"]
                #[doc = "   type AccountStore = System"]
                #[doc = "  }"]
                #[doc = " ```"]
                #[doc = ""]
                #[doc = " But this comes with tradeoffs, storing account balances in the system pallet stores"]
                #[doc = " `frame_system` data alongside the account data contrary to storing account balances in the"]
                #[doc = " `Balances` pallet, which uses a `StorageMap` to store balances data only."]
                #[doc = " NOTE: This is only used in the case that this pallet is used to store balances."]
                pub async fn account(
                    &self,
                    _0: &::subxt::sp_core::crypto::AccountId32,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_balances::AccountData<::core::primitive::u128>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<Account>()?
                        == [
                            129u8, 169u8, 171u8, 206u8, 229u8, 178u8, 69u8, 118u8, 199u8, 64u8,
                            254u8, 67u8, 16u8, 154u8, 160u8, 197u8, 177u8, 161u8, 148u8, 199u8,
                            78u8, 219u8, 187u8, 83u8, 99u8, 110u8, 207u8, 252u8, 243u8, 39u8, 46u8,
                            106u8,
                        ]
                    {
                        let entry = Account(_0);
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " The Balances pallet example of storing the balance of an account."]
                #[doc = ""]
                #[doc = " # Example"]
                #[doc = ""]
                #[doc = " ```nocompile"]
                #[doc = "  impl pallet_balances::Config for Runtime {"]
                #[doc = "    type AccountStore = StorageMapShim<Self::Account<Runtime>, frame_system::Provider<Runtime>, AccountId, Self::AccountData<Balance>>"]
                #[doc = "  }"]
                #[doc = " ```"]
                #[doc = ""]
                #[doc = " You can also store the balance of an account in the `System` pallet."]
                #[doc = ""]
                #[doc = " # Example"]
                #[doc = ""]
                #[doc = " ```nocompile"]
                #[doc = "  impl pallet_balances::Config for Runtime {"]
                #[doc = "   type AccountStore = System"]
                #[doc = "  }"]
                #[doc = " ```"]
                #[doc = ""]
                #[doc = " But this comes with tradeoffs, storing account balances in the system pallet stores"]
                #[doc = " `frame_system` data alongside the account data contrary to storing account balances in the"]
                #[doc = " `Balances` pallet, which uses a `StorageMap` to store balances data only."]
                #[doc = " NOTE: This is only used in the case that this pallet is used to store balances."]
                pub async fn account_iter(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Account<'a>>, ::subxt::BasicError>
                {
                    if self.client.metadata().storage_hash::<Account>()?
                        == [
                            129u8, 169u8, 171u8, 206u8, 229u8, 178u8, 69u8, 118u8, 199u8, 64u8,
                            254u8, 67u8, 16u8, 154u8, 160u8, 197u8, 177u8, 161u8, 148u8, 199u8,
                            78u8, 219u8, 187u8, 83u8, 99u8, 110u8, 207u8, 252u8, 243u8, 39u8, 46u8,
                            106u8,
                        ]
                    {
                        self.client.storage().iter(block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " Any liquidity locks on some account balances."]
                #[doc = " NOTE: Should only be accessed when setting, changing and freeing a lock."]
                pub async fn locks(
                    &self,
                    _0: &::subxt::sp_core::crypto::AccountId32,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::sp_runtime::bounded::weak_bounded_vec::WeakBoundedVec<
                        runtime_types::pallet_balances::BalanceLock<::core::primitive::u128>,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<Locks>()?
                        == [
                            31u8, 76u8, 213u8, 60u8, 86u8, 11u8, 155u8, 151u8, 33u8, 212u8, 74u8,
                            89u8, 174u8, 74u8, 195u8, 107u8, 29u8, 163u8, 178u8, 34u8, 209u8, 8u8,
                            201u8, 237u8, 77u8, 99u8, 205u8, 212u8, 236u8, 132u8, 2u8, 252u8,
                        ]
                    {
                        let entry = Locks(_0);
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " Any liquidity locks on some account balances."]
                #[doc = " NOTE: Should only be accessed when setting, changing and freeing a lock."]
                pub async fn locks_iter(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Locks<'a>>, ::subxt::BasicError>
                {
                    if self.client.metadata().storage_hash::<Locks>()?
                        == [
                            31u8, 76u8, 213u8, 60u8, 86u8, 11u8, 155u8, 151u8, 33u8, 212u8, 74u8,
                            89u8, 174u8, 74u8, 195u8, 107u8, 29u8, 163u8, 178u8, 34u8, 209u8, 8u8,
                            201u8, 237u8, 77u8, 99u8, 205u8, 212u8, 236u8, 132u8, 2u8, 252u8,
                        ]
                    {
                        self.client.storage().iter(block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " Named reserves on some account balances."]
                pub async fn reserves(
                    &self,
                    _0: &::subxt::sp_core::crypto::AccountId32,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::sp_runtime::bounded::bounded_vec::BoundedVec<
                        runtime_types::pallet_balances::ReserveData<
                            [::core::primitive::u8; 8usize],
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<Reserves>()?
                        == [
                            103u8, 6u8, 69u8, 151u8, 81u8, 40u8, 146u8, 113u8, 56u8, 239u8, 104u8,
                            31u8, 168u8, 242u8, 141u8, 121u8, 213u8, 213u8, 114u8, 63u8, 62u8,
                            47u8, 91u8, 119u8, 57u8, 91u8, 95u8, 81u8, 19u8, 208u8, 59u8, 146u8,
                        ]
                    {
                        let entry = Reserves(_0);
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " Named reserves on some account balances."]
                pub async fn reserves_iter(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Reserves<'a>>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<Reserves>()?
                        == [
                            103u8, 6u8, 69u8, 151u8, 81u8, 40u8, 146u8, 113u8, 56u8, 239u8, 104u8,
                            31u8, 168u8, 242u8, 141u8, 121u8, 213u8, 213u8, 114u8, 63u8, 62u8,
                            47u8, 91u8, 119u8, 57u8, 91u8, 95u8, 81u8, 19u8, 208u8, 59u8, 146u8,
                        ]
                    {
                        self.client.storage().iter(block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " Storage version of the pallet."]
                #[doc = ""]
                #[doc = " This is set to v2.0.0 for new networks."]
                pub async fn storage_version(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_balances::Releases,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<StorageVersion>()?
                        == [
                            135u8, 96u8, 28u8, 234u8, 124u8, 212u8, 56u8, 140u8, 40u8, 101u8,
                            235u8, 128u8, 136u8, 221u8, 182u8, 81u8, 17u8, 9u8, 184u8, 228u8,
                            174u8, 165u8, 200u8, 162u8, 214u8, 178u8, 227u8, 72u8, 34u8, 5u8,
                            173u8, 96u8,
                        ]
                    {
                        let entry = StorageVersion;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> ConstantsApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                #[doc = " The minimum amount required to keep an account open."]
                pub fn existential_deposit(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    if self
                        .client
                        .metadata()
                        .constant_hash("Balances", "ExistentialDeposit")?
                        == [
                            206u8, 122u8, 40u8, 104u8, 118u8, 1u8, 145u8, 94u8, 87u8, 187u8, 206u8,
                            187u8, 221u8, 67u8, 67u8, 160u8, 167u8, 24u8, 24u8, 132u8, 126u8, 27u8,
                            134u8, 98u8, 118u8, 35u8, 196u8, 219u8, 25u8, 123u8, 34u8, 37u8,
                        ]
                    {
                        let pallet = self.client.metadata().pallet("Balances")?;
                        let constant = pallet.constant("ExistentialDeposit")?;
                        let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                        Ok(value)
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " The maximum number of locks that should exist on an account."]
                #[doc = " Not strictly enforced, but used for weight estimation."]
                pub fn max_locks(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    if self
                        .client
                        .metadata()
                        .constant_hash("Balances", "MaxLocks")?
                        == [
                            250u8, 58u8, 19u8, 15u8, 35u8, 113u8, 227u8, 89u8, 39u8, 75u8, 21u8,
                            108u8, 202u8, 32u8, 163u8, 167u8, 207u8, 233u8, 69u8, 151u8, 53u8,
                            164u8, 230u8, 16u8, 14u8, 22u8, 172u8, 46u8, 36u8, 216u8, 29u8, 1u8,
                        ]
                    {
                        let pallet = self.client.metadata().pallet("Balances")?;
                        let constant = pallet.constant("MaxLocks")?;
                        let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                        Ok(value)
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = " The maximum number of named reserves that can exist on an account."]
                pub fn max_reserves(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    if self
                        .client
                        .metadata()
                        .constant_hash("Balances", "MaxReserves")?
                        == [
                            95u8, 163u8, 254u8, 186u8, 158u8, 222u8, 45u8, 163u8, 130u8, 111u8,
                            59u8, 232u8, 163u8, 210u8, 243u8, 112u8, 38u8, 103u8, 252u8, 120u8,
                            141u8, 104u8, 20u8, 200u8, 128u8, 65u8, 56u8, 145u8, 247u8, 95u8, 82u8,
                            42u8,
                        ]
                    {
                        let pallet = self.client.metadata().pallet("Balances")?;
                        let constant = pallet.constant("MaxReserves")?;
                        let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                        Ok(value)
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
            }
        }
    }
    pub mod transaction_payment {
        use super::root_mod;
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub struct NextFeeMultiplier;
            impl ::subxt::StorageEntry for NextFeeMultiplier {
                const PALLET: &'static str = "TransactionPayment";
                const STORAGE: &'static str = "NextFeeMultiplier";
                type Value = runtime_types::sp_arithmetic::fixed_point::FixedU128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageVersion;
            impl ::subxt::StorageEntry for StorageVersion {
                const PALLET: &'static str = "TransactionPayment";
                const STORAGE: &'static str = "StorageVersion";
                type Value = runtime_types::pallet_transaction_payment::Releases;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn next_fee_multiplier(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::sp_arithmetic::fixed_point::FixedU128,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<NextFeeMultiplier>()?
                        == [
                            232u8, 48u8, 68u8, 202u8, 209u8, 29u8, 249u8, 71u8, 0u8, 84u8, 229u8,
                            250u8, 176u8, 203u8, 27u8, 26u8, 34u8, 55u8, 83u8, 183u8, 224u8, 40u8,
                            62u8, 127u8, 131u8, 88u8, 128u8, 9u8, 56u8, 178u8, 31u8, 183u8,
                        ]
                    {
                        let entry = NextFeeMultiplier;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                pub async fn storage_version(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_transaction_payment::Releases,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<StorageVersion>()?
                        == [
                            219u8, 243u8, 82u8, 176u8, 65u8, 5u8, 132u8, 114u8, 8u8, 82u8, 176u8,
                            200u8, 97u8, 150u8, 177u8, 164u8, 166u8, 11u8, 34u8, 12u8, 12u8, 198u8,
                            58u8, 191u8, 186u8, 221u8, 221u8, 119u8, 181u8, 253u8, 154u8, 228u8,
                        ]
                    {
                        let entry = StorageVersion;
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> ConstantsApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                #[doc = " A fee mulitplier for `Operational` extrinsics to compute \"virtual tip\" to boost their"]
                #[doc = " `priority`"]
                #[doc = ""]
                #[doc = " This value is multipled by the `final_fee` to obtain a \"virtual tip\" that is later"]
                #[doc = " added to a tip component in regular `priority` calculations."]
                #[doc = " It means that a `Normal` transaction can front-run a similarly-sized `Operational`"]
                #[doc = " extrinsic (with no tip), by including a tip value greater than the virtual tip."]
                #[doc = ""]
                #[doc = " ```rust,ignore"]
                #[doc = " // For `Normal`"]
                #[doc = " let priority = priority_calc(tip);"]
                #[doc = ""]
                #[doc = " // For `Operational`"]
                #[doc = " let virtual_tip = (inclusion_fee + tip) * OperationalFeeMultiplier;"]
                #[doc = " let priority = priority_calc(tip + virtual_tip);"]
                #[doc = " ```"]
                #[doc = ""]
                #[doc = " Note that since we use `final_fee` the multiplier applies also to the regular `tip`"]
                #[doc = " sent with the transaction. So, not only does the transaction get a priority bump based"]
                #[doc = " on the `inclusion_fee`, but we also amplify the impact of tips applied to `Operational`"]
                #[doc = " transactions."]
                pub fn operational_fee_multiplier(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u8, ::subxt::BasicError>
                {
                    if self
                        .client
                        .metadata()
                        .constant_hash("TransactionPayment", "OperationalFeeMultiplier")?
                        == [
                            161u8, 232u8, 150u8, 43u8, 106u8, 83u8, 56u8, 248u8, 54u8, 123u8,
                            244u8, 73u8, 5u8, 49u8, 245u8, 150u8, 70u8, 92u8, 158u8, 207u8, 127u8,
                            115u8, 211u8, 21u8, 24u8, 136u8, 89u8, 44u8, 151u8, 211u8, 235u8,
                            196u8,
                        ]
                    {
                        let pallet = self.client.metadata().pallet("TransactionPayment")?;
                        let constant = pallet.constant("OperationalFeeMultiplier")?;
                        let value = ::subxt::codec::Decode::decode(&mut &constant.value[..])?;
                        Ok(value)
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
            }
        }
    }
    pub mod sudo {
        use super::root_mod;
        use super::runtime_types;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct Sudo {
                pub call: ::std::boxed::Box<runtime_types::node_template_runtime::Call>,
            }
            impl ::subxt::Call for Sudo {
                const PALLET: &'static str = "Sudo";
                const FUNCTION: &'static str = "sudo";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct SudoUncheckedWeight {
                pub call: ::std::boxed::Box<runtime_types::node_template_runtime::Call>,
                pub weight: ::core::primitive::u64,
            }
            impl ::subxt::Call for SudoUncheckedWeight {
                const PALLET: &'static str = "Sudo";
                const FUNCTION: &'static str = "sudo_unchecked_weight";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct SetKey {
                pub new:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
            }
            impl ::subxt::Call for SetKey {
                const PALLET: &'static str = "Sudo";
                const FUNCTION: &'static str = "set_key";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct SudoAs {
                pub who:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub call: ::std::boxed::Box<runtime_types::node_template_runtime::Call>,
            }
            impl ::subxt::Call for SudoAs {
                const PALLET: &'static str = "Sudo";
                const FUNCTION: &'static str = "sudo_as";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<X>,
            }
            impl<'a, T, X> TransactionApi<'a, T, X>
            where
                T: ::subxt::Config,
                X: ::subxt::extrinsic::ExtrinsicParams<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                #[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- O(1)."]
                #[doc = "- Limited storage reads."]
                #[doc = "- One DB write (event)."]
                #[doc = "- Weight of derivative `call` execution + 10,000."]
                #[doc = "# </weight>"]
                pub fn sudo(
                    &self,
                    call: runtime_types::node_template_runtime::Call,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<'a, T, X, Sudo, DispatchError, root_mod::Event>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<Sudo>()?
                        == [
                            124u8, 165u8, 255u8, 15u8, 71u8, 26u8, 3u8, 41u8, 204u8, 164u8, 65u8,
                            23u8, 233u8, 29u8, 8u8, 252u8, 241u8, 217u8, 103u8, 22u8, 166u8, 66u8,
                            211u8, 30u8, 149u8, 52u8, 136u8, 193u8, 186u8, 243u8, 70u8, 147u8,
                        ]
                    {
                        let call = Sudo {
                            call: ::std::boxed::Box::new(call),
                        };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
                #[doc = "This function does not check the weight of the call, and instead allows the"]
                #[doc = "Sudo user to specify the weight of the call."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- O(1)."]
                #[doc = "- The weight of this call is defined by the caller."]
                #[doc = "# </weight>"]
                pub fn sudo_unchecked_weight(
                    &self,
                    call: runtime_types::node_template_runtime::Call,
                    weight: ::core::primitive::u64,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        SudoUncheckedWeight,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<SudoUncheckedWeight>()?
                        == [
                            2u8, 174u8, 230u8, 249u8, 224u8, 24u8, 143u8, 100u8, 132u8, 64u8, 62u8,
                            140u8, 6u8, 131u8, 205u8, 202u8, 119u8, 199u8, 65u8, 21u8, 6u8, 140u8,
                            129u8, 18u8, 120u8, 245u8, 65u8, 188u8, 16u8, 67u8, 167u8, 172u8,
                        ]
                    {
                        let call = SudoUncheckedWeight {
                            call: ::std::boxed::Box::new(call),
                            weight,
                        };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Authenticates the current sudo key and sets the given AccountId (`new`) as the new sudo"]
                #[doc = "key."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- O(1)."]
                #[doc = "- Limited storage reads."]
                #[doc = "- One DB change."]
                #[doc = "# </weight>"]
                pub fn set_key(
                    &self,
                    new: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<'a, T, X, SetKey, DispatchError, root_mod::Event>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<SetKey>()?
                        == [
                            77u8, 253u8, 211u8, 157u8, 74u8, 92u8, 1u8, 102u8, 178u8, 103u8, 126u8,
                            56u8, 156u8, 105u8, 45u8, 44u8, 64u8, 154u8, 163u8, 102u8, 93u8, 93u8,
                            212u8, 5u8, 148u8, 184u8, 22u8, 135u8, 110u8, 102u8, 44u8, 172u8,
                        ]
                    {
                        let call = SetKey { new };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "Authenticates the sudo key and dispatches a function call with `Signed` origin from"]
                #[doc = "a given account."]
                #[doc = ""]
                #[doc = "The dispatch origin for this call must be _Signed_."]
                #[doc = ""]
                #[doc = "# <weight>"]
                #[doc = "- O(1)."]
                #[doc = "- Limited storage reads."]
                #[doc = "- One DB write (event)."]
                #[doc = "- Weight of derivative `call` execution + 10,000."]
                #[doc = "# </weight>"]
                pub fn sudo_as(
                    &self,
                    who: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    call: runtime_types::node_template_runtime::Call,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<'a, T, X, SudoAs, DispatchError, root_mod::Event>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<SudoAs>()?
                        == [
                            219u8, 14u8, 89u8, 145u8, 131u8, 111u8, 249u8, 238u8, 233u8, 103u8,
                            55u8, 138u8, 41u8, 228u8, 185u8, 7u8, 123u8, 77u8, 225u8, 193u8, 81u8,
                            25u8, 9u8, 255u8, 212u8, 239u8, 165u8, 94u8, 131u8, 166u8, 179u8,
                            196u8,
                        ]
                    {
                        let call = SudoAs {
                            who,
                            call: ::std::boxed::Box::new(call),
                        };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
            }
        }
        pub type Event = runtime_types::pallet_sudo::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "A sudo just took place. \\[result\\]"]
            pub struct Sudid {
                pub sudo_result:
                    ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
            }
            impl ::subxt::Event for Sudid {
                const PALLET: &'static str = "Sudo";
                const EVENT: &'static str = "Sudid";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "The \\[sudoer\\] just switched identity; the old key is supplied if one existed."]
            pub struct KeyChanged {
                pub old_sudoer: ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
            }
            impl ::subxt::Event for KeyChanged {
                const PALLET: &'static str = "Sudo";
                const EVENT: &'static str = "KeyChanged";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "A sudo just took place. \\[result\\]"]
            pub struct SudoAsDone {
                pub sudo_result:
                    ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
            }
            impl ::subxt::Event for SudoAsDone {
                const PALLET: &'static str = "Sudo";
                const EVENT: &'static str = "SudoAsDone";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Key;
            impl ::subxt::StorageEntry for Key {
                const PALLET: &'static str = "Sudo";
                const STORAGE: &'static str = "Key";
                type Value = ::subxt::sp_core::crypto::AccountId32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                #[doc = " The `AccountId` of the sudo key."]
                pub async fn key(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<Key>()?
                        == [
                            222u8, 90u8, 158u8, 233u8, 184u8, 23u8, 141u8, 135u8, 81u8, 187u8,
                            47u8, 100u8, 30u8, 81u8, 239u8, 197u8, 249u8, 253u8, 73u8, 207u8,
                            161u8, 141u8, 174u8, 59u8, 74u8, 181u8, 10u8, 90u8, 22u8, 109u8, 62u8,
                            27u8,
                        ]
                    {
                        let entry = Key;
                        self.client.storage().fetch(&entry, block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
            }
        }
    }
    pub mod template_module {
        use super::root_mod;
        use super::runtime_types;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: codec :: CompactAs,
                :: subxt :: codec :: Decode,
                :: subxt :: codec :: Encode,
                Debug,
            )]
            pub struct DoSomething {
                pub something: ::core::primitive::u32,
            }
            impl ::subxt::Call for DoSomething {
                const PALLET: &'static str = "TemplateModule";
                const FUNCTION: &'static str = "do_something";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct CauseError;
            impl ::subxt::Call for CauseError {
                const PALLET: &'static str = "TemplateModule";
                const FUNCTION: &'static str = "cause_error";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<X>,
            }
            impl<'a, T, X> TransactionApi<'a, T, X>
            where
                T: ::subxt::Config,
                X: ::subxt::extrinsic::ExtrinsicParams<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                #[doc = "An example dispatchable that takes a singles value as a parameter, writes the value to"]
                #[doc = "storage and emits an event. This function must be dispatched by a signed extrinsic."]
                pub fn do_something(
                    &self,
                    something: ::core::primitive::u32,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        DoSomething,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<DoSomething>()?
                        == [
                            118u8, 135u8, 114u8, 170u8, 240u8, 97u8, 215u8, 4u8, 153u8, 184u8,
                            191u8, 161u8, 106u8, 187u8, 143u8, 60u8, 254u8, 16u8, 244u8, 150u8,
                            224u8, 18u8, 74u8, 157u8, 89u8, 229u8, 39u8, 88u8, 157u8, 140u8, 184u8,
                            51u8,
                        ]
                    {
                        let call = DoSomething { something };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "An example dispatchable that may throw a custom error."]
                pub fn cause_error(
                    &self,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        CauseError,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<CauseError>()?
                        == [
                            130u8, 169u8, 2u8, 174u8, 179u8, 44u8, 35u8, 37u8, 1u8, 110u8, 218u8,
                            248u8, 55u8, 69u8, 235u8, 186u8, 204u8, 99u8, 5u8, 33u8, 20u8, 68u8,
                            138u8, 254u8, 126u8, 2u8, 199u8, 51u8, 194u8, 6u8, 53u8, 243u8,
                        ]
                    {
                        let call = CauseError {};
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
            }
        }
        pub type Event = runtime_types::pallet_template::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "Event documentation should end with an array that provides descriptive names for event"]
            #[doc = "parameters. [something, who]"]
            pub struct SomethingStored(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for SomethingStored {
                const PALLET: &'static str = "TemplateModule";
                const EVENT: &'static str = "SomethingStored";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Something;
            impl ::subxt::StorageEntry for Something {
                const PALLET: &'static str = "TemplateModule";
                const STORAGE: &'static str = "Something";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn something(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<Something>()?
                        == [
                            133u8, 15u8, 68u8, 243u8, 54u8, 252u8, 134u8, 227u8, 161u8, 207u8,
                            221u8, 164u8, 33u8, 75u8, 249u8, 148u8, 184u8, 225u8, 102u8, 168u8,
                            104u8, 72u8, 191u8, 148u8, 253u8, 173u8, 33u8, 89u8, 108u8, 218u8,
                            70u8, 179u8,
                        ]
                    {
                        let entry = Something;
                        self.client.storage().fetch(&entry, block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
            }
        }
    }
    pub mod ibc {
        use super::root_mod;
        use super::runtime_types;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(
                :: subxt :: codec :: CompactAs,
                :: subxt :: codec :: Decode,
                :: subxt :: codec :: Encode,
                Debug,
            )]
            pub struct DoSomething {
                pub something: ::core::primitive::u32,
            }
            impl ::subxt::Call for DoSomething {
                const PALLET: &'static str = "Ibc";
                const FUNCTION: &'static str = "do_something";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct CauseError;
            impl ::subxt::Call for CauseError {
                const PALLET: &'static str = "Ibc";
                const FUNCTION: &'static str = "cause_error";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<X>,
            }
            impl<'a, T, X> TransactionApi<'a, T, X>
            where
                T: ::subxt::Config,
                X: ::subxt::extrinsic::ExtrinsicParams<T>,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                #[doc = "An example dispatchable that takes a singles value as a parameter, writes the value to"]
                #[doc = "storage and emits an event. This function must be dispatched by a signed extrinsic."]
                pub fn do_something(
                    &self,
                    something: ::core::primitive::u32,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        DoSomething,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<DoSomething>()?
                        == [
                            118u8, 135u8, 114u8, 170u8, 240u8, 97u8, 215u8, 4u8, 153u8, 184u8,
                            191u8, 161u8, 106u8, 187u8, 143u8, 60u8, 254u8, 16u8, 244u8, 150u8,
                            224u8, 18u8, 74u8, 157u8, 89u8, 229u8, 39u8, 88u8, 157u8, 140u8, 184u8,
                            51u8,
                        ]
                    {
                        let call = DoSomething { something };
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                #[doc = "An example dispatchable that may throw a custom error."]
                pub fn cause_error(
                    &self,
                ) -> Result<
                    ::subxt::SubmittableExtrinsic<
                        'a,
                        T,
                        X,
                        CauseError,
                        DispatchError,
                        root_mod::Event,
                    >,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().call_hash::<CauseError>()?
                        == [
                            130u8, 169u8, 2u8, 174u8, 179u8, 44u8, 35u8, 37u8, 1u8, 110u8, 218u8,
                            248u8, 55u8, 69u8, 235u8, 186u8, 204u8, 99u8, 5u8, 33u8, 20u8, 68u8,
                            138u8, 254u8, 126u8, 2u8, 199u8, 51u8, 194u8, 6u8, 53u8, 243u8,
                        ]
                    {
                        let call = CauseError {};
                        Ok(::subxt::SubmittableExtrinsic::new(self.client, call))
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
            }
        }
        pub type Event = runtime_types::pallet_ibc::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "Event documentation should end with an array that provides descriptive names for event"]
            #[doc = "parameters. [something, who]"]
            pub struct SomethingStored(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for SomethingStored {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "SomethingStored";
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            #[doc = "Store map key, value and who"]
            pub struct SomeMapStored(
                pub ::core::primitive::u32,
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for SomeMapStored {
                const PALLET: &'static str = "Ibc";
                const EVENT: &'static str = "SomeMapStored";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Something;
            impl ::subxt::StorageEntry for Something {
                const PALLET: &'static str = "Ibc";
                const STORAGE: &'static str = "Something";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct SomeMap<'a>(pub &'a ::core::primitive::u32);
            impl ::subxt::StorageEntry for SomeMap<'_> {
                const PALLET: &'static str = "Ibc";
                const STORAGE: &'static str = "SomeMap";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn something(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::BasicError,
                > {
                    if self.client.metadata().storage_hash::<Something>()?
                        == [
                            133u8, 15u8, 68u8, 243u8, 54u8, 252u8, 134u8, 227u8, 161u8, 207u8,
                            221u8, 164u8, 33u8, 75u8, 249u8, 148u8, 184u8, 225u8, 102u8, 168u8,
                            104u8, 72u8, 191u8, 148u8, 253u8, 173u8, 33u8, 89u8, 108u8, 218u8,
                            70u8, 179u8,
                        ]
                    {
                        let entry = Something;
                        self.client.storage().fetch(&entry, block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                pub async fn some_map(
                    &self,
                    _0: &::core::primitive::u32,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    if self.client.metadata().storage_hash::<SomeMap>()?
                        == [
                            215u8, 216u8, 69u8, 77u8, 132u8, 39u8, 30u8, 117u8, 4u8, 8u8, 64u8,
                            189u8, 7u8, 77u8, 146u8, 215u8, 205u8, 136u8, 76u8, 246u8, 230u8,
                            141u8, 25u8, 58u8, 53u8, 23u8, 107u8, 88u8, 22u8, 31u8, 40u8, 121u8,
                        ]
                    {
                        let entry = SomeMap(_0);
                        self.client
                            .storage()
                            .fetch_or_default(&entry, block_hash)
                            .await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
                pub async fn some_map_iter(
                    &self,
                    block_hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, SomeMap<'a>>, ::subxt::BasicError>
                {
                    if self.client.metadata().storage_hash::<SomeMap>()?
                        == [
                            215u8, 216u8, 69u8, 77u8, 132u8, 39u8, 30u8, 117u8, 4u8, 8u8, 64u8,
                            189u8, 7u8, 77u8, 146u8, 215u8, 205u8, 136u8, 76u8, 246u8, 230u8,
                            141u8, 25u8, 58u8, 53u8, 23u8, 107u8, 88u8, 22u8, 31u8, 40u8, 121u8,
                        ]
                    {
                        self.client.storage().iter(block_hash).await
                    } else {
                        Err(::subxt::MetadataError::IncompatibleMetadata.into())
                    }
                }
            }
        }
    }
    pub mod runtime_types {
        use super::runtime_types;
        pub mod finality_grandpa {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct Equivocation<_0, _1, _2> {
                pub round_number: ::core::primitive::u64,
                pub identity: _0,
                pub first: (_1, _2),
                pub second: (_1, _2),
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct Precommit<_0, _1> {
                pub target_hash: _0,
                pub target_number: _1,
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct Prevote<_0, _1> {
                pub target_hash: _0,
                pub target_number: _1,
            }
        }
        pub mod frame_support {
            use super::runtime_types;
            pub mod traits {
                use super::runtime_types;
                pub mod tokens {
                    use super::runtime_types;
                    pub mod misc {
                        use super::runtime_types;
                        #[derive(
                            :: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug,
                        )]
                        pub enum BalanceStatus {
                            #[codec(index = 0)]
                            Free,
                            #[codec(index = 1)]
                            Reserved,
                        }
                    }
                }
            }
            pub mod weights {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub enum DispatchClass {
                    #[codec(index = 0)]
                    Normal,
                    #[codec(index = 1)]
                    Operational,
                    #[codec(index = 2)]
                    Mandatory,
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub struct DispatchInfo {
                    pub weight: ::core::primitive::u64,
                    pub class: runtime_types::frame_support::weights::DispatchClass,
                    pub pays_fee: runtime_types::frame_support::weights::Pays,
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub enum Pays {
                    #[codec(index = 0)]
                    Yes,
                    #[codec(index = 1)]
                    No,
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub struct PerDispatchClass<_0> {
                    pub normal: _0,
                    pub operational: _0,
                    pub mandatory: _0,
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub struct RuntimeDbWeight {
                    pub read: ::core::primitive::u64,
                    pub write: ::core::primitive::u64,
                }
            }
        }
        pub mod frame_system {
            use super::runtime_types;
            pub mod extensions {
                use super::runtime_types;
                pub mod check_genesis {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                    pub struct CheckGenesis;
                }
                pub mod check_mortality {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                    pub struct CheckMortality(pub runtime_types::sp_runtime::generic::era::Era);
                }
                pub mod check_non_zero_sender {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                    pub struct CheckNonZeroSender;
                }
                pub mod check_nonce {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                    pub struct CheckNonce(#[codec(compact)] pub ::core::primitive::u32);
                }
                pub mod check_spec_version {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                    pub struct CheckSpecVersion;
                }
                pub mod check_tx_version {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                    pub struct CheckTxVersion;
                }
                pub mod check_weight {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                    pub struct CheckWeight;
                }
            }
            pub mod limits {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub struct BlockLength {
                    pub max: runtime_types::frame_support::weights::PerDispatchClass<
                        ::core::primitive::u32,
                    >,
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub struct BlockWeights {
                    pub base_block: ::core::primitive::u64,
                    pub max_block: ::core::primitive::u64,
                    pub per_class: runtime_types::frame_support::weights::PerDispatchClass<
                        runtime_types::frame_system::limits::WeightsPerClass,
                    >,
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub struct WeightsPerClass {
                    pub base_extrinsic: ::core::primitive::u64,
                    pub max_extrinsic: ::core::option::Option<::core::primitive::u64>,
                    pub max_total: ::core::option::Option<::core::primitive::u64>,
                    pub reserved: ::core::option::Option<::core::primitive::u64>,
                }
            }
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "A dispatch that will fill the block weight up to the given ratio."]
                    fill_block {
                        ratio: runtime_types::sp_arithmetic::per_things::Perbill,
                    },
                    #[codec(index = 1)]
                    #[doc = "Make some on-chain remark."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- `O(1)`"]
                    #[doc = "# </weight>"]
                    remark {
                        remark: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 2)]
                    #[doc = "Set the number of pages in the WebAssembly environment's heap."]
                    set_heap_pages { pages: ::core::primitive::u64 },
                    #[codec(index = 3)]
                    #[doc = "Set the new runtime code."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- `O(C + S)` where `C` length of `code` and `S` complexity of `can_set_code`"]
                    #[doc = "- 1 call to `can_set_code`: `O(S)` (calls `sp_io::misc::runtime_version` which is"]
                    #[doc = "  expensive)."]
                    #[doc = "- 1 storage write (codec `O(C)`)."]
                    #[doc = "- 1 digest item."]
                    #[doc = "- 1 event."]
                    #[doc = "The weight of this function is dependent on the runtime, but generally this is very"]
                    #[doc = "expensive. We will treat this as a full block."]
                    #[doc = "# </weight>"]
                    set_code {
                        code: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 4)]
                    #[doc = "Set the new runtime code without doing any checks of the given `code`."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- `O(C)` where `C` length of `code`"]
                    #[doc = "- 1 storage write (codec `O(C)`)."]
                    #[doc = "- 1 digest item."]
                    #[doc = "- 1 event."]
                    #[doc = "The weight of this function is dependent on the runtime. We will treat this as a full"]
                    #[doc = "block. # </weight>"]
                    set_code_without_checks {
                        code: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 5)]
                    #[doc = "Set some items of storage."]
                    set_storage {
                        items: ::std::vec::Vec<(
                            ::std::vec::Vec<::core::primitive::u8>,
                            ::std::vec::Vec<::core::primitive::u8>,
                        )>,
                    },
                    #[codec(index = 6)]
                    #[doc = "Kill some items from storage."]
                    kill_storage {
                        keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                    },
                    #[codec(index = 7)]
                    #[doc = "Kill all storage items with a key that starts with the given prefix."]
                    #[doc = ""]
                    #[doc = "**NOTE:** We rely on the Root origin to provide us the number of subkeys under"]
                    #[doc = "the prefix we are removing to accurately calculate the weight of this function."]
                    kill_prefix {
                        prefix: ::std::vec::Vec<::core::primitive::u8>,
                        subkeys: ::core::primitive::u32,
                    },
                    #[codec(index = 8)]
                    #[doc = "Make some on-chain remark and emit event."]
                    remark_with_event {
                        remark: ::std::vec::Vec<::core::primitive::u8>,
                    },
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "The name of specification does not match between the current runtime"]
                    #[doc = "and the new runtime."]
                    InvalidSpecName,
                    #[codec(index = 1)]
                    #[doc = "The specification version is not allowed to decrease between the current runtime"]
                    #[doc = "and the new runtime."]
                    SpecVersionNeedsToIncrease,
                    #[codec(index = 2)]
                    #[doc = "Failed to extract the runtime version from the new runtime."]
                    #[doc = ""]
                    #[doc = "Either calling `Core_version` or decoding `RuntimeVersion` failed."]
                    FailedToExtractRuntimeVersion,
                    #[codec(index = 3)]
                    #[doc = "Suicide called when the account has non-default composite data."]
                    NonDefaultComposite,
                    #[codec(index = 4)]
                    #[doc = "There is a non-zero reference count preventing the account from being purged."]
                    NonZeroRefCount,
                    #[codec(index = 5)]
                    #[doc = "The origin filter prevent the call to be dispatched."]
                    CallFiltered,
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "An extrinsic completed successfully."]
                    ExtrinsicSuccess {
                        dispatch_info: runtime_types::frame_support::weights::DispatchInfo,
                    },
                    #[codec(index = 1)]
                    #[doc = "An extrinsic failed."]
                    ExtrinsicFailed {
                        dispatch_error: runtime_types::sp_runtime::DispatchError,
                        dispatch_info: runtime_types::frame_support::weights::DispatchInfo,
                    },
                    #[codec(index = 2)]
                    #[doc = "`:code` was updated."]
                    CodeUpdated,
                    #[codec(index = 3)]
                    #[doc = "A new account was created."]
                    NewAccount {
                        account: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 4)]
                    #[doc = "An account was reaped."]
                    KilledAccount {
                        account: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 5)]
                    #[doc = "On on-chain remark happened."]
                    Remarked {
                        sender: ::subxt::sp_core::crypto::AccountId32,
                        hash: ::subxt::sp_core::H256,
                    },
                }
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct AccountInfo<_0, _1> {
                pub nonce: _0,
                pub consumers: _0,
                pub providers: _0,
                pub sufficients: _0,
                pub data: _1,
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct EventRecord<_0, _1> {
                pub phase: runtime_types::frame_system::Phase,
                pub event: _0,
                pub topics: ::std::vec::Vec<_1>,
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct LastRuntimeUpgradeInfo {
                #[codec(compact)]
                pub spec_version: ::core::primitive::u32,
                pub spec_name: ::std::string::String,
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub enum Phase {
                #[codec(index = 0)]
                ApplyExtrinsic(::core::primitive::u32),
                #[codec(index = 1)]
                Finalization,
                #[codec(index = 2)]
                Initialization,
            }
        }
        pub mod node_template_runtime {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub enum Call {
                #[codec(index = 0)]
                System(runtime_types::frame_system::pallet::Call),
                #[codec(index = 2)]
                Timestamp(runtime_types::pallet_timestamp::pallet::Call),
                #[codec(index = 4)]
                Grandpa(runtime_types::pallet_grandpa::pallet::Call),
                #[codec(index = 5)]
                Balances(runtime_types::pallet_balances::pallet::Call),
                #[codec(index = 7)]
                Sudo(runtime_types::pallet_sudo::pallet::Call),
                #[codec(index = 8)]
                TemplateModule(runtime_types::pallet_template::pallet::Call),
                #[codec(index = 9)]
                Ibc(runtime_types::pallet_ibc::pallet::Call),
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub enum Event {
                #[codec(index = 0)]
                System(runtime_types::frame_system::pallet::Event),
                #[codec(index = 4)]
                Grandpa(runtime_types::pallet_grandpa::pallet::Event),
                #[codec(index = 5)]
                Balances(runtime_types::pallet_balances::pallet::Event),
                #[codec(index = 7)]
                Sudo(runtime_types::pallet_sudo::pallet::Event),
                #[codec(index = 8)]
                TemplateModule(runtime_types::pallet_template::pallet::Event),
                #[codec(index = 9)]
                Ibc(runtime_types::pallet_ibc::pallet::Event),
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct Runtime;
        }
        pub mod pallet_balances {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Transfer some liquid free balance to another account."]
                    #[doc = ""]
                    #[doc = "`transfer` will set the `FreeBalance` of the sender and receiver."]
                    #[doc = "If the sender's account is below the existential deposit as a result"]
                    #[doc = "of the transfer, the account will be reaped."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be `Signed` by the transactor."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- Dependent on arguments but not critical, given proper implementations for input config"]
                    #[doc = "  types. See related functions below."]
                    #[doc = "- It contains a limited number of reads and writes internally and no complex"]
                    #[doc = "  computation."]
                    #[doc = ""]
                    #[doc = "Related functions:"]
                    #[doc = ""]
                    #[doc = "  - `ensure_can_withdraw` is always called internally but has a bounded complexity."]
                    #[doc = "  - Transferring balances to accounts that did not exist before will cause"]
                    #[doc = "    `T::OnNewAccount::on_new_account` to be called."]
                    #[doc = "  - Removing enough funds from an account will trigger `T::DustRemoval::on_unbalanced`."]
                    #[doc = "  - `transfer_keep_alive` works the same way as `transfer`, but has an additional check"]
                    #[doc = "    that the transfer will not kill the origin account."]
                    #[doc = "---------------------------------"]
                    #[doc = "- Origin account is already in memory, so no DB operations for them."]
                    #[doc = "# </weight>"]
                    transfer {
                        dest: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 1)]
                    #[doc = "Set the balances of a given account."]
                    #[doc = ""]
                    #[doc = "This will alter `FreeBalance` and `ReservedBalance` in storage. it will"]
                    #[doc = "also alter the total issuance of the system (`TotalIssuance`) appropriately."]
                    #[doc = "If the new free or reserved balance is below the existential deposit,"]
                    #[doc = "it will reset the account nonce (`frame_system::AccountNonce`)."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call is `root`."]
                    set_balance {
                        who: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        new_free: ::core::primitive::u128,
                        #[codec(compact)]
                        new_reserved: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    #[doc = "Exactly as `transfer`, except the origin must be root and the source account may be"]
                    #[doc = "specified."]
                    #[doc = "# <weight>"]
                    #[doc = "- Same as transfer, but additional read and write because the source account is not"]
                    #[doc = "  assumed to be in the overlay."]
                    #[doc = "# </weight>"]
                    force_transfer {
                        source: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        dest: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    #[doc = "Same as the [`transfer`] call, but with a check that the transfer will not kill the"]
                    #[doc = "origin account."]
                    #[doc = ""]
                    #[doc = "99% of the time you want [`transfer`] instead."]
                    #[doc = ""]
                    #[doc = "[`transfer`]: struct.Pallet.html#method.transfer"]
                    transfer_keep_alive {
                        dest: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 4)]
                    #[doc = "Transfer the entire transferable balance from the caller account."]
                    #[doc = ""]
                    #[doc = "NOTE: This function only attempts to transfer _transferable_ balances. This means that"]
                    #[doc = "any locked, reserved, or existential deposits (when `keep_alive` is `true`), will not be"]
                    #[doc = "transferred by this function. To ensure that this function results in a killed account,"]
                    #[doc = "you might need to prepare the account by removing any reference counters, storage"]
                    #[doc = "deposits, etc..."]
                    #[doc = ""]
                    #[doc = "The dispatch origin of this call must be Signed."]
                    #[doc = ""]
                    #[doc = "- `dest`: The recipient of the transfer."]
                    #[doc = "- `keep_alive`: A boolean to determine if the `transfer_all` operation should send all"]
                    #[doc = "  of the funds the account has, causing the sender account to be killed (false), or"]
                    #[doc = "  transfer everything except at least the existential deposit, which will guarantee to"]
                    #[doc = "  keep the sender account alive (true). # <weight>"]
                    #[doc = "- O(1). Just like transfer, but reading the user's transferable balance first."]
                    #[doc = "  #</weight>"]
                    transfer_all {
                        dest: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        keep_alive: ::core::primitive::bool,
                    },
                    #[codec(index = 5)]
                    #[doc = "Unreserve some balance from a user by force."]
                    #[doc = ""]
                    #[doc = "Can only be called by ROOT."]
                    force_unreserve {
                        who: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        amount: ::core::primitive::u128,
                    },
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Vesting balance too high to send value"]
                    VestingBalance,
                    #[codec(index = 1)]
                    #[doc = "Account liquidity restrictions prevent withdrawal"]
                    LiquidityRestrictions,
                    #[codec(index = 2)]
                    #[doc = "Balance too low to send value"]
                    InsufficientBalance,
                    #[codec(index = 3)]
                    #[doc = "Value too low to create account due to existential deposit"]
                    ExistentialDeposit,
                    #[codec(index = 4)]
                    #[doc = "Transfer/payment would kill account"]
                    KeepAlive,
                    #[codec(index = 5)]
                    #[doc = "A vesting schedule already exists for this account"]
                    ExistingVestingSchedule,
                    #[codec(index = 6)]
                    #[doc = "Beneficiary account must pre-exist"]
                    DeadAccount,
                    #[codec(index = 7)]
                    #[doc = "Number of named reserves exceed MaxReserves"]
                    TooManyReserves,
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "An account was created with some free balance."]
                    Endowed {
                        account: ::subxt::sp_core::crypto::AccountId32,
                        free_balance: ::core::primitive::u128,
                    },
                    #[codec(index = 1)]
                    #[doc = "An account was removed whose balance was non-zero but below ExistentialDeposit,"]
                    #[doc = "resulting in an outright loss."]
                    DustLost {
                        account: ::subxt::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    #[doc = "Transfer succeeded."]
                    Transfer {
                        from: ::subxt::sp_core::crypto::AccountId32,
                        to: ::subxt::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    #[doc = "A balance was set by root."]
                    BalanceSet {
                        who: ::subxt::sp_core::crypto::AccountId32,
                        free: ::core::primitive::u128,
                        reserved: ::core::primitive::u128,
                    },
                    #[codec(index = 4)]
                    #[doc = "Some balance was reserved (moved from free to reserved)."]
                    Reserved {
                        who: ::subxt::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 5)]
                    #[doc = "Some balance was unreserved (moved from reserved to free)."]
                    Unreserved {
                        who: ::subxt::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 6)]
                    #[doc = "Some balance was moved from the reserve of the first account to the second account."]
                    #[doc = "Final argument indicates the destination balance type."]
                    ReserveRepatriated {
                        from: ::subxt::sp_core::crypto::AccountId32,
                        to: ::subxt::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                        destination_status:
                            runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
                    },
                    #[codec(index = 7)]
                    #[doc = "Some amount was deposited (e.g. for transaction fees)."]
                    Deposit {
                        who: ::subxt::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 8)]
                    #[doc = "Some amount was withdrawn from the account (e.g. for transaction fees)."]
                    Withdraw {
                        who: ::subxt::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 9)]
                    #[doc = "Some amount was removed from the account (e.g. for misbehavior)."]
                    Slashed {
                        who: ::subxt::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                }
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct AccountData<_0> {
                pub free: _0,
                pub reserved: _0,
                pub misc_frozen: _0,
                pub fee_frozen: _0,
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct BalanceLock<_0> {
                pub id: [::core::primitive::u8; 8usize],
                pub amount: _0,
                pub reasons: runtime_types::pallet_balances::Reasons,
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub enum Reasons {
                #[codec(index = 0)]
                Fee,
                #[codec(index = 1)]
                Misc,
                #[codec(index = 2)]
                All,
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub enum Releases {
                #[codec(index = 0)]
                V1_0_0,
                #[codec(index = 1)]
                V2_0_0,
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct ReserveData<_0, _1> {
                pub id: _0,
                pub amount: _1,
            }
        }
        pub mod pallet_grandpa {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Report voter equivocation/misbehavior. This method will verify the"]
                    #[doc = "equivocation proof and validate the given key ownership proof"]
                    #[doc = "against the extracted offender. If both are valid, the offence"]
                    #[doc = "will be reported."]
                    report_equivocation {
                        equivocation_proof: ::std::boxed::Box<
                            runtime_types::sp_finality_grandpa::EquivocationProof<
                                ::subxt::sp_core::H256,
                                ::core::primitive::u32,
                            >,
                        >,
                        key_owner_proof: runtime_types::sp_core::Void,
                    },
                    #[codec(index = 1)]
                    #[doc = "Report voter equivocation/misbehavior. This method will verify the"]
                    #[doc = "equivocation proof and validate the given key ownership proof"]
                    #[doc = "against the extracted offender. If both are valid, the offence"]
                    #[doc = "will be reported."]
                    #[doc = ""]
                    #[doc = "This extrinsic must be called unsigned and it is expected that only"]
                    #[doc = "block authors will call it (validated in `ValidateUnsigned`), as such"]
                    #[doc = "if the block author is defined it will be defined as the equivocation"]
                    #[doc = "reporter."]
                    report_equivocation_unsigned {
                        equivocation_proof: ::std::boxed::Box<
                            runtime_types::sp_finality_grandpa::EquivocationProof<
                                ::subxt::sp_core::H256,
                                ::core::primitive::u32,
                            >,
                        >,
                        key_owner_proof: runtime_types::sp_core::Void,
                    },
                    #[codec(index = 2)]
                    #[doc = "Note that the current authority set of the GRANDPA finality gadget has stalled."]
                    #[doc = ""]
                    #[doc = "This will trigger a forced authority set change at the beginning of the next session, to"]
                    #[doc = "be enacted `delay` blocks after that. The `delay` should be high enough to safely assume"]
                    #[doc = "that the block signalling the forced change will not be re-orged e.g. 1000 blocks."]
                    #[doc = "The block production rate (which may be slowed down because of finality lagging) should"]
                    #[doc = "be taken into account when choosing the `delay`. The GRANDPA voters based on the new"]
                    #[doc = "authority will start voting on top of `best_finalized_block_number` for new finalized"]
                    #[doc = "blocks. `best_finalized_block_number` should be the highest of the latest finalized"]
                    #[doc = "block of all validators of the new authority set."]
                    #[doc = ""]
                    #[doc = "Only callable by root."]
                    note_stalled {
                        delay: ::core::primitive::u32,
                        best_finalized_block_number: ::core::primitive::u32,
                    },
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Attempt to signal GRANDPA pause when the authority set isn't live"]
                    #[doc = "(either paused or already pending pause)."]
                    PauseFailed,
                    #[codec(index = 1)]
                    #[doc = "Attempt to signal GRANDPA resume when the authority set isn't paused"]
                    #[doc = "(either live or already pending resume)."]
                    ResumeFailed,
                    #[codec(index = 2)]
                    #[doc = "Attempt to signal GRANDPA change with one already pending."]
                    ChangePending,
                    #[codec(index = 3)]
                    #[doc = "Cannot signal forced change so soon after last."]
                    TooSoon,
                    #[codec(index = 4)]
                    #[doc = "A key ownership proof provided as part of an equivocation report is invalid."]
                    InvalidKeyOwnershipProof,
                    #[codec(index = 5)]
                    #[doc = "An equivocation proof provided as part of an equivocation report is invalid."]
                    InvalidEquivocationProof,
                    #[codec(index = 6)]
                    #[doc = "A given equivocation report is valid but already previously reported."]
                    DuplicateOffenceReport,
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "New authority set has been applied."]
                    NewAuthorities {
                        authority_set: ::std::vec::Vec<(
                            runtime_types::sp_finality_grandpa::app::Public,
                            ::core::primitive::u64,
                        )>,
                    },
                    #[codec(index = 1)]
                    #[doc = "Current authority set has been paused."]
                    Paused,
                    #[codec(index = 2)]
                    #[doc = "Current authority set has been resumed."]
                    Resumed,
                }
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct StoredPendingChange<_0> {
                pub scheduled_at: _0,
                pub delay: _0,
                pub next_authorities:
                    runtime_types::sp_runtime::bounded::weak_bounded_vec::WeakBoundedVec<(
                        runtime_types::sp_finality_grandpa::app::Public,
                        ::core::primitive::u64,
                    )>,
                pub forced: ::core::option::Option<_0>,
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub enum StoredState<_0> {
                #[codec(index = 0)]
                Live,
                #[codec(index = 1)]
                PendingPause { scheduled_at: _0, delay: _0 },
                #[codec(index = 2)]
                Paused,
                #[codec(index = 3)]
                PendingResume { scheduled_at: _0, delay: _0 },
            }
        }
        pub mod pallet_ibc {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "An example dispatchable that takes a singles value as a parameter, writes the value to"]
                    #[doc = "storage and emits an event. This function must be dispatched by a signed extrinsic."]
                    do_something { something: ::core::primitive::u32 },
                    #[codec(index = 1)]
                    #[doc = "An example dispatchable that may throw a custom error."]
                    cause_error,
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Error names should be descriptive."]
                    NoneValue,
                    #[codec(index = 1)]
                    #[doc = "Errors should have helpful documentation associated with them."]
                    StorageOverflow,
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "Event documentation should end with an array that provides descriptive names for event"]
                    #[doc = "parameters. [something, who]"]
                    SomethingStored(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                    ),
                    #[codec(index = 1)]
                    #[doc = "Store map key, value and who"]
                    SomeMapStored(
                        ::core::primitive::u32,
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                    ),
                }
            }
        }
        pub mod pallet_sudo {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- O(1)."]
                    #[doc = "- Limited storage reads."]
                    #[doc = "- One DB write (event)."]
                    #[doc = "- Weight of derivative `call` execution + 10,000."]
                    #[doc = "# </weight>"]
                    sudo {
                        call: ::std::boxed::Box<runtime_types::node_template_runtime::Call>,
                    },
                    #[codec(index = 1)]
                    #[doc = "Authenticates the sudo key and dispatches a function call with `Root` origin."]
                    #[doc = "This function does not check the weight of the call, and instead allows the"]
                    #[doc = "Sudo user to specify the weight of the call."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- O(1)."]
                    #[doc = "- The weight of this call is defined by the caller."]
                    #[doc = "# </weight>"]
                    sudo_unchecked_weight {
                        call: ::std::boxed::Box<runtime_types::node_template_runtime::Call>,
                        weight: ::core::primitive::u64,
                    },
                    #[codec(index = 2)]
                    #[doc = "Authenticates the current sudo key and sets the given AccountId (`new`) as the new sudo"]
                    #[doc = "key."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- O(1)."]
                    #[doc = "- Limited storage reads."]
                    #[doc = "- One DB change."]
                    #[doc = "# </weight>"]
                    set_key {
                        new: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 3)]
                    #[doc = "Authenticates the sudo key and dispatches a function call with `Signed` origin from"]
                    #[doc = "a given account."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be _Signed_."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- O(1)."]
                    #[doc = "- Limited storage reads."]
                    #[doc = "- One DB write (event)."]
                    #[doc = "- Weight of derivative `call` execution + 10,000."]
                    #[doc = "# </weight>"]
                    sudo_as {
                        who: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        call: ::std::boxed::Box<runtime_types::node_template_runtime::Call>,
                    },
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Sender must be the Sudo account"]
                    RequireSudo,
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "A sudo just took place. \\[result\\]"]
                    Sudid {
                        sudo_result:
                            ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    },
                    #[codec(index = 1)]
                    #[doc = "The \\[sudoer\\] just switched identity; the old key is supplied if one existed."]
                    KeyChanged {
                        old_sudoer: ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
                    },
                    #[codec(index = 2)]
                    #[doc = "A sudo just took place. \\[result\\]"]
                    SudoAsDone {
                        sudo_result:
                            ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    },
                }
            }
        }
        pub mod pallet_template {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "An example dispatchable that takes a singles value as a parameter, writes the value to"]
                    #[doc = "storage and emits an event. This function must be dispatched by a signed extrinsic."]
                    do_something { something: ::core::primitive::u32 },
                    #[codec(index = 1)]
                    #[doc = "An example dispatchable that may throw a custom error."]
                    cause_error,
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Error names should be descriptive."]
                    NoneValue,
                    #[codec(index = 1)]
                    #[doc = "Errors should have helpful documentation associated with them."]
                    StorageOverflow,
                }
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "Event documentation should end with an array that provides descriptive names for event"]
                    #[doc = "parameters. [something, who]"]
                    SomethingStored(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                    ),
                }
            }
        }
        pub mod pallet_timestamp {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "Set the current time."]
                    #[doc = ""]
                    #[doc = "This call should be invoked exactly once per block. It will panic at the finalization"]
                    #[doc = "phase, if this call hasn't been invoked by that time."]
                    #[doc = ""]
                    #[doc = "The timestamp should be greater than the previous one by the amount specified by"]
                    #[doc = "`MinimumPeriod`."]
                    #[doc = ""]
                    #[doc = "The dispatch origin for this call must be `Inherent`."]
                    #[doc = ""]
                    #[doc = "# <weight>"]
                    #[doc = "- `O(1)` (Note that implementations of `OnTimestampSet` must also be `O(1)`)"]
                    #[doc = "- 1 storage read and 1 storage mutation (codec `O(1)`). (because of `DidUpdate::take` in"]
                    #[doc = "  `on_finalize`)"]
                    #[doc = "- 1 event handler `on_timestamp_set`. Must be `O(1)`."]
                    #[doc = "# </weight>"]
                    set {
                        #[codec(compact)]
                        now: ::core::primitive::u64,
                    },
                }
            }
        }
        pub mod pallet_transaction_payment {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct ChargeTransactionPayment(#[codec(compact)] pub ::core::primitive::u128);
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub enum Releases {
                #[codec(index = 0)]
                V1Ancient,
                #[codec(index = 1)]
                V2,
            }
        }
        pub mod primitive_types {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct H256(pub [::core::primitive::u8; 32usize]);
        }
        pub mod sp_arithmetic {
            use super::runtime_types;
            pub mod fixed_point {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: CompactAs,
                    :: subxt :: codec :: Decode,
                    :: subxt :: codec :: Encode,
                    Debug,
                )]
                pub struct FixedU128(pub ::core::primitive::u128);
            }
            pub mod per_things {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: CompactAs,
                    :: subxt :: codec :: Decode,
                    :: subxt :: codec :: Encode,
                    Debug,
                )]
                pub struct Perbill(pub ::core::primitive::u32);
            }
        }
        pub mod sp_consensus_aura {
            use super::runtime_types;
            pub mod sr25519 {
                use super::runtime_types;
                pub mod app_sr25519 {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                    pub struct Public(pub runtime_types::sp_core::sr25519::Public);
                }
            }
        }
        pub mod sp_consensus_slots {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: CompactAs,
                :: subxt :: codec :: Decode,
                :: subxt :: codec :: Encode,
                Debug,
            )]
            pub struct Slot(pub ::core::primitive::u64);
        }
        pub mod sp_core {
            use super::runtime_types;
            pub mod crypto {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub struct AccountId32(pub [::core::primitive::u8; 32usize]);
            }
            pub mod ecdsa {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub struct Signature(pub [::core::primitive::u8; 65usize]);
            }
            pub mod ed25519 {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub struct Public(pub [::core::primitive::u8; 32usize]);
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub struct Signature(pub [::core::primitive::u8; 64usize]);
            }
            pub mod sr25519 {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub struct Public(pub [::core::primitive::u8; 32usize]);
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub struct Signature(pub [::core::primitive::u8; 64usize]);
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub enum Void {}
        }
        pub mod sp_finality_grandpa {
            use super::runtime_types;
            pub mod app {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub struct Public(pub runtime_types::sp_core::ed25519::Public);
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub struct Signature(pub runtime_types::sp_core::ed25519::Signature);
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub enum Equivocation<_0, _1> {
                #[codec(index = 0)]
                Prevote(
                    runtime_types::finality_grandpa::Equivocation<
                        runtime_types::sp_finality_grandpa::app::Public,
                        runtime_types::finality_grandpa::Prevote<_0, _1>,
                        runtime_types::sp_finality_grandpa::app::Signature,
                    >,
                ),
                #[codec(index = 1)]
                Precommit(
                    runtime_types::finality_grandpa::Equivocation<
                        runtime_types::sp_finality_grandpa::app::Public,
                        runtime_types::finality_grandpa::Precommit<_0, _1>,
                        runtime_types::sp_finality_grandpa::app::Signature,
                    >,
                ),
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct EquivocationProof<_0, _1> {
                pub set_id: ::core::primitive::u64,
                pub equivocation: runtime_types::sp_finality_grandpa::Equivocation<_0, _1>,
            }
        }
        pub mod sp_runtime {
            use super::runtime_types;
            pub mod bounded {
                use super::runtime_types;
                pub mod bounded_vec {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                    pub struct BoundedVec<_0>(pub ::std::vec::Vec<_0>);
                }
                pub mod weak_bounded_vec {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                    pub struct WeakBoundedVec<_0>(pub ::std::vec::Vec<_0>);
                }
            }
            pub mod generic {
                use super::runtime_types;
                pub mod digest {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                    pub struct Digest {
                        pub logs:
                            ::std::vec::Vec<runtime_types::sp_runtime::generic::digest::DigestItem>,
                    }
                    #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                    pub enum DigestItem {
                        #[codec(index = 6)]
                        PreRuntime(
                            [::core::primitive::u8; 4usize],
                            ::std::vec::Vec<::core::primitive::u8>,
                        ),
                        #[codec(index = 4)]
                        Consensus(
                            [::core::primitive::u8; 4usize],
                            ::std::vec::Vec<::core::primitive::u8>,
                        ),
                        #[codec(index = 5)]
                        Seal(
                            [::core::primitive::u8; 4usize],
                            ::std::vec::Vec<::core::primitive::u8>,
                        ),
                        #[codec(index = 0)]
                        Other(::std::vec::Vec<::core::primitive::u8>),
                        #[codec(index = 8)]
                        RuntimeEnvironmentUpdated,
                    }
                }
                pub mod era {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                    pub enum Era {
                        #[codec(index = 0)]
                        Immortal,
                        #[codec(index = 1)]
                        Mortal1(::core::primitive::u8),
                        #[codec(index = 2)]
                        Mortal2(::core::primitive::u8),
                        #[codec(index = 3)]
                        Mortal3(::core::primitive::u8),
                        #[codec(index = 4)]
                        Mortal4(::core::primitive::u8),
                        #[codec(index = 5)]
                        Mortal5(::core::primitive::u8),
                        #[codec(index = 6)]
                        Mortal6(::core::primitive::u8),
                        #[codec(index = 7)]
                        Mortal7(::core::primitive::u8),
                        #[codec(index = 8)]
                        Mortal8(::core::primitive::u8),
                        #[codec(index = 9)]
                        Mortal9(::core::primitive::u8),
                        #[codec(index = 10)]
                        Mortal10(::core::primitive::u8),
                        #[codec(index = 11)]
                        Mortal11(::core::primitive::u8),
                        #[codec(index = 12)]
                        Mortal12(::core::primitive::u8),
                        #[codec(index = 13)]
                        Mortal13(::core::primitive::u8),
                        #[codec(index = 14)]
                        Mortal14(::core::primitive::u8),
                        #[codec(index = 15)]
                        Mortal15(::core::primitive::u8),
                        #[codec(index = 16)]
                        Mortal16(::core::primitive::u8),
                        #[codec(index = 17)]
                        Mortal17(::core::primitive::u8),
                        #[codec(index = 18)]
                        Mortal18(::core::primitive::u8),
                        #[codec(index = 19)]
                        Mortal19(::core::primitive::u8),
                        #[codec(index = 20)]
                        Mortal20(::core::primitive::u8),
                        #[codec(index = 21)]
                        Mortal21(::core::primitive::u8),
                        #[codec(index = 22)]
                        Mortal22(::core::primitive::u8),
                        #[codec(index = 23)]
                        Mortal23(::core::primitive::u8),
                        #[codec(index = 24)]
                        Mortal24(::core::primitive::u8),
                        #[codec(index = 25)]
                        Mortal25(::core::primitive::u8),
                        #[codec(index = 26)]
                        Mortal26(::core::primitive::u8),
                        #[codec(index = 27)]
                        Mortal27(::core::primitive::u8),
                        #[codec(index = 28)]
                        Mortal28(::core::primitive::u8),
                        #[codec(index = 29)]
                        Mortal29(::core::primitive::u8),
                        #[codec(index = 30)]
                        Mortal30(::core::primitive::u8),
                        #[codec(index = 31)]
                        Mortal31(::core::primitive::u8),
                        #[codec(index = 32)]
                        Mortal32(::core::primitive::u8),
                        #[codec(index = 33)]
                        Mortal33(::core::primitive::u8),
                        #[codec(index = 34)]
                        Mortal34(::core::primitive::u8),
                        #[codec(index = 35)]
                        Mortal35(::core::primitive::u8),
                        #[codec(index = 36)]
                        Mortal36(::core::primitive::u8),
                        #[codec(index = 37)]
                        Mortal37(::core::primitive::u8),
                        #[codec(index = 38)]
                        Mortal38(::core::primitive::u8),
                        #[codec(index = 39)]
                        Mortal39(::core::primitive::u8),
                        #[codec(index = 40)]
                        Mortal40(::core::primitive::u8),
                        #[codec(index = 41)]
                        Mortal41(::core::primitive::u8),
                        #[codec(index = 42)]
                        Mortal42(::core::primitive::u8),
                        #[codec(index = 43)]
                        Mortal43(::core::primitive::u8),
                        #[codec(index = 44)]
                        Mortal44(::core::primitive::u8),
                        #[codec(index = 45)]
                        Mortal45(::core::primitive::u8),
                        #[codec(index = 46)]
                        Mortal46(::core::primitive::u8),
                        #[codec(index = 47)]
                        Mortal47(::core::primitive::u8),
                        #[codec(index = 48)]
                        Mortal48(::core::primitive::u8),
                        #[codec(index = 49)]
                        Mortal49(::core::primitive::u8),
                        #[codec(index = 50)]
                        Mortal50(::core::primitive::u8),
                        #[codec(index = 51)]
                        Mortal51(::core::primitive::u8),
                        #[codec(index = 52)]
                        Mortal52(::core::primitive::u8),
                        #[codec(index = 53)]
                        Mortal53(::core::primitive::u8),
                        #[codec(index = 54)]
                        Mortal54(::core::primitive::u8),
                        #[codec(index = 55)]
                        Mortal55(::core::primitive::u8),
                        #[codec(index = 56)]
                        Mortal56(::core::primitive::u8),
                        #[codec(index = 57)]
                        Mortal57(::core::primitive::u8),
                        #[codec(index = 58)]
                        Mortal58(::core::primitive::u8),
                        #[codec(index = 59)]
                        Mortal59(::core::primitive::u8),
                        #[codec(index = 60)]
                        Mortal60(::core::primitive::u8),
                        #[codec(index = 61)]
                        Mortal61(::core::primitive::u8),
                        #[codec(index = 62)]
                        Mortal62(::core::primitive::u8),
                        #[codec(index = 63)]
                        Mortal63(::core::primitive::u8),
                        #[codec(index = 64)]
                        Mortal64(::core::primitive::u8),
                        #[codec(index = 65)]
                        Mortal65(::core::primitive::u8),
                        #[codec(index = 66)]
                        Mortal66(::core::primitive::u8),
                        #[codec(index = 67)]
                        Mortal67(::core::primitive::u8),
                        #[codec(index = 68)]
                        Mortal68(::core::primitive::u8),
                        #[codec(index = 69)]
                        Mortal69(::core::primitive::u8),
                        #[codec(index = 70)]
                        Mortal70(::core::primitive::u8),
                        #[codec(index = 71)]
                        Mortal71(::core::primitive::u8),
                        #[codec(index = 72)]
                        Mortal72(::core::primitive::u8),
                        #[codec(index = 73)]
                        Mortal73(::core::primitive::u8),
                        #[codec(index = 74)]
                        Mortal74(::core::primitive::u8),
                        #[codec(index = 75)]
                        Mortal75(::core::primitive::u8),
                        #[codec(index = 76)]
                        Mortal76(::core::primitive::u8),
                        #[codec(index = 77)]
                        Mortal77(::core::primitive::u8),
                        #[codec(index = 78)]
                        Mortal78(::core::primitive::u8),
                        #[codec(index = 79)]
                        Mortal79(::core::primitive::u8),
                        #[codec(index = 80)]
                        Mortal80(::core::primitive::u8),
                        #[codec(index = 81)]
                        Mortal81(::core::primitive::u8),
                        #[codec(index = 82)]
                        Mortal82(::core::primitive::u8),
                        #[codec(index = 83)]
                        Mortal83(::core::primitive::u8),
                        #[codec(index = 84)]
                        Mortal84(::core::primitive::u8),
                        #[codec(index = 85)]
                        Mortal85(::core::primitive::u8),
                        #[codec(index = 86)]
                        Mortal86(::core::primitive::u8),
                        #[codec(index = 87)]
                        Mortal87(::core::primitive::u8),
                        #[codec(index = 88)]
                        Mortal88(::core::primitive::u8),
                        #[codec(index = 89)]
                        Mortal89(::core::primitive::u8),
                        #[codec(index = 90)]
                        Mortal90(::core::primitive::u8),
                        #[codec(index = 91)]
                        Mortal91(::core::primitive::u8),
                        #[codec(index = 92)]
                        Mortal92(::core::primitive::u8),
                        #[codec(index = 93)]
                        Mortal93(::core::primitive::u8),
                        #[codec(index = 94)]
                        Mortal94(::core::primitive::u8),
                        #[codec(index = 95)]
                        Mortal95(::core::primitive::u8),
                        #[codec(index = 96)]
                        Mortal96(::core::primitive::u8),
                        #[codec(index = 97)]
                        Mortal97(::core::primitive::u8),
                        #[codec(index = 98)]
                        Mortal98(::core::primitive::u8),
                        #[codec(index = 99)]
                        Mortal99(::core::primitive::u8),
                        #[codec(index = 100)]
                        Mortal100(::core::primitive::u8),
                        #[codec(index = 101)]
                        Mortal101(::core::primitive::u8),
                        #[codec(index = 102)]
                        Mortal102(::core::primitive::u8),
                        #[codec(index = 103)]
                        Mortal103(::core::primitive::u8),
                        #[codec(index = 104)]
                        Mortal104(::core::primitive::u8),
                        #[codec(index = 105)]
                        Mortal105(::core::primitive::u8),
                        #[codec(index = 106)]
                        Mortal106(::core::primitive::u8),
                        #[codec(index = 107)]
                        Mortal107(::core::primitive::u8),
                        #[codec(index = 108)]
                        Mortal108(::core::primitive::u8),
                        #[codec(index = 109)]
                        Mortal109(::core::primitive::u8),
                        #[codec(index = 110)]
                        Mortal110(::core::primitive::u8),
                        #[codec(index = 111)]
                        Mortal111(::core::primitive::u8),
                        #[codec(index = 112)]
                        Mortal112(::core::primitive::u8),
                        #[codec(index = 113)]
                        Mortal113(::core::primitive::u8),
                        #[codec(index = 114)]
                        Mortal114(::core::primitive::u8),
                        #[codec(index = 115)]
                        Mortal115(::core::primitive::u8),
                        #[codec(index = 116)]
                        Mortal116(::core::primitive::u8),
                        #[codec(index = 117)]
                        Mortal117(::core::primitive::u8),
                        #[codec(index = 118)]
                        Mortal118(::core::primitive::u8),
                        #[codec(index = 119)]
                        Mortal119(::core::primitive::u8),
                        #[codec(index = 120)]
                        Mortal120(::core::primitive::u8),
                        #[codec(index = 121)]
                        Mortal121(::core::primitive::u8),
                        #[codec(index = 122)]
                        Mortal122(::core::primitive::u8),
                        #[codec(index = 123)]
                        Mortal123(::core::primitive::u8),
                        #[codec(index = 124)]
                        Mortal124(::core::primitive::u8),
                        #[codec(index = 125)]
                        Mortal125(::core::primitive::u8),
                        #[codec(index = 126)]
                        Mortal126(::core::primitive::u8),
                        #[codec(index = 127)]
                        Mortal127(::core::primitive::u8),
                        #[codec(index = 128)]
                        Mortal128(::core::primitive::u8),
                        #[codec(index = 129)]
                        Mortal129(::core::primitive::u8),
                        #[codec(index = 130)]
                        Mortal130(::core::primitive::u8),
                        #[codec(index = 131)]
                        Mortal131(::core::primitive::u8),
                        #[codec(index = 132)]
                        Mortal132(::core::primitive::u8),
                        #[codec(index = 133)]
                        Mortal133(::core::primitive::u8),
                        #[codec(index = 134)]
                        Mortal134(::core::primitive::u8),
                        #[codec(index = 135)]
                        Mortal135(::core::primitive::u8),
                        #[codec(index = 136)]
                        Mortal136(::core::primitive::u8),
                        #[codec(index = 137)]
                        Mortal137(::core::primitive::u8),
                        #[codec(index = 138)]
                        Mortal138(::core::primitive::u8),
                        #[codec(index = 139)]
                        Mortal139(::core::primitive::u8),
                        #[codec(index = 140)]
                        Mortal140(::core::primitive::u8),
                        #[codec(index = 141)]
                        Mortal141(::core::primitive::u8),
                        #[codec(index = 142)]
                        Mortal142(::core::primitive::u8),
                        #[codec(index = 143)]
                        Mortal143(::core::primitive::u8),
                        #[codec(index = 144)]
                        Mortal144(::core::primitive::u8),
                        #[codec(index = 145)]
                        Mortal145(::core::primitive::u8),
                        #[codec(index = 146)]
                        Mortal146(::core::primitive::u8),
                        #[codec(index = 147)]
                        Mortal147(::core::primitive::u8),
                        #[codec(index = 148)]
                        Mortal148(::core::primitive::u8),
                        #[codec(index = 149)]
                        Mortal149(::core::primitive::u8),
                        #[codec(index = 150)]
                        Mortal150(::core::primitive::u8),
                        #[codec(index = 151)]
                        Mortal151(::core::primitive::u8),
                        #[codec(index = 152)]
                        Mortal152(::core::primitive::u8),
                        #[codec(index = 153)]
                        Mortal153(::core::primitive::u8),
                        #[codec(index = 154)]
                        Mortal154(::core::primitive::u8),
                        #[codec(index = 155)]
                        Mortal155(::core::primitive::u8),
                        #[codec(index = 156)]
                        Mortal156(::core::primitive::u8),
                        #[codec(index = 157)]
                        Mortal157(::core::primitive::u8),
                        #[codec(index = 158)]
                        Mortal158(::core::primitive::u8),
                        #[codec(index = 159)]
                        Mortal159(::core::primitive::u8),
                        #[codec(index = 160)]
                        Mortal160(::core::primitive::u8),
                        #[codec(index = 161)]
                        Mortal161(::core::primitive::u8),
                        #[codec(index = 162)]
                        Mortal162(::core::primitive::u8),
                        #[codec(index = 163)]
                        Mortal163(::core::primitive::u8),
                        #[codec(index = 164)]
                        Mortal164(::core::primitive::u8),
                        #[codec(index = 165)]
                        Mortal165(::core::primitive::u8),
                        #[codec(index = 166)]
                        Mortal166(::core::primitive::u8),
                        #[codec(index = 167)]
                        Mortal167(::core::primitive::u8),
                        #[codec(index = 168)]
                        Mortal168(::core::primitive::u8),
                        #[codec(index = 169)]
                        Mortal169(::core::primitive::u8),
                        #[codec(index = 170)]
                        Mortal170(::core::primitive::u8),
                        #[codec(index = 171)]
                        Mortal171(::core::primitive::u8),
                        #[codec(index = 172)]
                        Mortal172(::core::primitive::u8),
                        #[codec(index = 173)]
                        Mortal173(::core::primitive::u8),
                        #[codec(index = 174)]
                        Mortal174(::core::primitive::u8),
                        #[codec(index = 175)]
                        Mortal175(::core::primitive::u8),
                        #[codec(index = 176)]
                        Mortal176(::core::primitive::u8),
                        #[codec(index = 177)]
                        Mortal177(::core::primitive::u8),
                        #[codec(index = 178)]
                        Mortal178(::core::primitive::u8),
                        #[codec(index = 179)]
                        Mortal179(::core::primitive::u8),
                        #[codec(index = 180)]
                        Mortal180(::core::primitive::u8),
                        #[codec(index = 181)]
                        Mortal181(::core::primitive::u8),
                        #[codec(index = 182)]
                        Mortal182(::core::primitive::u8),
                        #[codec(index = 183)]
                        Mortal183(::core::primitive::u8),
                        #[codec(index = 184)]
                        Mortal184(::core::primitive::u8),
                        #[codec(index = 185)]
                        Mortal185(::core::primitive::u8),
                        #[codec(index = 186)]
                        Mortal186(::core::primitive::u8),
                        #[codec(index = 187)]
                        Mortal187(::core::primitive::u8),
                        #[codec(index = 188)]
                        Mortal188(::core::primitive::u8),
                        #[codec(index = 189)]
                        Mortal189(::core::primitive::u8),
                        #[codec(index = 190)]
                        Mortal190(::core::primitive::u8),
                        #[codec(index = 191)]
                        Mortal191(::core::primitive::u8),
                        #[codec(index = 192)]
                        Mortal192(::core::primitive::u8),
                        #[codec(index = 193)]
                        Mortal193(::core::primitive::u8),
                        #[codec(index = 194)]
                        Mortal194(::core::primitive::u8),
                        #[codec(index = 195)]
                        Mortal195(::core::primitive::u8),
                        #[codec(index = 196)]
                        Mortal196(::core::primitive::u8),
                        #[codec(index = 197)]
                        Mortal197(::core::primitive::u8),
                        #[codec(index = 198)]
                        Mortal198(::core::primitive::u8),
                        #[codec(index = 199)]
                        Mortal199(::core::primitive::u8),
                        #[codec(index = 200)]
                        Mortal200(::core::primitive::u8),
                        #[codec(index = 201)]
                        Mortal201(::core::primitive::u8),
                        #[codec(index = 202)]
                        Mortal202(::core::primitive::u8),
                        #[codec(index = 203)]
                        Mortal203(::core::primitive::u8),
                        #[codec(index = 204)]
                        Mortal204(::core::primitive::u8),
                        #[codec(index = 205)]
                        Mortal205(::core::primitive::u8),
                        #[codec(index = 206)]
                        Mortal206(::core::primitive::u8),
                        #[codec(index = 207)]
                        Mortal207(::core::primitive::u8),
                        #[codec(index = 208)]
                        Mortal208(::core::primitive::u8),
                        #[codec(index = 209)]
                        Mortal209(::core::primitive::u8),
                        #[codec(index = 210)]
                        Mortal210(::core::primitive::u8),
                        #[codec(index = 211)]
                        Mortal211(::core::primitive::u8),
                        #[codec(index = 212)]
                        Mortal212(::core::primitive::u8),
                        #[codec(index = 213)]
                        Mortal213(::core::primitive::u8),
                        #[codec(index = 214)]
                        Mortal214(::core::primitive::u8),
                        #[codec(index = 215)]
                        Mortal215(::core::primitive::u8),
                        #[codec(index = 216)]
                        Mortal216(::core::primitive::u8),
                        #[codec(index = 217)]
                        Mortal217(::core::primitive::u8),
                        #[codec(index = 218)]
                        Mortal218(::core::primitive::u8),
                        #[codec(index = 219)]
                        Mortal219(::core::primitive::u8),
                        #[codec(index = 220)]
                        Mortal220(::core::primitive::u8),
                        #[codec(index = 221)]
                        Mortal221(::core::primitive::u8),
                        #[codec(index = 222)]
                        Mortal222(::core::primitive::u8),
                        #[codec(index = 223)]
                        Mortal223(::core::primitive::u8),
                        #[codec(index = 224)]
                        Mortal224(::core::primitive::u8),
                        #[codec(index = 225)]
                        Mortal225(::core::primitive::u8),
                        #[codec(index = 226)]
                        Mortal226(::core::primitive::u8),
                        #[codec(index = 227)]
                        Mortal227(::core::primitive::u8),
                        #[codec(index = 228)]
                        Mortal228(::core::primitive::u8),
                        #[codec(index = 229)]
                        Mortal229(::core::primitive::u8),
                        #[codec(index = 230)]
                        Mortal230(::core::primitive::u8),
                        #[codec(index = 231)]
                        Mortal231(::core::primitive::u8),
                        #[codec(index = 232)]
                        Mortal232(::core::primitive::u8),
                        #[codec(index = 233)]
                        Mortal233(::core::primitive::u8),
                        #[codec(index = 234)]
                        Mortal234(::core::primitive::u8),
                        #[codec(index = 235)]
                        Mortal235(::core::primitive::u8),
                        #[codec(index = 236)]
                        Mortal236(::core::primitive::u8),
                        #[codec(index = 237)]
                        Mortal237(::core::primitive::u8),
                        #[codec(index = 238)]
                        Mortal238(::core::primitive::u8),
                        #[codec(index = 239)]
                        Mortal239(::core::primitive::u8),
                        #[codec(index = 240)]
                        Mortal240(::core::primitive::u8),
                        #[codec(index = 241)]
                        Mortal241(::core::primitive::u8),
                        #[codec(index = 242)]
                        Mortal242(::core::primitive::u8),
                        #[codec(index = 243)]
                        Mortal243(::core::primitive::u8),
                        #[codec(index = 244)]
                        Mortal244(::core::primitive::u8),
                        #[codec(index = 245)]
                        Mortal245(::core::primitive::u8),
                        #[codec(index = 246)]
                        Mortal246(::core::primitive::u8),
                        #[codec(index = 247)]
                        Mortal247(::core::primitive::u8),
                        #[codec(index = 248)]
                        Mortal248(::core::primitive::u8),
                        #[codec(index = 249)]
                        Mortal249(::core::primitive::u8),
                        #[codec(index = 250)]
                        Mortal250(::core::primitive::u8),
                        #[codec(index = 251)]
                        Mortal251(::core::primitive::u8),
                        #[codec(index = 252)]
                        Mortal252(::core::primitive::u8),
                        #[codec(index = 253)]
                        Mortal253(::core::primitive::u8),
                        #[codec(index = 254)]
                        Mortal254(::core::primitive::u8),
                        #[codec(index = 255)]
                        Mortal255(::core::primitive::u8),
                    }
                }
                pub mod unchecked_extrinsic {
                    use super::runtime_types;
                    #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                    pub struct UncheckedExtrinsic<_0, _1, _2, _3>(
                        pub ::std::vec::Vec<::core::primitive::u8>,
                        #[codec(skip)] pub ::core::marker::PhantomData<(_0, _1, _2, _3)>,
                    );
                }
            }
            pub mod multiaddress {
                use super::runtime_types;
                #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
                pub enum MultiAddress<_0, _1> {
                    #[codec(index = 0)]
                    Id(_0),
                    #[codec(index = 1)]
                    Index(#[codec(compact)] _1),
                    #[codec(index = 2)]
                    Raw(::std::vec::Vec<::core::primitive::u8>),
                    #[codec(index = 3)]
                    Address32([::core::primitive::u8; 32usize]),
                    #[codec(index = 4)]
                    Address20([::core::primitive::u8; 20usize]),
                }
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub enum ArithmeticError {
                #[codec(index = 0)]
                Underflow,
                #[codec(index = 1)]
                Overflow,
                #[codec(index = 2)]
                DivisionByZero,
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub enum DispatchError {
                #[codec(index = 0)]
                Other,
                #[codec(index = 1)]
                CannotLookup,
                #[codec(index = 2)]
                BadOrigin,
                #[codec(index = 3)]
                Module(runtime_types::sp_runtime::ModuleError),
                #[codec(index = 4)]
                ConsumerRemaining,
                #[codec(index = 5)]
                NoProviders,
                #[codec(index = 6)]
                TooManyConsumers,
                #[codec(index = 7)]
                Token(runtime_types::sp_runtime::TokenError),
                #[codec(index = 8)]
                Arithmetic(runtime_types::sp_runtime::ArithmeticError),
                #[codec(index = 9)]
                Transactional(runtime_types::sp_runtime::TransactionalError),
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct ModuleError {
                pub index: ::core::primitive::u8,
                pub error: [::core::primitive::u8; 4usize],
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub enum MultiSignature {
                #[codec(index = 0)]
                Ed25519(runtime_types::sp_core::ed25519::Signature),
                #[codec(index = 1)]
                Sr25519(runtime_types::sp_core::sr25519::Signature),
                #[codec(index = 2)]
                Ecdsa(runtime_types::sp_core::ecdsa::Signature),
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub enum TokenError {
                #[codec(index = 0)]
                NoFunds,
                #[codec(index = 1)]
                WouldDie,
                #[codec(index = 2)]
                BelowMinimum,
                #[codec(index = 3)]
                CannotCreate,
                #[codec(index = 4)]
                UnknownAsset,
                #[codec(index = 5)]
                Frozen,
                #[codec(index = 6)]
                Unsupported,
            }
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub enum TransactionalError {
                #[codec(index = 0)]
                LimitReached,
                #[codec(index = 1)]
                NoLayer,
            }
        }
        pub mod sp_version {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Decode, :: subxt :: codec :: Encode, Debug)]
            pub struct RuntimeVersion {
                pub spec_name: ::std::string::String,
                pub impl_name: ::std::string::String,
                pub authoring_version: ::core::primitive::u32,
                pub spec_version: ::core::primitive::u32,
                pub impl_version: ::core::primitive::u32,
                pub apis:
                    ::std::vec::Vec<([::core::primitive::u8; 8usize], ::core::primitive::u32)>,
                pub transaction_version: ::core::primitive::u32,
                pub state_version: ::core::primitive::u8,
            }
        }
    }
    #[doc = r" The default error type returned when there is a runtime issue."]
    pub type DispatchError = runtime_types::sp_runtime::DispatchError;
    impl ::subxt::HasModuleError for runtime_types::sp_runtime::DispatchError {
        fn module_error_data(&self) -> Option<::subxt::ModuleErrorData> {
            if let Self::Module(module_error) = self {
                Some(::subxt::ModuleErrorData {
                    pallet_index: module_error.index,
                    error: module_error.error,
                })
            } else {
                None
            }
        }
    }
    pub struct RuntimeApi<T: ::subxt::Config, X> {
        pub client: ::subxt::Client<T>,
        marker: ::core::marker::PhantomData<X>,
    }
    impl<T, X> ::core::convert::From<::subxt::Client<T>> for RuntimeApi<T, X>
    where
        T: ::subxt::Config,
        X: ::subxt::extrinsic::ExtrinsicParams<T>,
    {
        fn from(client: ::subxt::Client<T>) -> Self {
            Self {
                client,
                marker: ::core::marker::PhantomData,
            }
        }
    }
    impl<'a, T, X> RuntimeApi<T, X>
    where
        T: ::subxt::Config,
        X: ::subxt::extrinsic::ExtrinsicParams<T>,
    {
        pub fn validate_metadata(&'a self) -> Result<(), ::subxt::MetadataError> {
            if self.client.metadata().metadata_hash(&PALLETS)
                != [
                    162u8, 95u8, 254u8, 12u8, 34u8, 1u8, 87u8, 146u8, 86u8, 61u8, 177u8, 121u8,
                    216u8, 197u8, 247u8, 131u8, 148u8, 65u8, 225u8, 98u8, 204u8, 188u8, 103u8,
                    59u8, 213u8, 194u8, 99u8, 159u8, 145u8, 20u8, 209u8, 226u8,
                ]
            {
                Err(::subxt::MetadataError::IncompatibleMetadata)
            } else {
                Ok(())
            }
        }
        pub fn constants(&'a self) -> ConstantsApi<'a, T> {
            ConstantsApi {
                client: &self.client,
            }
        }
        pub fn storage(&'a self) -> StorageApi<'a, T> {
            StorageApi {
                client: &self.client,
            }
        }
        pub fn tx(&'a self) -> TransactionApi<'a, T, X> {
            TransactionApi {
                client: &self.client,
                marker: ::core::marker::PhantomData,
            }
        }
        pub fn events(&'a self) -> EventsApi<'a, T> {
            EventsApi {
                client: &self.client,
            }
        }
    }
    pub struct EventsApi<'a, T: ::subxt::Config> {
        client: &'a ::subxt::Client<T>,
    }
    impl<'a, T: ::subxt::Config> EventsApi<'a, T> {
        pub async fn at(
            &self,
            block_hash: T::Hash,
        ) -> Result<::subxt::events::Events<'a, T, Event>, ::subxt::BasicError> {
            ::subxt::events::at::<T, Event>(self.client, block_hash).await
        }
        pub async fn subscribe(
            &self,
        ) -> Result<
            ::subxt::events::EventSubscription<'a, ::subxt::events::EventSub<T::Header>, T, Event>,
            ::subxt::BasicError,
        > {
            ::subxt::events::subscribe::<T, Event>(self.client).await
        }
        pub async fn subscribe_finalized(
            &self,
        ) -> Result<
            ::subxt::events::EventSubscription<
                'a,
                ::subxt::events::FinalizedEventSub<'a, T::Header>,
                T,
                Event,
            >,
            ::subxt::BasicError,
        > {
            ::subxt::events::subscribe_finalized::<T, Event>(self.client).await
        }
    }
    pub struct ConstantsApi<'a, T: ::subxt::Config> {
        client: &'a ::subxt::Client<T>,
    }
    impl<'a, T: ::subxt::Config> ConstantsApi<'a, T> {
        pub fn system(&self) -> system::constants::ConstantsApi<'a, T> {
            system::constants::ConstantsApi::new(self.client)
        }
        pub fn timestamp(&self) -> timestamp::constants::ConstantsApi<'a, T> {
            timestamp::constants::ConstantsApi::new(self.client)
        }
        pub fn grandpa(&self) -> grandpa::constants::ConstantsApi<'a, T> {
            grandpa::constants::ConstantsApi::new(self.client)
        }
        pub fn balances(&self) -> balances::constants::ConstantsApi<'a, T> {
            balances::constants::ConstantsApi::new(self.client)
        }
        pub fn transaction_payment(&self) -> transaction_payment::constants::ConstantsApi<'a, T> {
            transaction_payment::constants::ConstantsApi::new(self.client)
        }
    }
    pub struct StorageApi<'a, T: ::subxt::Config> {
        client: &'a ::subxt::Client<T>,
    }
    impl<'a, T> StorageApi<'a, T>
    where
        T: ::subxt::Config,
    {
        pub fn system(&self) -> system::storage::StorageApi<'a, T> {
            system::storage::StorageApi::new(self.client)
        }
        pub fn randomness_collective_flip(
            &self,
        ) -> randomness_collective_flip::storage::StorageApi<'a, T> {
            randomness_collective_flip::storage::StorageApi::new(self.client)
        }
        pub fn timestamp(&self) -> timestamp::storage::StorageApi<'a, T> {
            timestamp::storage::StorageApi::new(self.client)
        }
        pub fn aura(&self) -> aura::storage::StorageApi<'a, T> {
            aura::storage::StorageApi::new(self.client)
        }
        pub fn grandpa(&self) -> grandpa::storage::StorageApi<'a, T> {
            grandpa::storage::StorageApi::new(self.client)
        }
        pub fn balances(&self) -> balances::storage::StorageApi<'a, T> {
            balances::storage::StorageApi::new(self.client)
        }
        pub fn transaction_payment(&self) -> transaction_payment::storage::StorageApi<'a, T> {
            transaction_payment::storage::StorageApi::new(self.client)
        }
        pub fn sudo(&self) -> sudo::storage::StorageApi<'a, T> {
            sudo::storage::StorageApi::new(self.client)
        }
        pub fn template_module(&self) -> template_module::storage::StorageApi<'a, T> {
            template_module::storage::StorageApi::new(self.client)
        }
        pub fn ibc(&self) -> ibc::storage::StorageApi<'a, T> {
            ibc::storage::StorageApi::new(self.client)
        }
    }
    pub struct TransactionApi<'a, T: ::subxt::Config, X> {
        client: &'a ::subxt::Client<T>,
        marker: ::core::marker::PhantomData<X>,
    }
    impl<'a, T, X> TransactionApi<'a, T, X>
    where
        T: ::subxt::Config,
        X: ::subxt::extrinsic::ExtrinsicParams<T>,
    {
        pub fn system(&self) -> system::calls::TransactionApi<'a, T, X> {
            system::calls::TransactionApi::new(self.client)
        }
        pub fn timestamp(&self) -> timestamp::calls::TransactionApi<'a, T, X> {
            timestamp::calls::TransactionApi::new(self.client)
        }
        pub fn grandpa(&self) -> grandpa::calls::TransactionApi<'a, T, X> {
            grandpa::calls::TransactionApi::new(self.client)
        }
        pub fn balances(&self) -> balances::calls::TransactionApi<'a, T, X> {
            balances::calls::TransactionApi::new(self.client)
        }
        pub fn sudo(&self) -> sudo::calls::TransactionApi<'a, T, X> {
            sudo::calls::TransactionApi::new(self.client)
        }
        pub fn template_module(&self) -> template_module::calls::TransactionApi<'a, T, X> {
            template_module::calls::TransactionApi::new(self.client)
        }
        pub fn ibc(&self) -> ibc::calls::TransactionApi<'a, T, X> {
            ibc::calls::TransactionApi::new(self.client)
        }
    }
}
