use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AssetGate {
    #[serde(rename = "hasContract")]
    ///True when the given wallet owns any token from a given contract.
    pub has_contract: bool,
    #[serde(rename = "hasToken")]
    ///Only true when the given wallet owns a particular token from a given contract.
    pub has_token: bool,
}
impl std::fmt::Display for AssetGate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AutoSuggestion {
    ///A hex string representing the address of the entity
    pub address: Option<String>,
    ///Media metadata corresponding to the suggestion
    pub previews: Option<Vec<MediaPreview>>,
    ///A URL-encoded string that can be used in conjunction with the search endpoint
    pub query: Option<String>,
    ///The suggestion result's title fully spelled out
    pub title: Option<String>,
    #[serde(rename = "tokenID")]
    ///For tokens, the unique identifier within the contract
    pub token_id: Option<String>,
    #[serde(rename = "type")]
    ///The type of result the suggestion is based on, e.g. Bored Ape Yacht Club would be a collection.
    pub type_: Option<String>,
}
impl std::fmt::Display for AutoSuggestion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BlockchainInfo {
    #[serde(rename = "chainID")]
    ///A CAIP-2 compliant chain ID. This ID is designed to provide a unique identifier for a given chain.
    pub chain_id: Option<String>,
    ///The name of a given chain. This is often a canonical name since a protocol may have multiple chains.
    pub name: Option<String>,
    #[serde(rename = "shortChainID")]
    ///An ID used for a chain in a given ecosystem. This ID is not canonical and may collide with other chains. Do not use this value as a unique identifier.
    pub short_chain_id: Option<String>,
    #[serde(rename = "shortName")]
    ///The short-hand name for a given chain. Multiple chains (e.g. mainnet and a testnet) may have the same name.
    pub short_name: Option<String>,
}
impl std::fmt::Display for BlockchainInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Collection {
    #[serde(rename = "bannerImage")]
    pub banner_image: Option<Media>,
    pub blockchain: BlockchainInfo,
    #[serde(rename = "contractAddress")]
    ///Address of the contract that minted this NFT.
    pub contract_address: String,
    #[serde(rename = "createdDate")]
    ///Timestamp of creation of this contract in RFC 3339.
    pub created_date: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "featuredImage")]
    pub featured_image: Option<Media>,
    #[serde(rename = "floorPrice")]
    pub floor_price: Option<CurrencyInfo>,
    ///When the value is true, it indicates that this collection, or the collection this NFT belongs to, has been hidden on OpenSea.
    pub hidden: Option<bool>,
    #[serde(rename = "logoImage")]
    pub logo_image: Option<Media>,
    pub name: Option<String>,
    #[serde(rename = "openSeaCollectionVerified")]
    ///Whether or not the collection is verified on OpenSea.
    pub open_sea_collection_verified: Option<bool>,
    #[serde(rename = "shortDescription")]
    pub short_description: Option<String>,
    pub slug: Option<String>,
    #[serde(rename = "socialMedia")]
    pub social_media: Option<Vec<SocialMedia>>,
    pub symbol: Option<String>,
    #[serde(rename = "totalSupply")]
    ///Number of unique tokens in the collection.
    pub total_supply: Option<i64>,
    pub urls: Option<Vec<Url>>,
}
impl std::fmt::Display for Collection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ContractProtocol(pub String);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CreditEvent {
    ///Human-readable, machine parsable, event descriptions for credit-related contract events
    pub event: Option<String>,
    ///Brand or identifier closely associated with a contract
    pub protocol: Option<String>,
}
impl std::fmt::Display for CreditEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CurrencyInfo {
    #[serde(rename = "contractAddress")]
    pub contract_address: Option<String>,
    ///The base value for a given token. For Ethereum this would be 18. This would yield an equation such as 890000000000000000wei / 10^18 = 0.89 ETH.
    pub decimals: i64,
    ///Value of token in fiat currency using an exchange rate from the last 24 hours.
    pub fiat: Option<Vec<CurrencyInfo>>,
    #[serde(rename = "historicalFiat")]
    ///Value of token in fiat currency using an exchange rate from the day of the transaction.
    pub historical_fiat: Option<Vec<CurrencyInfo>>,
    pub name: Option<String>,
    ///A formatted, human-friendly representation of the transaction value, e.g. the value in Ether, possibly with localized thousands separator and radix character.
    pub pretty: String,
    pub symbol: Option<String>,
    #[serde(rename = "symbolLogos")]
    ///Logo images associated with the token for a given currency.
    pub symbol_logos: Option<Vec<MediaPreview>>,
    #[serde(rename = "tokenValue")]
    ///Transaction value as a float representing the amount of tokens, e.g. 0.89 would be 89% of a token.
    pub token_value: f64,
    #[serde(rename = "updateTime")]
    ///The time the fiat was quoted to obtain this token's equivalent value.
    pub update_time: Option<String>,
    ///Transaction value as a string-encoded bigint in the base unit for a given blockchain (e.g. wei for Ethereum) or fiat currency (e.g. US Dollars).
    pub value: String,
}
impl std::fmt::Display for CurrencyInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Ens {
    ///Date the ENS registration expires.
    pub expires: Option<String>,
    #[serde(rename = "isPrimary")]
    ///A wallet address can have multiple ENS records. The primary ENS name represents a "cross-platform web3 username and profile." A wallet address can only have one primary name, and it can change at any time.
    pub is_primary: Option<bool>,
    ///The ENS or domain name.
    pub name: Option<String>,
    #[serde(rename = "pointsTo")]
    ///The address the ENS record points to.
    pub points_to: Option<String>,
}
impl std::fmt::Display for Ens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Erc20Metadata {
    #[serde(rename = "currentPrice")]
    pub current_price: Option<CurrencyInfo>,
    ///The number of decimals the token uses
    pub decimals: Option<i64>,
    ///The name of the erc-20 token
    pub name: Option<String>,
    ///The symbol of the erc-20 token
    pub symbol: Option<String>,
    #[serde(rename = "symbolLogos")]
    ///Logo images associated with the token for a given currency.
    pub symbol_logos: Option<Vec<MediaPreview>>,
    #[serde(rename = "totalSupply")]
    ///The total supply as a string-encoded bigint.
    pub total_supply: Option<String>,
}
impl std::fmt::Display for Erc20Metadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ErrorMessage {
    pub error: String,
}
impl std::fmt::Display for ErrorMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ExchangeEvent {
    ///Human-readable, machine parsable, event descriptions for token exchange-related contract events
    pub event: Option<String>,
    ///Brand or identifier closely associated with a contract
    pub protocol: Option<String>,
}
impl std::fmt::Display for ExchangeEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Hidden(pub bool);
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Media {
    #[serde(rename = "URI")]
    ///URI for the image asset.
    pub uri: Option<String>,
    pub key: Option<String>,
    pub version: Option<Vec<MediaVersion>>,
}
impl std::fmt::Display for Media {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MediaPreview {
    #[serde(rename = "URI")]
    ///URI for the image asset.
    pub uri: Option<String>,
    ///If a video, length of video in ISO 8601 duration format.
    pub duration: Option<String>,
    pub format: Option<String>,
    ///The image's approximate height in pixels.
    pub height: Option<i64>,
    pub kind: Option<String>,
    ///The image's approximate width in pixels.
    pub width: Option<i64>,
}
impl std::fmt::Display for MediaPreview {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MediaVersion {
    ///If a video, length of video in ISO 8601 duration format.
    pub duration: Option<String>,
    pub format: Option<String>,
    pub height: Option<i64>,
    pub kind: Option<String>,
    #[serde(rename = "numBytes")]
    pub num_bytes: Option<i64>,
    pub width: Option<i64>,
}
impl std::fmt::Display for MediaVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NftTransaction {
    #[serde(rename = "fromAddress")]
    pub from_address: Option<String>,
    #[serde(rename = "logLine")]
    ///Deprecated: do not use. Use `nft.lastSoldPrice` instead.
    pub log_line: Option<Vec<NftTransactionLogLine>>,
    ///Timestamp of the transaction in RFC 3339.
    pub timestamp: Option<String>,
    #[serde(rename = "toAddress")]
    pub to_address: Option<String>,
    #[serde(rename = "transactionHash")]
    ///TransactionHash for this transaction.
    pub transaction_hash: Option<String>,
}
impl std::fmt::Display for NftTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NftTransactionLogLine {
    pub price: Option<CurrencyInfo>,
}
impl std::fmt::Display for NftTransactionLogLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NonFungibleToken {
    pub attributes: Option<Vec<TokenAttribute>>,
    pub collection: Option<Collection>,
    #[serde(rename = "contractTitle")]
    pub contract_title: Option<String>,
    #[serde(rename = "creatorAddress")]
    ///The wallet address for the creator of this NFT.
    pub creator_address: Option<String>,
    ///A description of a contract may contain markup such as HTML or Markdown.
    pub description: Option<String>,
    ///A rough heuristic indicating the underlying technology and hypothetical durability of an asset. "On-Chain" assets store metadata and the asset media (e.g. SVG file, source code or MIDI) on a blockchain. "Distributed" assets are stored on decentralized protocols such as IPFS. "Web" assets are stored on services such as Amazon's S3. Each technology has different tradeoffs, in particular certain projects need more flexibility than is provided by on-chain data."
    pub durability: Option<String>,
    ///When the value is true, it indicates that this collection, or the collection this NFT belongs to, has been hidden on OpenSea.
    pub hidden: Option<bool>,
    #[serde(rename = "lastSoldPrice")]
    pub last_sold_price: Option<CurrencyInfo>,
    pub media: Option<Media>,
    pub owner: Option<Wallet>,
    ///Transformed media assets that are resized and transcoded in to friendlier formats for web and mobile delivery.
    pub previews: Option<Vec<MediaPreview>>,
    #[serde(rename = "projectName")]
    ///The project name of the NFT.
    pub project_name: Option<String>,
    pub purchase: Option<NftTransaction>,
    pub title: Option<String>,
    #[serde(rename = "tokenID")]
    ///ID of this particular token.
    pub token_id: String,
}
impl std::fmt::Display for NonFungibleToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct OpenSeaContract {
    #[serde(rename = "assetContractType")]
    pub asset_contract_type: Option<String>,
    #[serde(rename = "buyerFeeBasisPoints")]
    ///Total fee levied on buyers by this contract, in basis points
    pub buyer_fee_basis_points: Option<i64>,
    #[serde(rename = "defaultToFiat")]
    pub default_to_fiat: Option<bool>,
    #[serde(rename = "nftVersion")]
    ///The NFT Version
    pub nft_version: Option<String>,
    #[serde(rename = "openSeaVersion")]
    ///The OpenSea Version
    pub open_sea_version: Option<String>,
    #[serde(rename = "sellerFeeBasisPoints")]
    ///Total fee levied on sellers by this contract, in basis points
    pub seller_fee_basis_points: Option<i64>,
}
impl std::fmt::Display for OpenSeaContract {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct OwnedCollection {
    pub collection: Option<Collection>,
    ///A sample of the tokens owned by this wallet.
    pub tokens: Option<Vec<Token>>,
    #[serde(rename = "totalSpent")]
    pub total_spent: Option<CurrencyInfo>,
}
impl std::fmt::Display for OwnedCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SearchDocument {
    pub collection: Option<Collection>,
    pub token: Option<Token>,
    pub wallet: Option<Wallet>,
}
impl std::fmt::Display for SearchDocument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SocialMedia {
    pub network: Option<String>,
    pub url: Option<String>,
    pub username: Option<String>,
}
impl std::fmt::Display for SocialMedia {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    pub blockchain: BlockchainInfo,
    #[serde(rename = "contractAddress")]
    ///Address of the contract that minted this NFT.
    pub contract_address: String,
    pub nft: Option<NonFungibleToken>,
    #[serde(rename = "openSeaContract")]
    pub open_sea_contract: Option<OpenSeaContract>,
    pub symbol: Option<String>,
    #[serde(rename = "tokenStandard")]
    pub token_standard: Option<String>,
}
impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TokenAttribute {
    #[serde(rename = "traitType")]
    ///The key or kind of trait.
    pub trait_type: Option<String>,
    ///A distinct attribute belonging to a particular type of trait.
    pub value: Option<String>,
}
impl std::fmt::Display for TokenAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TokenEvents {
    ///A list of the events for a token.
    pub events: Option<Vec<Transaction>>,
    ///Metadata for the token specified in the request.
    pub token: Option<Vec<Token>>,
}
impl std::fmt::Display for TokenEvents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Transaction {
    #[serde(rename = "blockNumber")]
    ///Block number of the transaction.
    pub block_number: Option<i64>,
    #[serde(rename = "fromAddress")]
    pub from_address: Option<String>,
    ///Amount of gas used
    pub gas: Option<i64>,
    #[serde(rename = "gasFee")]
    pub gas_fee: Option<CurrencyInfo>,
    #[serde(rename = "logLine")]
    ///Log lines related to the transaction.
    pub log_line: Option<Vec<TransactionLogLine>>,
    #[serde(rename = "spenderAddress")]
    ///The wallet or contract address allowed to spend the approved amount of tokens
    pub spender_address: Option<String>,
    ///Timestamp of the transaction in RFC 3339.
    pub timestamp: Option<String>,
    #[serde(rename = "toAddress")]
    pub to_address: Option<String>,
    #[serde(rename = "tokenID")]
    ///A token ID if the transaction was related to an NFT.
    pub token_id: Option<String>,
    #[serde(rename = "transactionHash")]
    ///TransactionHash for this transaction.
    pub transaction_hash: Option<String>,
    #[serde(rename = "transactionIndex")]
    ///Index of the transaction in the block.
    pub transaction_index: Option<i64>,
    pub value: Option<CurrencyInfo>,
}
impl std::fmt::Display for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TransactionLogLine {
    ///When the value is true, it indicates that a token has been burned, and this transaction corresponds to the burn.
    pub burned: Option<bool>,
    #[serde(rename = "eventName")]
    ///Contract topic0 signature related to the log line.
    pub event_name: Option<String>,
    #[serde(rename = "logAddress")]
    ///Protocol contract address that generated the given log entry.
    pub log_address: Option<String>,
    #[serde(rename = "logAddressProtocol")]
    ///Protocol name that generated the given log entry.
    pub log_address_protocol: Option<String>,
    #[serde(rename = "logData")]
    ///Raw hex data from contract call.
    pub log_data: Option<String>,
    #[serde(rename = "logIndex")]
    ///Index of the log in the transaction.
    pub log_index: Option<i64>,
    #[serde(rename = "logTopics")]
    ///Topic signatures.
    pub log_topics: Option<Vec<String>>,
    ///When the value is true, it indicates this is the transaction representing a token minting.
    pub mint: Option<bool>,
    #[serde(rename = "transactionIndex")]
    ///Transaction index of the log in the transaction.
    pub transaction_index: Option<i64>,
}
impl std::fmt::Display for TransactionLogLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Url {
    ///Human-readable name of the site.
    pub name: Option<String>,
    pub url: Option<String>,
}
impl std::fmt::Display for Url {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Wallet {
    pub address: String,
    ///The balance of fungible tokens from smart contracts or blockchains. Certain non-standard ERC-20 tokens (e.g. stETH) may occasionally have stale balances, for example interest payments can be slow to update.
    pub balances: Option<Vec<CurrencyInfo>>,
    ///A sample of the collections this wallet owns.
    pub collections: Option<Vec<OwnedCollection>>,
    pub ens: Option<Vec<Ens>>,
    ///A reverse-chronological list of the latest transactions for a wallet.
    pub history: Option<Vec<Transaction>>,
    #[serde(rename = "recentTokens")]
    ///A sample of the most recently transferred tokens owned by this wallet.
    pub recent_tokens: Option<Vec<Token>>,
    #[serde(rename = "socialMedia")]
    pub social_media: Option<Vec<SocialMedia>>,
    #[serde(rename = "topTokens")]
    ///A sample of the most expensive tokens owned by this wallet.
    pub top_tokens: Option<Vec<Token>>,
    #[serde(rename = "totalBalance")]
    ///The total balance in a fiat currency (e.g. USD) for the wallet.
    pub total_balance: Option<Vec<CurrencyInfo>>,
}
impl std::fmt::Display for Wallet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
