use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap};
use near_sdk::json_types::U128;
use near_sdk::{env, near_bindgen, AccountId, Balance, PanicOnDefault, StorageUsage};

pub mod ft_core;
pub mod metadata;
pub mod storage;
pub mod internal;
pub mod events;

use crate::metadata::*;
use crate::events::*;

/// The image URL for the default icon
const DATA_IMAGE_SVG_GT_ICON: &str = "data:image/jpeg;base64,/9j/4AAQSkZJRgABAQIAJQAlAAD/4gHbSUNDX1BST0ZJTEUAAQEAAAHLAAAAAAJAAABtbnRyUkdCIFhZWiAAAAAAAAAAAAAAAABhY3NwAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAQAA9tYAAQAAAADTLVF0BQ8AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAlyWFlaAAAA8AAAABRnWFlaAAABBAAAABRiWFlaAAABGAAAABR3dHB0AAABLAAAABRjcHJ0AAABQAAAAAxyVFJDAAABTAAAACBnVFJDAAABTAAAACBiVFJDAAABTAAAACBkZXNjAAABbAAAAF9YWVogAAAAAAAAb58AADj0AAADkVhZWiAAAAAAAABilgAAt4cAABjcWFlaIAAAAAAAACShAAAPhQAAttNYWVogAAAAAAAA808AAQAAAAEWwnRleHQAAAAATi9BAHBhcmEAAAAAAAMAAAACZmYAAPKnAAANWQAAE9AAAApbZGVzYwAAAAAAAAAFc1JHQgAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAD/2wBDAAoHBwgHBgoICAgLCgoLDhgQDg0NDh0VFhEYIx8lJCIfIiEmKzcvJik0KSEiMEExNDk7Pj4+JS5ESUM8SDc9Pjv/2wBDAQoLCw4NDhwQEBw7KCIoOzs7Ozs7Ozs7Ozs7Ozs7Ozs7Ozs7Ozs7Ozs7Ozs7Ozs7Ozs7Ozs7Ozs7Ozs7Ozs7Ozv/wAARCABmAGYDASIAAhEBAxEB/8QAGwAAAQUBAQAAAAAAAAAAAAAABQABAwQGAgf/xABBEAACAQMDAgQDBAQMBwEAAAABAgMEBREAEiEGMRMiQVEUYYEycZGhI1JighUWJDNCU3KSorHB8AcXQ1Rzs8LR/8QAGQEBAQEBAQEAAAAAAAAAAAAABAMCAQAF/8QALREAAgIBAwIEBAcBAAAAAAAAAQIAAxEEEiETMSJBUYEUMkLwI2FxkaGx0cH/2gAMAwEAAhEDEQA/APKhpaWnAzpQmYgM67VM67SPPpq9TULyngaTXSWhrLlQcyksOfTUq0zH01rrP0ZcLku+GnPhjvI/lUfU8aNRdLWWn8tVe4Xcd0o4WqMH28vbSelWvDGBOpd/kH37zzr4N/1dcNSsPTXp38D9NBghmuJYgkfyQjj7vqNRS9N2CcYhvQgc9hWU7wj+8eNexT9gzguvH0/yP9nmTQkemoimNb269D19HD46xCaAjImhYOhHvkdvrrKVVveEnKnWGoBGVORLV6sZ2twYKIxptTvHjUJGNDZSI9WBEbGlpaWsTcdRnU0aZOo0GiFDAZZAANXpr3GHus2LmW7bbXqZVVELFjgADOdb6ltVv6cT+WxJWXEKH+GLYjpx6NK3/wA/7HFuij6YtUdWdguNVGXgLjIpoh3lI/ID1JH0A26O6dWXpaKzSBUCtK7yvnAzhnl4O5jx79x9F2WBRtU4A7mfOrrNh3vznsIQuPUVXdtg2TVMBfYrgeFTqQM7VB8vA9WyQNVpbhVMYS1QPCeURnwF3Bc+m5sJn7uPnri79P3jpqgomapp7naqiY+Ad58JJjkZYHHse/HBzqjbRXX+tioqSmavuJzkzEGGBQf6KjygfM8c8D3mrpjI7SrVvnmF6eezydGz109zlF2RsLEMAZzwNuOQQCc//mqkFwq1VnFTiJFjLNNHhMsoJG5Mgc9s/XGi/wDy9lktMt2/jLQlVzIzJF+iBTI+17DkdtZO5CWgaWluNP4UzpmOel8qyDuMgcFT9wPY866rg5xzOMhBAPEOUvUFXatlYkc1DFM7Is8LBo5GX7WV+yw579/now0ds6pRUdIKK4S/zUkZxT1R9h+o3yP56xUVJHJR1T1zPTTQxb1jVtgUFPK23B3bjtBxjGcn5XXkgpul6O4tWePNK/hSU2AF2KMDGOzDAOf2h9fKecr3nHQEBWg682ae31EkM0TI6HBBHbQORMHXqFPLH1haPhpX8W5QRb6eY/aqYx3Vv21/P/Lz6vpWglZSMYOtOBYu4d/Odpdqm2N7QURpa7dedLQCs+oDxOohkjWn6bpQ9bTgorb5VXDHAxnkn5AZP01nKdcsNa+1xFaOdlOGFOUU+oaVliB/uu+nU+CstPm6nxuqesi6svM1zq5HjjJWfE0gwcRwA4iU47DBBPzfRPp64w/8P7vBUNUpdKWvogKgUpLNDk5B5wPpkd9Zl1uVdXXCWhirHinJgxTBtuARhWwMEY9NT7TaEEj015otxEe74kRBu/rsHGjkBgR5S/ykEd4e6u6yt93sdF0/b1qhRwy+LNVTRAMftYAUHt5j6jtov0xZa20dPzVVNWVdJcJ6D4spFSKUZQD4abip8xwCVHvrMU3j3iritFFUXWWqlqvCcT1++neMAlhuVfUA888a1XU/WNzoqtKKEQxV0OGaKmcvFCP2iQC3HZcADv3xiYXHgWbLD5mmgkpXS3Gr/jZU/Bld28Qw7WB/c5z/AK6yPVdjr6m2U9XVVFXUVTl2RJKZMRquSocqvlJBAwfU/LQBbxdi3xKVdQAsjTDbCvhB+AWC4wOSPqfc60dj6vqK4z266jxKqsjKU8ruUic44UjBCk+4Bz+A1VUZDny85BnWwbR38pjqCloxaquoe47QyRoS1OxAbcDt+fCn8NdW6ShtF4jr3qYqowuzGBqQlGJyOBnH3enA1FXRzUV2aCuWQiCoUxhZS0MILEsOR7g+3Y99GD1FWmhVRc5xOVALmsnJB9Tjt/p9+u45InS2ADiDLPd56e8ePT4jleU1EKqmxUkyTsA/VIyv3HRLqvwq2umrIIhHHMFmjA9VdQwP45BHuDqjer1NNSxI1bLP4ciMPEnkkJYZ5w2MHHsPXRCfbPaoGH/TaaD6ZWVP/a/4apScPj14krxmvf6c/wCzGSrhtLUtSu2Qj56WpuuGMXW2VBjUn2xrY0U0NPbqmWdJHVDSkCM853yH2PqBrGUzYca1lK++21Kjv8MJR98Uisf8Jf8ADVRzSYWzjUL7/wBQfTVoiRo6OeVYVlcgNIQeffGOfpriWV6t46arq2MM1VEMM+do5BPP365stNaz1FPFdwhhCSmIM20PJjyhjkcfUDOpLva7RJW1DW2bZBHGhl8OF5VibHmwwyAM5750L4gCzpY5ivhsjqAzd2m6FrtTUlit8MkNBTSSCnDeFsLbU3sxGWYhjoPZuqoem+pr0t7pHSpqJt26PDle5C544II/DTdHy1El4udLPeZZDS0BQyRxqjbRIqkFipY7cgg/IempurrDS3eqpaWgMk15SIBnZx5o143Sn39AfXOPbFN43EgcSPw/4XTY5/OU5ukrpcIpuoKWGKOkldp0pS+GZM54GMen5afqrrCh6jpbdQ2ukkWtWdNmQBsOcBVx88asUvUHWVvtAso6eqHkRPCScQswUf2h5T9+dQ9KdORWS8o98Qx3CRC9INwMf7WCO7jP0zkZ7iz2DHEPTp3LZt8u0uNdpbf1JUx3OlgjirYxHLTufFSUgg44HB85Ofnqv0xa+n5Lzct8QqTE6ilpiNxwwzjB7nnHm7AHPuA99rJKO6o8VzlQBniFRMglfAVOe2ck45HbGqFHbamkYVtFX1kTFSfHjo5fMDz39tcscKcEc8f1NLp2sDMrYBzx7/xCfWNptVo6ipkqNiR1EZkqKemGPCPIGDg/5Dt2GrCy0ktmzSpMqisUEzHk/oG7cD2Gs9coozEtSlz+OnqSisz7WZwfMcDlk2kAc4znjjOjY/RWmIf1ks0v0UJEv5h9cpO5x+steu2kg+kzFZ/PH79LTVTZlP36Wt2HxGbqHgErwtgjWos9UI1SVl3iFtzJ+uhBV1+qltZRDjRe11ZhlU513TkEFD5yWrQ4Dr3EluUAtdxkWRIqiIL4TmRCwdO6OCCCMjacgj1+enSguFxsNRXQ0NMlspvKdjMFjclSX5bczDgc5wDovUUiXGkWBE3zRIRAv9dFyTH/AGl5K/LI9NZ5JKqnikpY6qTwZXTZCAFimI7hxkYIwPfP56OybWIYS6WdRA1ZhS1VbUXVEtRFX0aQV00tH4jKWUo/dyODtBKnkj8Aca6mjpIj4dKzTOXZ3uIkVn8QcZz6g9sDjH4682EIuTypRUiBlkMmxX/SMpx5VHYgYPYZ50coepfjKZKG6VEkU8aLTxSuSsfhjPD7RuDDsGGfTOOTqaYzzLODjImxnuVM84pamCT+EHG5BC7or443hxwB9/bHY+tC6zxLTOK7+UtsDSVW4KVYdgg7rjvnGPfvjVF7rMsixCqoyrIzblrIyowV9d3z9eeNALjeYodyU8okmVsq0bFo1znJy3JbJ4xwMeuk9NV5YwnV3eFBz9/tIarfVTTFTDVilhJZMkM24Es4x3KnGcH0zjGcEqenH8Fxt8GxPgg7vhpD6e/jD/L6aHU9NTpUxfB18FJI0qMr1TOssB5BU4G0jnOccj27amuAqKStWBY6MqY/Ecy26KNohkjzKAcdsjnkEe+jMxdiYtFCKFlG2RPUvTwxUsXixthGXlpXfG0E59Py50fukscMRjhfdFEqwRN+sqZy37zF2+o0rbRC30wlwYp5oz4QxhoYjw0h9mbkKPQZOhN0qldtiAKijCgdgNM06bRvMBqX6jioe/8AyDJmy5OlqJ2ydLR2bJjlXAxOFONTxSFTnOq+ulONYRtpmmUMMTSW64KU8KQnGQQQcFSOxB9Dq/W0sNxBkkkihnf7UzjENR/bx9h/2ux+WsnFKVPB0Vo7q8XBOR6g6flLhhuDPlmuzTsWr5HmIqilqKCWGlrIW8OJi0cE5x3z9iQemTnvjXCVEkqEXMxORwrVkDMCP/Knm+nI0ZprjGYTCjhYj3gkQSRH909v3caf4OglO5aQwse70dUU/wADggfjqD6dwMAZ/SXTWVE5Y4P5/eICHwvgyOtugaIOMss8m044yM+bHP8AvGngrBHUbaSnihRWIMtFEzueOCHk5XnjIxj20bNro927xLrkeuYD+e7/AE10KCgXBemmnx/3VZgD91F5/HUxQ5+kyh1dI+sfvM9DRz1VQ8Sh5JJvtRxnxZZPNnLHsPTn5dtHaK2Q2/BlWGaeM5EAO6GA+8jf02/ZH11M9dFBC0KukUR7w0qeEh+/B3N9Tj5aE1l13L4cYCIvAVRgDSE02ObDgQzatn8NI9zLFzuX2wJGkdzukkY+Zz7n/fGs9LIWYnTyzFycnVctnWLrgfCvaX02n2cnvGOlptLQ8x0YafOlpa9PRwcakVyNLS1tSZhhJkqGXsTqylfKvYnS0tMR29YOytT3ElFymx3Oo3uErep0tLVS7esitSZ7Su9S7dydQNITpaWiOxPeMrRR2EjLa5zpaWjmJEWdLS0tcnZ//9k=";

/// The specific version of the standard we're using
pub const FT_METADATA_SPEC: &str = "ft-1.0.0";

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    /// Keep track of each account's balances
    pub accounts: LookupMap<AccountId, Balance>,

    /// Total supply of all tokens.
    pub total_supply: Balance,

    /// The bytes for the largest possible account ID that can be registered on the contract
    pub bytes_for_longest_account_id: StorageUsage,

    /// Metadata for the contract itself
    pub metadata: LazyOption<FungibleTokenMetadata>,
}

/// Helper structure for keys of the persistent collections.
#[derive(BorshSerialize)]
pub enum StorageKey {
    Accounts,
    Metadata
}

#[near_bindgen]
impl Contract {
    /// Initializes the contract with the given total supply owned by the given `owner_id` with
    /// default metadata (for example purposes only).
    #[init]
    pub fn new_default_meta(owner_id: AccountId, total_supply: U128) -> Self {
        // Calls the other function "new: with some default metadata and the owner_id & total supply passed in
        Self::new(
            owner_id,
            total_supply,
            FungibleTokenMetadata {
                spec: FT_METADATA_SPEC.to_string(),
                name: "Spudcoin".to_string(),
                symbol: "SPUD".to_string(),
                icon: Some(DATA_IMAGE_SVG_GT_ICON.to_string()),
                reference: None,
                reference_hash: None,
                decimals: 24,
            },
        )
    }

    /// Initializes the contract with the given total supply owned by the given `owner_id` with
    /// the given fungible token metadata.
    #[init]
    pub fn new(
        owner_id: AccountId,
        total_supply: U128,
        metadata: FungibleTokenMetadata,
    ) -> Self {
        // Create a variable of type Self with all the fields initialized.
        let mut this = Self {
            // Set the total supply
            total_supply: total_supply.0,
            // Set the bytes for the longest account ID to 0 temporarily until it's calculated later
            bytes_for_longest_account_id: 0,
            // Storage keys are simply the prefixes used for the collections. This helps avoid data collision
            accounts: LookupMap::new(StorageKey::Accounts.try_to_vec().unwrap()),
            metadata: LazyOption::new(
                StorageKey::Metadata.try_to_vec().unwrap(),
                Some(&metadata),
            )
        };

        // Measure the bytes for the longest account ID and store it in the contract.
        this.measure_bytes_for_longest_account_id();

        // Register the owner's account and set their balance to the total supply.
        this.internal_register_account(&owner_id);

        // Set the owner's balance to the total supply
        this.internal_deposit(&owner_id, total_supply.into());

        // Emit an event
        FtMint {
            owner_id: &owner_id,
            amount: &total_supply,
            memo: Some("Initial token supply is minted"),
        }.emit();

        // Return the Contract object
        this
    }
}