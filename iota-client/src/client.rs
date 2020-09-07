//! The Client module to connect through IRI with API usages
use crate::error::*;
use crate::builder::ClientBuilder;
use crate::types::*;

use bee_transaction::atomic::{
    Hash, Message, 
};

use reqwest::Url;

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};

// macro_rules! response {
//     ($self:ident, $body:ident) => {
//         $self
//             .client
//             .post($self.get_node()?)
//             .header("Content-Type", "application/json")
//             .header("X-IOTA-API-Version", "1")
//             .body($body.to_string())
//             .send()
//             .await?
//             .json()
//             .await?
//     };
//     ($self:ident, $body:ident, $node:ident) => {
//         $self
//             .client
//             .post($node)
//             .header("Content-Type", "application/json")
//             .header("X-IOTA-API-Version", "1")
//             .body($body.to_string())
//             .send()
//             .await?
//             .json()
//             .await?
//     };
// }

/// An instance of the client using IRI URI
#[derive(Debug, Clone)]
pub struct Client {
    /// Node pool of IOTA nodes
    pub(crate) pool: Arc<RwLock<HashSet<Url>>>,
    pub(crate) sync: Arc<RwLock<Vec<Url>>>,
    /// A reqwest Client to make Requests with
    pub(crate) client: reqwest::Client,
    pub(crate) mwm: u8,
    pub(crate) quorum_size: u8,
    pub(crate) quorum_threshold: u8,
}

impl Client {
    /// Create the builder to instntiate the IOTA Client.
    pub fn new() -> ClientBuilder {
        ClientBuilder::new()
    }

    pub(crate) fn sync(&mut self) {
        let mut sync_list: HashMap<usize, Vec<Url>> = HashMap::new();
        for url in &*self.pool.read().unwrap() {
            if let Ok(milestone) = self.get_info(url.clone()) {
                let set = sync_list
                    .entry(milestone.latest_solid_subtangle_milestone_index)
                    .or_insert(Vec::new());
                set.push(url.clone());
            };
        }

        *self.sync.write().unwrap() = sync_list.into_iter().max_by_key(|(x, _)| *x).unwrap().1;
    }

    /// Add a node to the node pool.
    pub fn add_node(&mut self, uri: &str) -> Result<bool> {
        let url = Url::parse(uri).map_err(|_| Error::UrlError)?;
        Ok(self.pool.write().unwrap().insert(url))
    }

    /// Remove a node from the node pool.
    pub fn remove_node(&mut self, uri: &str) -> Result<bool> {
        let url = Url::parse(uri).map_err(|_| Error::UrlError)?;
        Ok(self.pool.write().unwrap().remove(&url))
    }

    // pub(crate) fn get_node(&self) -> Result<Url> {
    //     // TODO getbalance, isconfirmed and were_addresses_spent_from should do quorum mode
    //     Ok(self
    //         .sync
    //         .read()
    //         .unwrap()
    //         .iter()
    //         .next()
    //         .ok_or(Error::NodePoolEmpty)?
    //         .clone())
    // }

    //////////////////////////////////////////////////////////////////////
    // Chrysalis Node API
    //////////////////////////////////////////////////////////////////////
    
    /// GET /info endpoint
    pub fn get_info(&self, _url: Url) -> Result<GetInfoResponse> {
        Ok(GetInfoResponse {
            name: String::from("Bee"),
            version: String::from("v0.1.0"),
            is_healthy: true,
            latest_solid_subtangle_milestone_index: 0,
        })
    }

    /// GET /tips endpoint
    pub fn get_tips(&self) -> Result<(Hash, Hash)> {
        Ok((Hash([0;32]), Hash([0;32])))
    }

    /// GET /messages/* endpoint
    pub fn get_messages(&self) -> Result<Vec<Message>> {
        Ok(Vec::new())
    }

    /// GET /transactions/* endpoint
    pub fn get_transactions(&self) -> Result<Vec<Message>> {
        Ok(Vec::new())
    }

    /// GET /outputs/* endpoint
    pub fn get_outputs(&self) -> Result<Vec<Output>> {
        Ok(Vec::new())
    }
}
