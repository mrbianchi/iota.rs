use failure::Error;
use reqwest::header::{ContentType, Headers};

pub fn get_transactions_to_approve(
    uri: &str,
    depth: usize,
    reference: &Option<String>,
) -> Result<GetTransactionsToApprove, Error> {
    let client = reqwest::Client::new();
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    headers.set_raw("X-IOTA-API-Version", "1");

    let mut body = json!({
        "command": "getTransactionsToApprove",
        "depth": depth,
    });

    if let Some(reference) = reference {
        body["reference"] = json!(reference);
    }

    let to_approve: GetTransactionsToApprove = client
        .post(uri)
        .headers(headers)
        .body(body.to_string())
        .send()?
        .json()?;

    if let Some(error) = to_approve.error() {
        return Err(format_err!("{}", error));
    }

    Ok(to_approve)
}

#[derive(Deserialize, Debug)]
pub struct GetTransactionsToApprove {
    duration: i64,
    error: Option<String>,
    #[serde(rename = "trunkTransaction")]
    trunk_transaction: Option<String>,
    #[serde(rename = "branchTransaction")]
    branch_transaction: Option<String>,
}

impl GetTransactionsToApprove {
    pub fn duration(&self) -> i64 {
        self.duration
    }
    pub fn error(&self) -> Option<String> {
        self.error.clone()
    }
    pub fn trunk_transaction(&self) -> Option<String> {
        self.trunk_transaction.clone()
    }
    pub fn branch_transaction(&self) -> Option<String> {
        self.branch_transaction.clone()
    }
}
