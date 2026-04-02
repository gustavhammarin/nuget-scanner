use std::{collections::HashSet, error::Error};

use futures::{StreamExt, stream};

use crate::schemas::{OsvPackage, OsvQuery, OsvResponse, OsvVuln, Package};

pub struct VulnFetcher {
    client: reqwest::Client,
}

impl VulnFetcher {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    pub async fn fetch_vulnerabilities(&self, deps: HashSet<Package>) -> Result<Vec<OsvVuln>, Box<dyn Error>> {
        
        let results = stream::iter(deps)
            .map(|p| self.fetch_vulns_for_package(p.package_id.clone(), p.version.clone()))
            .buffer_unordered(10)
            .filter_map(|r| async {r.ok()})
            .collect::<Vec<_>>()
            .await
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();


        Ok(results)
    }

    async fn fetch_vulns_for_package(&self, name: String, version: String) -> Result<Vec<OsvVuln>, Box<dyn Error>> {
        let query = OsvQuery {
            package: OsvPackage{
                name: name.to_string(),
                ecosystem: "NuGet".to_string(), 
            },
            version: version.to_string(),
        };

        let response: OsvResponse = self.client
            .post("https://api.osv.dev/v1/query")
            .json(&query)
            .send()
            .await?
            .json()
            .await?;

        Ok(response.vulns.unwrap_or_default())
    }
}
