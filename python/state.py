from borsh_construct import *
from solana.publickey import PublicKey
class Constants:
    INGL_NFT_COLLECTION_KEY = "ingl_nft_collection_newer"
    INGL_MINT_AUTHORITY_KEY = "mint_authority"
    INGL_MINTING_POOL_KEY = "minting_pool"
    COLLECTION_HOLDER_KEY = "collection_holder"
    INGL_PROGRAM_ID = PublicKey("xEm9etyopoopNE6ZdEnp2mjks4XmMAxJb8T9YqP6q5K")
    GLOBAL_GEM_KEY = "global_gem_account"
    GEM_ACCOUNT_CONST = "gem_account"

ClassEnum = Enum(
    "Ruby",
    "Diamond",
    "Sapphire",
    "Emerald",
    "Serendibite",
    "Benitoite",

    enum_name = "ClassEnum",
)