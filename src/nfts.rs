use crate::state::{Rarity, Class};

pub fn get_uri<'life>(class: Class, rarity: Option<Rarity>) -> &'life str{
    match class{
        Class::Ruby => {
            match rarity {
                Some(rarity_type) => {
                    match rarity_type {
                        Rarity::Common => {"RubyCommon"}
                        Rarity::Uncommon => {"RubyUncommon"}
                        Rarity::Rare => {"RubyRare"}
                        Rarity::Exalted => {"RubyExalted"}
                        Rarity::Mythic => {"RubyMythic"}
                    }
                }
                None => {"DefaultRuby"}
            }
        }
        Class::Diamond => {
            match rarity {
                Some(rarity_type) => {
                    match rarity_type {
                        Rarity::Common => {"DiamondCommon"}
                        Rarity::Uncommon => {"DiamondUncommon"}
                        Rarity::Rare => {"DiamondRare"}
                        Rarity::Exalted => {"DiamondExalted"}
                        Rarity::Mythic => {"DiamondMythic"}
                    }
                }
                None => {"DefaultDiamond"}
            }
        }
        Class::Sapphire => {
            match rarity {
                Some(rarity_type) => {
                    match rarity_type {
                        Rarity::Common => {"SapphireCommon"}
                        Rarity::Uncommon => {"SapphireUncommon"}
                        Rarity::Rare => {"SapphireRare"}
                        Rarity::Exalted => {"SapphireExalted"}
                        Rarity::Mythic => {"SapphireMythic"}
                    }
                }
                None => {"DefaultSapphire"}
            }
        }
        Class::Emerald => {
            match rarity {
                Some(rarity_type) => {
                    match rarity_type {
                        Rarity::Common => {"EmeraldCommon"}
                        Rarity::Uncommon => {"EmeraldUncommon"}
                        Rarity::Rare => {"EmeraldRare"}
                        Rarity::Exalted => {"EmeraldExalted"}
                        Rarity::Mythic => {"EmeraldMythic"}
                    }
                }
                None => {"DefaultEmerald"}
            }
        }
        Class::Serendibite => {
            match rarity {
                Some(rarity_type) => {
                    match rarity_type {
                        Rarity::Common => {"SerendibiteCommon"}
                        Rarity::Uncommon => {"SerendibiteUncommon"}
                        Rarity::Rare => {"SerendibiteRare"}
                        Rarity::Exalted => {"SerendibiteExalted"}
                        Rarity::Mythic => {"SerendibiteMythic"}
                    }
                }
                None => {"DefaultSerendibite"}
            }
        }
        Class::Benitoite => {
            match rarity {
                Some(rarity_type) => {
                    match rarity_type {
                        Rarity::Common => {"BenitoiteCommon"}
                        Rarity::Uncommon => {"BenitoiteUncommon"}
                        Rarity::Rare => {"BenitoiteRare"}
                        Rarity::Exalted => {"BenitoiteExalted"}
                        Rarity::Mythic => {"BenitoiteMythic"}
                    }
                }
                None => {"DefaultBenitoite"}
            }
        }
    }
}