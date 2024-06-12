pub mod state {
    use candid::CandidType;
    use ic_stable_structures::StableBTreeMap;
    use junobuild_collections::types::rules::Rules;
    use junobuild_shared::types::core::{Blob, CollectionKey};
    use junobuild_shared::types::memory::Memory;
    use junobuild_storage::types::config::StorageConfig;
    use junobuild_storage::types::domain::CustomDomains;
    use junobuild_storage::types::state::FullPath;
    use junobuild_storage::types::store::{Asset, EncodingType};
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    pub type AssetsStable = StableBTreeMap<StableKey, Asset, Memory>;
    pub type ContentChunksStable = StableBTreeMap<StableEncodingChunkKey, Blob, Memory>;

    pub type AssetsHeap = HashMap<FullPath, Asset>;

    #[derive(CandidType, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct StableKey {
        pub collection: CollectionKey,
        pub full_path: FullPath,
    }

    #[derive(CandidType, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct StableEncodingChunkKey {
        pub full_path: FullPath,
        pub encoding_type: EncodingType,
        pub chunk_index: usize,
    }

    #[derive(Default, CandidType, Serialize, Deserialize, Clone)]
    pub struct StorageHeapState {
        pub assets: AssetsHeap,
        pub rules: Rules,
        pub config: StorageConfig,
        pub custom_domains: CustomDomains,
    }
}
