use std::error::Error;

use crate::{
    helpers::extract_packages_from_dependency_group,
    schemas::{DependencyGroup, DependencyGroups, Package, Root},
};

pub struct NugetDependencyFetcher {
    client: reqwest::Client,
}

impl NugetDependencyFetcher {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            client: reqwest::Client::builder()
                .user_agent("nuget-scanner")
                .build()?,
        })
    }
    pub async fn fetch_dependencies(
        &self,
        package_id: &str,
        version: &str,
        target_framework: &str,
    ) -> Result<Vec<Package>, Box<dyn Error>> {
        let catalog_url = self.fetch_catalog_entry(package_id, version).await?;
        let deps_group =
            self.fetch_catalog_dependency_group(&catalog_url, target_framework).await?;

        match deps_group {
            Some(g) => Ok(extract_packages_from_dependency_group(g)),
            None => Ok(Vec::new()),
        }
    }

    pub async fn fetch_catalog_entry(
        &self,
        package_id: &str,
        version: &str,
    ) -> Result<String, Box<dyn Error>> {
        let url = format!(
            "https://api.nuget.org/v3/registration5-gz-semver2/{package_id}/{version}.json"
        );
        let response = self.client.get(url).send().await?;
        let json: Root = response.json().await?;

        Ok(json.catalog_entry)
    }

    pub async fn fetch_catalog_dependency_group(
        &self,
        url: &str,
        target_framework: &str,
    ) -> Result<Option<DependencyGroup>, Box<dyn Error>> {
        let response = self.client.get(url).send().await?;
        let groups: DependencyGroups = response.json().await?;

        let dependency_group = groups.dependency_groups.into_iter().find(|dg| {
            dg.target_framework.to_lowercase() == target_framework.to_lowercase()
                && dg.dependencies.is_some()
        });

        Ok(dependency_group)
    }
}

#[tokio::test]
async fn test_fetch_catalog_entry() {
    let client = NugetDependencyFetcher::new().unwrap();
    let package_id = "npgsql";
    let version = "10.0.0";

    let string = client.fetch_catalog_entry(package_id, version)
        .await
        .unwrap();

    println!("{}", string)
}
