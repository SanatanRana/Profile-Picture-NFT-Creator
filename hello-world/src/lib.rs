#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, Vec, Symbol};

#[contracttype]
#[derive(Clone)]
pub struct NFT {
    pub token_id: u64,
    pub owner: Address,
    pub image_url: String,
    pub name: String,
    pub bio: String,
}

#[contracttype]
pub enum NFTKey {
    Token(u64),
    Count,
    OwnerTokens(Address),
}

#[contract]
pub struct ProfilePictureNFTCreator;

#[contractimpl]
impl ProfilePictureNFTCreator {
    // Mint a new Profile Picture NFT
    pub fn mint_nft(
        env: Env,
        owner: Address,
        image_url: String,
        name: String,
        bio: String,
    ) -> u64 {
        let mut count = env.storage().instance().get(&NFTKey::Count).unwrap_or(0);
        count += 1;

        let nft = NFT {
            token_id: count,
            owner: owner.clone(),
            image_url,
            name,
            bio,
        };

        env.storage().instance().set(&NFTKey::Token(count), &nft);
        env.storage().instance().set(&NFTKey::Count, &count);

        count
    }

    // Retrieve a specific NFT
    pub fn get_nft(env: Env, token_id: u64) -> NFT {
        env.storage()
            .instance()
            .get(&NFTKey::Token(token_id))
            .expect("NFT not found")
    }

    // Transfer ownership of an NFT
    pub fn transfer_nft(env: Env, token_id: u64, new_owner: Address) {
        let mut nft: NFT = env
            .storage()
            .instance()
            .get(&NFTKey::Token(token_id))
            .expect("NFT not found");

        nft.owner = new_owner;
        env.storage().instance().set(&NFTKey::Token(token_id), &nft);
    }
}
