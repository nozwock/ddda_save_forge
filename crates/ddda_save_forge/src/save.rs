// TODO: Equipped skills, Learned Skills, Inclinations

// `class name mPlayerDataManual`: Non-checkpoint Save
/// `class type sSave::playerData`
pub struct SaveData {
    pub pl_cmc_edit_and_param: PlayerPawnEditAndParam,
    pub pl_game_data: PlayerGameData,
}

/// `class name PlCmcEditAndParam`
pub struct PlayerPawnEditAndParam {
    pub player: SaveDataPlayer,   // class name mPl
    pub pawns: [SaveDataPawn; 3], // array name mCmc
    pub item: [SaveDataItem; 4],  // array name mItem
} // class type sSave::playerData > class name PlCmcEditAndParam

/// `class name mPlGameData`
pub struct PlayerGameData {
    pub storage: SaveDataStorageItem,
} // class type sSave::playerData > class name mPlGameData

/// `class type cSAVE_DATA_PL`
pub struct SaveDataPlayer {
    pub param: SaveDataParam,
} // class name PlCmcEditAndParam > class name mPl > class type cSAVE_DATA_PL

/// `class type cSAVE_DATA_CMC`
pub struct SaveDataPawn {
    pub param: SaveDataParam,
} // class name PlCmcEditAndParam > class name mCmc > class type cSAVE_DATA_CMC

/// `class type cSAVE_DATA_ITEM`\
/// Player/Pawn Inventory Items
pub struct SaveDataItem {
    pub item_count: u32, // u32 name mItemCount
    pub item_capacity: u32,
    pub item: Vec<ItemParamData>, // array name mItem
}

pub struct SaveDataStorageItem {
    pub item_count: u32, // u32 name mStorageItemCount
    pub storage_item_capacity: u32,
    pub storage_item: Vec<ItemParamData>, // array name mStorageItem
} // Transparent container

pub enum ItemQualityFlag {
    Raw = 3,
    Star1 = 13,
    Star2 = 19,
    Star3 = 35,
    DragonForged = 67,
    SilverRarefied = 515,
    GoldRarefied = 1027,
}

/// `class type sItemManager::cITEM_PARAM_DATA`
pub struct ItemParamData {
    pub count: i16,        // s16 name data.mNum
    pub item_id: i16,      // s16 name data.mItemNo
    pub quality_flag: u32, // u32 name data.Flag
    pub chg_num: u16,      // u16 name data.mChgNum
    pub day1: u16,         // u16 name data.mDay1
    pub day2: u16,         // u16 name data.mDay2
    pub day3: u16,         // u16 name data.mDay3
    pub mutation_pool: i8, // s8 name data.mMutationPool
    pub owner_id: i8,      // s8 name data.mOwnerId
    pub key: u32,          // u32 name data.mKey
}

/// `class name mParam`\
/// Player/Pawn parameters
pub struct SaveDataParam {
    pub gold: i32,                       // s32 name mGold
    pub rift_crystals: i32,              // s32 name OnlinePoint
    pub level: u8,                       // u8 name mLevel
    pub exp: u32,                        // u32 name mExp
    pub next_exp: u32,                   // u32 name mNextExp
    pub hp: f32,                         // f32 name mHp
    pub max_hp: f32,                     // f32 name mHpMax
    pub max_hp_white: f32,               // f32 name mHpMaxWhite
    pub stamina: f32,                    // f32 name mStamina
    pub stamina_base: f32,               // f32 name mStaminaBase
    pub stamina_from_lv: f32,            // f32 name mStaminaLv
    pub base_strength: f32,              // f32 name mBasicAttack
    pub base_defense: f32,               // f32 name mBasicDefend
    pub base_magick: f32,                // f32 name mBasicMgcAttack
    pub base_magick_defense: f32,        // f32 name mBasicMgcDefend
    pub current_vocation: u8,            // u8 name mJob
    pub vocation_level: [u8; 10],        // array name mJobLevel
    pub discipline_point: [i32; 10],     // array name mJobPoint
    pub equip_item: [ItemParamData; 12], // array name mEquipItem
} // class name PlCmcEditAndParam > * > class name mParam
