use futures_util::{Future, FutureExt};
use serde::Deserialize;

use super::super::{Client, HttpError};

#[derive(Clone, Debug, Deserialize)]
pub struct CryptoDepositAddress {
    pub address: String,
    pub currency: String,
}

impl Client {
    pub fn crypto_deposit_address(
        self,
        currency: &str,
    ) -> impl Future<Output = Result<Option<CryptoDepositAddress>, HttpError>> {
        self.get_request(&format!("user/deposit/address/{}", currency))
            .then(|res| async move {
                match res {
                    Ok(f) => Ok(f),
                    Err(HttpError::UnparseableResponse(err, text)) => {
                        // Catch the case where no deposit address has been generated yet
                        if text == "{}" {
                            return Ok(None);
                        }

                        Err(HttpError::UnparseableResponse(err, text))
                    }
                    Err(e) => Err(e),
                }
            })
    }
}
