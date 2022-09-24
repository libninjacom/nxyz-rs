//! [`NxyzClient`](struct.NxyzClient.html) is the main entry point for this library.
//!
//! Library created with [`libninja`](https://www.libninja.com).
#![allow(non_camel_case_types)]
#![allow(unused)]
pub mod model;
pub mod request;
use crate::model::*;

pub struct NxyzClient {
    pub(crate) client: httpclient::Client,
    authentication: Option<NxyzAuthenticator>,
}
impl NxyzClient {
    pub fn from_env() -> Self {
        let url = std::env::var("NXYZ_BASE_URL")
            .expect("Missing environment variable NXYZ_BASE_URL");
        Self::new(&url)
    }
}
impl NxyzClient {
    pub fn new(url: &str) -> Self {
        let client = httpclient::Client::new(Some(url.to_string()));
        let authentication = None;
        Self { client, authentication }
    }
    pub fn with_authentication(mut self, authentication: NxyzAuthenticator) -> Self {
        self.authentication = Some(authentication);
        self
    }
    pub fn authenticate<'a>(
        &self,
        mut r: httpclient::RequestBuilder<'a>,
    ) -> httpclient::RequestBuilder<'a> {
        if let Some(ref authentication) = self.authentication {
            match authentication {
                NxyzAuthenticator::Apikey { apikey } => {
                    r = r.push_query("apikey", apikey);
                }
            }
        }
        r
    }
    pub fn with_middleware<M: httpclient::Middleware + 'static>(
        mut self,
        middleware: M,
    ) -> Self {
        self.client = self.client.with_middleware(middleware);
        self
    }
    /**Autocomplete collections

Get autocomplete-style search suggestions for collections.*/
    pub fn get_collections_suggestions_results(
        &self,
    ) -> request::GetCollectionsSuggestionsResultsRequest {
        request::GetCollectionsSuggestionsResultsRequest {
            client: &self,
            query: None,
        }
    }
    /**Get top collections

Returns trending and interesting collections on Ethereum. Useful for powering discovery experiences and providing an on-ramp to exploring creative blockchain data.*/
    pub fn get_top_collections(&self) -> request::GetTopCollectionsRequest {
        request::GetTopCollectionsRequest {
            client: &self,
        }
    }
    /**Submit a request for metadata to be refreshed.

The metadata will be refreshed offline and later updated.*/
    pub fn post_refresh(
        &self,
        contract_address: &str,
        token_id: &str,
    ) -> request::PostRefreshRequest {
        request::PostRefreshRequest {
            client: &self,
            contract_address: contract_address.to_owned(),
            token_id: token_id.to_owned(),
            chain_id: None,
            media: None,
        }
    }
    /**Autocomplete tokens

Get autocomplete-style search suggestions for tokens.*/
    pub fn get_tokens_suggestions_results(
        &self,
    ) -> request::GetTokensSuggestionsResultsRequest {
        request::GetTokensSuggestionsResultsRequest {
            client: &self,
            query: None,
        }
    }
    /**Get top tokens

Returns trending and interesting NFTs and SFTs on Ethereum. Useful for powering discovery experiences and providing an on-ramp to exploring creative blockchain data.*/
    pub fn get_top_tokens(&self) -> request::GetTopTokensRequest {
        request::GetTopTokensRequest {
            client: &self,
        }
    }
    /**Autocomplete wallets

Get autocomplete-style search suggestions for wallets.*/
    pub fn get_wallets_suggestions_results(
        &self,
    ) -> request::GetWalletsSuggestionsResultsRequest {
        request::GetWalletsSuggestionsResultsRequest {
            client: &self,
            query: None,
        }
    }
    /**Get top wallets

Returns trending and interesting wallets on Ethereum. Useful for powering discovery experiences and providing an on-ramp to exploring creative blockchain data.*/
    pub fn get_top_wallets(&self) -> request::GetTopWalletsRequest {
        request::GetTopWalletsRequest {
            client: &self,
        }
    }
    /**Get supported blockchains

Lists all supported blockchains.*/
    pub fn get_blockchains(&self) -> request::GetBlockchainsRequest {
        request::GetBlockchainsRequest {
            client: &self,
        }
    }
    /**Get collection

Get a collection by its contract address.*/
    pub fn get_collection(
        &self,
        contract_address: &str,
    ) -> request::GetCollectionRequest {
        request::GetCollectionRequest {
            client: &self,
            contract_address: contract_address.to_owned(),
            chain_id: None,
        }
    }
    /**Get collection transactions

Get the transaction history for a collection.*/
    pub fn get_contract_transaction_history(
        &self,
        contract_address: &str,
    ) -> request::GetContractTransactionHistoryRequest {
        request::GetContractTransactionHistoryRequest {
            client: &self,
            contract_address: contract_address.to_owned(),
            cursor: None,
            chain_id: None,
            limit: None,
        }
    }
    /**Search

Get search results such as wallets, tokens, and collections by a search query.*/
    pub fn get_search_results(&self, query: &str) -> request::GetSearchResultsRequest {
        request::GetSearchResultsRequest {
            client: &self,
            query: query.to_owned(),
            cursor: None,
            limit: None,
        }
    }
    /**Autocomplete

Get autocomplete-style search suggestions for results.*/
    pub fn get_suggestions_results(&self) -> request::GetSuggestionsResultsRequest {
        request::GetSuggestionsResultsRequest {
            client: &self,
            query: None,
        }
    }
    /**Batch token lookup

Returns tokens from a batch lookup.*/
    pub fn get_contract_tokens_by_contract_and_id(
        &self,
        contract_addresses: &str,
        token_identifiers: &str,
    ) -> request::GetContractTokensByContractAndIdRequest {
        request::GetContractTokensByContractAndIdRequest {
            client: &self,
            contract_addresses: contract_addresses.to_owned(),
            token_identifiers: token_identifiers.to_owned(),
            chain_id: None,
            limit: None,
        }
    }
    /**Get ERC-20 metadata

Get ERC-20 metadata by contract address.*/
    pub fn get_erc20_metadata(
        &self,
        contract_address: &str,
    ) -> request::GetErc20MetadataRequest {
        request::GetErc20MetadataRequest {
            client: &self,
            contract_address: contract_address.to_owned(),
            chain_id: None,
        }
    }
    /**Search tokens

Get tokens by a search query.*/
    pub fn get_tokens_by_search_query(
        &self,
        query: &str,
    ) -> request::GetTokensBySearchQueryRequest {
        request::GetTokensBySearchQueryRequest {
            client: &self,
            query: query.to_owned(),
            chain_id: None,
            limit: None,
            cursor: None,
        }
    }
    /**Get tokens

Get tokens by contract address.*/
    pub fn get_contract_tokens(
        &self,
        contract_address: &str,
    ) -> request::GetContractTokensRequest {
        request::GetContractTokensRequest {
            client: &self,
            contract_address: contract_address.to_owned(),
            chain_id: None,
            limit: None,
        }
    }
    /**Get token

Get a token by its contract address and token ID.*/
    pub fn get_token(
        &self,
        contract_address: &str,
        token_id: &str,
    ) -> request::GetTokenRequest {
        request::GetTokenRequest {
            client: &self,
            contract_address: contract_address.to_owned(),
            token_id: token_id.to_owned(),
            chain_id: None,
        }
    }
    /**Get token transfers

Returns a list of transfer transactions on a specified token.*/
    pub fn get_token_transfers(
        &self,
        contract_address: &str,
        token_id: &str,
    ) -> request::GetTokenTransfersRequest {
        request::GetTokenTransfersRequest {
            client: &self,
            contract_address: contract_address.to_owned(),
            token_id: token_id.to_owned(),
            chain_id: None,
            limit: None,
        }
    }
    /**Get wallet

Get a wallet by a wallet address.*/
    pub fn get_wallet(&self, wallet_address: &str) -> request::GetWalletRequest {
        request::GetWalletRequest {
            client: &self,
            wallet_address: wallet_address.to_owned(),
            chain_id: None,
        }
    }
    ///Returns the latest approval events for all contracts the wallet has given spending authority to.
    pub fn get_wallet_contract_approvals(
        &self,
        wallet_address: &str,
    ) -> request::GetWalletContractApprovalsRequest {
        request::GetWalletContractApprovalsRequest {
            client: &self,
            wallet_address: wallet_address.to_owned(),
        }
    }
    /**Get balances

Returns a list of balances for tokens this wallet currently owns, sorted by contract.*/
    pub fn get_wallet_balances(
        &self,
        wallet_address: &str,
    ) -> request::GetWalletBalancesRequest {
        request::GetWalletBalancesRequest {
            client: &self,
            wallet_address: wallet_address.to_owned(),
            limit: None,
            chain_id: None,
        }
    }
    /**Has tokens

Determine if a wallet has any token from a contract.*/
    pub fn get_contract_gate(
        &self,
        contract_address: &str,
        wallet_address: &str,
    ) -> request::GetContractGateRequest {
        request::GetContractGateRequest {
            client: &self,
            contract_address: contract_address.to_owned(),
            wallet_address: wallet_address.to_owned(),
            chain_id: None,
        }
    }
    /**Has token

Determine if a wallet has a given token from a contract.*/
    pub fn get_token_gate(
        &self,
        token_id: &str,
        contract_address: &str,
        wallet_address: &str,
    ) -> request::GetTokenGateRequest {
        request::GetTokenGateRequest {
            client: &self,
            token_id: token_id.to_owned(),
            contract_address: contract_address.to_owned(),
            wallet_address: wallet_address.to_owned(),
            chain_id: None,
        }
    }
    /**Get minted tokens

Returns a list of tokens minted by a wallet.*/
    pub fn get_wallet_mints(
        &self,
        wallet_address: &str,
    ) -> request::GetWalletMintsRequest {
        request::GetWalletMintsRequest {
            client: &self,
            wallet_address: wallet_address.to_owned(),
            chain_id: None,
            cursor: None,
            limit: None,
        }
    }
    /**Get sold tokens

Returns a list of tokens sold by a wallet.*/
    pub fn get_sold_tokens(
        &self,
        wallet_address: &str,
    ) -> request::GetSoldTokensRequest {
        request::GetSoldTokensRequest {
            client: &self,
            wallet_address: wallet_address.to_owned(),
            chain_id: None,
            cursor: None,
            limit: None,
        }
    }
    /**Get owned tokens

Returns a list of tokens owned by a wallet.*/
    pub fn get_wallet_tokens(
        &self,
        wallet_address: &str,
    ) -> request::GetWalletTokensRequest {
        request::GetWalletTokensRequest {
            client: &self,
            filter_airdrops: None,
            wallet_address: wallet_address.to_owned(),
            cursor: None,
            chain_id: None,
            limit: None,
        }
    }
    /**Get wallet transactions

Returns transactions related to a wallet.*/
    pub fn get_wallet_transactions(
        &self,
        wallet_address: &str,
    ) -> request::GetWalletTransactionsRequest {
        request::GetWalletTransactionsRequest {
            client: &self,
            wallet_address: wallet_address.to_owned(),
            cursor: None,
            limit: None,
            chain_id: None,
            token_type: None,
        }
    }
}
pub enum NxyzAuthenticator {
    Apikey { apikey: String },
}
impl NxyzAuthenticator {
    pub fn from_env() -> Self {
        Self::Apikey {
            apikey: std::env::var("NXYZ_APIKEY")
                .expect("Environment variable NXYZ_APIKEY is not set."),
        }
    }
}
