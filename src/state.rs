use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{borsh::try_from_slice_unchecked, native_token::LAMPORTS_PER_SOL, program_error::ProgramError};

use crate::error::InglError;

#[derive(BorshSerialize, BorshDeserialize)]
pub enum Class {
    Ruby,
    Diamond,
    Sapphire,
    Emerald,
    Serendibite,
    Benitoite
}

impl Class {
    pub fn get_class_lamports(self)->u64{
        match self {
            Self::Ruby => {500 * LAMPORTS_PER_SOL}
            Self::Diamond => {100 * LAMPORTS_PER_SOL}
            Self::Sapphire => {50 * LAMPORTS_PER_SOL}
            Self::Emerald => {10 * LAMPORTS_PER_SOL}
            Self::Serendibite => {5 * LAMPORTS_PER_SOL}
            Self::Benitoite => {1 * LAMPORTS_PER_SOL}
        }
    }
}


#[derive(BorshDeserialize, BorshSerialize)]
pub enum Rarity{
    Common,
    Uncommon,
    Rare,
    Exalted,
    Mythic,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct GemAccountV0_0_1{
    struct_id: GemAccountVersions,
    date_created: i64,
    redeemable_data: i64,
    numeration: u32,
    rarity: Rarity,

}

#[derive(BorshDeserialize, BorshSerialize)]
pub enum GemAccountVersions{
    GemAccountV0_0_1,
    BlankCase,
}
impl GemAccountVersions{
    pub fn decode<T: BorshDeserialize>(data: &[u8]) -> Result<T, ProgramError>{
        let version: GemAccountVersions = try_from_slice_unchecked(&data[0..1]).unwrap();
        match version{
            GemAccountVersions::GemAccountV0_0_1 => {    
                //Change the code here to represent the conversion to the appropriate version you expect.
                let a: T = try_from_slice_unchecked(data).unwrap();
                Ok(a)
            }
            // GemAccountVersions::AnotherOption => {
                //  Do Something in Here to convert data to the appropriate struct to return
            // }
            _ => {
                Err(InglError::InvalidStructType.utilize("GemAccountVerssions deserialize"))}
        }

    }
}


