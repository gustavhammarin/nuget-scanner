use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DependencyGroups{
    #[serde(rename = "dependencyGroups")]
    pub dependency_groups: Vec<DependencyGroup>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DependencyGroup{
    pub dependencies: Option<Vec<Dependency>>,
    #[serde(rename = "targetFramework")]
    pub target_framework: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Dependency{
    pub id: String,
    pub range: String
}

#[derive(Serialize, Deserialize, Debug, Clone,Hash, PartialEq, Eq)]
pub struct Package{
    pub package_id: String,
    pub version: String
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Root{
    #[serde(rename = "catalogEntry")]
    pub catalog_entry: String
}

#[derive(Serialize)]
pub struct OsvQuery{
    pub package: OsvPackage,
    pub version: String
}
#[derive(Serialize)]
pub struct OsvPackage{
    pub name: String,
    pub ecosystem: String
}

#[derive(Deserialize, Debug)]
pub struct OsvResponse {
    pub vulns: Option<Vec<OsvVuln>>,
}

#[derive(Deserialize, Debug)]
pub struct OsvVuln {
    pub id: String,
    pub summary: Option<String>,
}



