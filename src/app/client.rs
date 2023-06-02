use anyhow::Ok;
use keride::error::Result;

pub struct SignifyClient {

}

impl SignifyClient {
    pub fn new() -> Result<Self> {
        let mut client = Self::default();

        return Ok(client)
    }
}

impl Default for SignifyClient {
    fn default() -> Self {
        SignifyClient {
        }
    }
}
