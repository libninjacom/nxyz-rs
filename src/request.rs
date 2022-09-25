use serde_json::json;
use crate::model::*;
use crate::NxyzClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetCollectionsSuggestionsResultsRequest<'a> {
    pub(crate) client: &'a NxyzClient,
    pub query: Option<String>,
}
impl<'a> GetCollectionsSuggestionsResultsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Vec<AutoSuggestion>> {
        let mut r = self.client.client.get("/api/v1-alpha/collections/suggestions");
        if let Some(ref unwrapped) = self.query {
            r = r.push_query("query", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn query(mut self, query: &str) -> Self {
        self.query = Some(query.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetTopCollectionsRequest<'a> {
    pub(crate) client: &'a NxyzClient,
}
impl<'a> GetTopCollectionsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Vec<Collection>> {
        let mut r = self.client.client.get("/api/v1-alpha/collections/top");
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostRefreshRequest<'a> {
    pub(crate) client: &'a NxyzClient,
    pub contract_address: String,
    pub token_id: String,
    pub chain_id: Option<String>,
    pub media: Option<bool>,
}
impl<'a> PostRefreshRequest<'a> {
    pub async fn send(self) -> anyhow::Result<()> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/api/v1-alpha/refresh/{contract_address}/{token_id}",
                    contract_address = self.contract_address, token_id = self.token_id
                ),
            );
        if let Some(ref unwrapped) = self.chain_id {
            r = r.push_query("chainID", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.media {
            r = r.push_query("media", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => Ok(()),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn chain_id(mut self, chain_id: &str) -> Self {
        self.chain_id = Some(chain_id.to_owned());
        self
    }
    pub fn media(mut self, media: bool) -> Self {
        self.media = Some(media);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetTokensSuggestionsResultsRequest<'a> {
    pub(crate) client: &'a NxyzClient,
    pub query: Option<String>,
}
impl<'a> GetTokensSuggestionsResultsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Vec<AutoSuggestion>> {
        let mut r = self.client.client.get("/api/v1-alpha/tokens/suggestions");
        if let Some(ref unwrapped) = self.query {
            r = r.push_query("query", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn query(mut self, query: &str) -> Self {
        self.query = Some(query.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetTopTokensRequest<'a> {
    pub(crate) client: &'a NxyzClient,
}
impl<'a> GetTopTokensRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Vec<Token>> {
        let mut r = self.client.client.get("/api/v1-alpha/tokens/top");
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetWalletsSuggestionsResultsRequest<'a> {
    pub(crate) client: &'a NxyzClient,
    pub query: Option<String>,
}
impl<'a> GetWalletsSuggestionsResultsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Vec<AutoSuggestion>> {
        let mut r = self.client.client.get("/api/v1-alpha/wallets/suggestions");
        if let Some(ref unwrapped) = self.query {
            r = r.push_query("query", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn query(mut self, query: &str) -> Self {
        self.query = Some(query.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetTopWalletsRequest<'a> {
    pub(crate) client: &'a NxyzClient,
}
impl<'a> GetTopWalletsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Vec<Wallet>> {
        let mut r = self.client.client.get("/api/v1-alpha/wallets/top");
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetBlockchainsRequest<'a> {
    pub(crate) client: &'a NxyzClient,
}
impl<'a> GetBlockchainsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Vec<BlockchainInfo>> {
        let mut r = self.client.client.get("/api/v1/blockchains");
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetCollectionRequest<'a> {
    pub(crate) client: &'a NxyzClient,
    pub contract_address: String,
    pub chain_id: Option<String>,
}
impl<'a> GetCollectionRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Vec<Collection>> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/api/v1/collections/{contract_address}", contract_address = self
                    .contract_address
                ),
            );
        if let Some(ref unwrapped) = self.chain_id {
            r = r.push_query("chainID", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn chain_id(mut self, chain_id: &str) -> Self {
        self.chain_id = Some(chain_id.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetContractTransactionHistoryRequest<'a> {
    pub(crate) client: &'a NxyzClient,
    pub contract_address: String,
    pub cursor: Option<String>,
    pub chain_id: Option<String>,
    pub limit: Option<i64>,
}
impl<'a> GetContractTransactionHistoryRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Vec<Transaction>> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/api/v1/collections/{contract_address}/transactions/history",
                    contract_address = self.contract_address
                ),
            );
        if let Some(ref unwrapped) = self.cursor {
            r = r.push_query("cursor", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.chain_id {
            r = r.push_query("chainID", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.cursor = Some(cursor.to_owned());
        self
    }
    pub fn chain_id(mut self, chain_id: &str) -> Self {
        self.chain_id = Some(chain_id.to_owned());
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetSearchResultsRequest<'a> {
    pub(crate) client: &'a NxyzClient,
    pub query: String,
    pub cursor: Option<String>,
    pub limit: Option<i64>,
}
impl<'a> GetSearchResultsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Vec<SearchDocument>> {
        let mut r = self
            .client
            .client
            .get(&format!("/api/v1/search/{query}", query = self.query));
        if let Some(ref unwrapped) = self.cursor {
            r = r.push_query("cursor", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.cursor = Some(cursor.to_owned());
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetSuggestionsResultsRequest<'a> {
    pub(crate) client: &'a NxyzClient,
    pub query: Option<String>,
}
impl<'a> GetSuggestionsResultsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Vec<AutoSuggestion>> {
        let mut r = self.client.client.get("/api/v1/suggestions");
        if let Some(ref unwrapped) = self.query {
            r = r.push_query("query", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn query(mut self, query: &str) -> Self {
        self.query = Some(query.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetContractTokensByContractAndIdRequest<'a> {
    pub(crate) client: &'a NxyzClient,
    pub contract_addresses: String,
    pub token_identifiers: String,
    pub chain_id: Option<String>,
    pub limit: Option<i64>,
}
impl<'a> GetContractTokensByContractAndIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Vec<Token>> {
        let mut r = self.client.client.get("/api/v1/token-batch");
        r = r.push_query("contractAddresses", &self.contract_addresses.to_string());
        r = r.push_query("tokenIdentifiers", &self.token_identifiers.to_string());
        if let Some(ref unwrapped) = self.chain_id {
            r = r.push_query("chainID", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn chain_id(mut self, chain_id: &str) -> Self {
        self.chain_id = Some(chain_id.to_owned());
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetErc20MetadataRequest<'a> {
    pub(crate) client: &'a NxyzClient,
    pub contract_address: String,
    pub chain_id: Option<String>,
}
impl<'a> GetErc20MetadataRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Vec<Erc20Metadata>> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/api/v1/token/{contract_address}/erc20/metadata", contract_address =
                    self.contract_address
                ),
            );
        if let Some(ref unwrapped) = self.chain_id {
            r = r.push_query("chainID", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn chain_id(mut self, chain_id: &str) -> Self {
        self.chain_id = Some(chain_id.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetTokensBySearchQueryRequest<'a> {
    pub(crate) client: &'a NxyzClient,
    pub query: String,
    pub chain_id: Option<String>,
    pub limit: Option<i64>,
    pub cursor: Option<String>,
}
impl<'a> GetTokensBySearchQueryRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Vec<Token>> {
        let mut r = self
            .client
            .client
            .get(&format!("/api/v1/tokens/search/{query}", query = self.query));
        if let Some(ref unwrapped) = self.chain_id {
            r = r.push_query("chainID", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.cursor {
            r = r.push_query("cursor", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn chain_id(mut self, chain_id: &str) -> Self {
        self.chain_id = Some(chain_id.to_owned());
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.cursor = Some(cursor.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetContractTokensRequest<'a> {
    pub(crate) client: &'a NxyzClient,
    pub contract_address: String,
    pub chain_id: Option<String>,
    pub limit: Option<i64>,
}
impl<'a> GetContractTokensRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Vec<Token>> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/api/v1/tokens/{contract_address}", contract_address = self
                    .contract_address
                ),
            );
        if let Some(ref unwrapped) = self.chain_id {
            r = r.push_query("chainID", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn chain_id(mut self, chain_id: &str) -> Self {
        self.chain_id = Some(chain_id.to_owned());
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetTokenRequest<'a> {
    pub(crate) client: &'a NxyzClient,
    pub contract_address: String,
    pub token_id: String,
    pub chain_id: Option<String>,
}
impl<'a> GetTokenRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Vec<Token>> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/api/v1/tokens/{contract_address}/{token_id}", contract_address =
                    self.contract_address, token_id = self.token_id
                ),
            );
        if let Some(ref unwrapped) = self.chain_id {
            r = r.push_query("chainID", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn chain_id(mut self, chain_id: &str) -> Self {
        self.chain_id = Some(chain_id.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetTokenTransfersRequest<'a> {
    pub(crate) client: &'a NxyzClient,
    pub contract_address: String,
    pub token_id: String,
    pub chain_id: Option<String>,
    pub limit: Option<i64>,
}
impl<'a> GetTokenTransfersRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TokenEvents> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/api/v1/tokens/{contract_address}/{token_id}/transfers",
                    contract_address = self.contract_address, token_id = self.token_id
                ),
            );
        if let Some(ref unwrapped) = self.chain_id {
            r = r.push_query("chainID", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn chain_id(mut self, chain_id: &str) -> Self {
        self.chain_id = Some(chain_id.to_owned());
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetWalletRequest<'a> {
    pub(crate) client: &'a NxyzClient,
    pub wallet_address: String,
    pub chain_id: Option<String>,
}
impl<'a> GetWalletRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Vec<Wallet>> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/api/v1/wallets/{wallet_address}", wallet_address = self
                    .wallet_address
                ),
            );
        if let Some(ref unwrapped) = self.chain_id {
            r = r.push_query("chainID", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn chain_id(mut self, chain_id: &str) -> Self {
        self.chain_id = Some(chain_id.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetWalletContractApprovalsRequest<'a> {
    pub(crate) client: &'a NxyzClient,
    pub wallet_address: String,
}
impl<'a> GetWalletContractApprovalsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Vec<Transaction>> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/api/v1/wallets/{wallet_address}/approvals", wallet_address = self
                    .wallet_address
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetWalletBalancesRequest<'a> {
    pub(crate) client: &'a NxyzClient,
    pub wallet_address: String,
    pub limit: Option<i64>,
    pub chain_id: Option<String>,
}
impl<'a> GetWalletBalancesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Vec<CurrencyInfo>> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/api/v1/wallets/{wallet_address}/balances", wallet_address = self
                    .wallet_address
                ),
            );
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.chain_id {
            r = r.push_query("chainID", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn chain_id(mut self, chain_id: &str) -> Self {
        self.chain_id = Some(chain_id.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetContractGateRequest<'a> {
    pub(crate) client: &'a NxyzClient,
    pub contract_address: String,
    pub wallet_address: String,
    pub chain_id: Option<String>,
}
impl<'a> GetContractGateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<AssetGate> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/api/v1/wallets/{wallet_address}/gate/{contract_address}",
                    contract_address = self.contract_address, wallet_address = self
                    .wallet_address
                ),
            );
        if let Some(ref unwrapped) = self.chain_id {
            r = r.push_query("chainID", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn chain_id(mut self, chain_id: &str) -> Self {
        self.chain_id = Some(chain_id.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetTokenGateRequest<'a> {
    pub(crate) client: &'a NxyzClient,
    pub token_id: String,
    pub contract_address: String,
    pub wallet_address: String,
    pub chain_id: Option<String>,
}
impl<'a> GetTokenGateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<AssetGate> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/api/v1/wallets/{wallet_address}/gate/{contract_address}/{token_id}",
                    token_id = self.token_id, contract_address = self.contract_address,
                    wallet_address = self.wallet_address
                ),
            );
        if let Some(ref unwrapped) = self.chain_id {
            r = r.push_query("chainID", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn chain_id(mut self, chain_id: &str) -> Self {
        self.chain_id = Some(chain_id.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetWalletMintsRequest<'a> {
    pub(crate) client: &'a NxyzClient,
    pub wallet_address: String,
    pub chain_id: Option<String>,
    pub cursor: Option<String>,
    pub limit: Option<i64>,
}
impl<'a> GetWalletMintsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Vec<Token>> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/api/v1/wallets/{wallet_address}/mints", wallet_address = self
                    .wallet_address
                ),
            );
        if let Some(ref unwrapped) = self.chain_id {
            r = r.push_query("chainID", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.cursor {
            r = r.push_query("cursor", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn chain_id(mut self, chain_id: &str) -> Self {
        self.chain_id = Some(chain_id.to_owned());
        self
    }
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.cursor = Some(cursor.to_owned());
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetSoldTokensRequest<'a> {
    pub(crate) client: &'a NxyzClient,
    pub wallet_address: String,
    pub chain_id: Option<String>,
    pub cursor: Option<String>,
    pub limit: Option<i64>,
}
impl<'a> GetSoldTokensRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Vec<Token>> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/api/v1/wallets/{wallet_address}/sold-tokens", wallet_address = self
                    .wallet_address
                ),
            );
        if let Some(ref unwrapped) = self.chain_id {
            r = r.push_query("chainID", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.cursor {
            r = r.push_query("cursor", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn chain_id(mut self, chain_id: &str) -> Self {
        self.chain_id = Some(chain_id.to_owned());
        self
    }
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.cursor = Some(cursor.to_owned());
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetWalletTokensRequest<'a> {
    pub(crate) client: &'a NxyzClient,
    pub filter_airdrops: Option<bool>,
    pub wallet_address: String,
    pub cursor: Option<String>,
    pub chain_id: Option<String>,
    pub limit: Option<i64>,
}
impl<'a> GetWalletTokensRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Vec<Token>> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/api/v1/wallets/{wallet_address}/tokens", wallet_address = self
                    .wallet_address
                ),
            );
        if let Some(ref unwrapped) = self.filter_airdrops {
            r = r.push_query("filterAirdrops", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.cursor {
            r = r.push_query("cursor", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.chain_id {
            r = r.push_query("chainID", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn filter_airdrops(mut self, filter_airdrops: bool) -> Self {
        self.filter_airdrops = Some(filter_airdrops);
        self
    }
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.cursor = Some(cursor.to_owned());
        self
    }
    pub fn chain_id(mut self, chain_id: &str) -> Self {
        self.chain_id = Some(chain_id.to_owned());
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetWalletTransactionsRequest<'a> {
    pub(crate) client: &'a NxyzClient,
    pub wallet_address: String,
    pub cursor: Option<String>,
    pub limit: Option<i64>,
    pub chain_id: Option<String>,
    pub token_type: Option<String>,
}
impl<'a> GetWalletTransactionsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Vec<Transaction>> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/api/v1/wallets/{wallet_address}/transactions/history",
                    wallet_address = self.wallet_address
                ),
            );
        if let Some(ref unwrapped) = self.cursor {
            r = r.push_query("cursor", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.chain_id {
            r = r.push_query("chainID", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.token_type {
            r = r.push_query("tokenType", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.cursor = Some(cursor.to_owned());
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn chain_id(mut self, chain_id: &str) -> Self {
        self.chain_id = Some(chain_id.to_owned());
        self
    }
    pub fn token_type(mut self, token_type: &str) -> Self {
        self.token_type = Some(token_type.to_owned());
        self
    }
}
