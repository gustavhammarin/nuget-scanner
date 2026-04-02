use std::{collections::HashSet, error::Error};

use async_recursion::async_recursion;
use clap::{Parser};
use comfy_table::Table;

use crate::{fetchers::NugetDependencyFetcher, osv::VulnFetcher, schemas::Package};
mod schemas;
mod helpers;
mod fetchers;
mod osv;

#[derive(Parser)]
#[command(name = "nuget-scanner")]
#[command(about = "Scanning Nuget-packages for vulnerabilities")]
struct Cli {
    package_id: String,
    version: String,
    target_framework: String
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let cli = Cli::parse();

    let mut seen: HashSet<String> = HashSet::new();
    let mut all_deps: HashSet<Package> = HashSet::new();

    let client = NugetDependencyFetcher::new()?;

    println!("Gathering data...");

    resolve(&client, &cli.package_id.to_lowercase(), &cli.version, &cli.target_framework, &mut seen, &mut all_deps).await?;

    println!("Checking for vulnerabilites...");
    let vuln_fetcher = VulnFetcher::new();
    let vulns = vuln_fetcher.fetch_vulnerabilities(all_deps.clone()).await?;

    let mut table = Table::new();
    table.set_header(vec!["ID", "SUMMARY"]);

    for vuln in vulns{
        let summary = vuln.summary.unwrap_or_default();
        let id = vuln.id;

        table.add_row(vec![
            &id,
            &summary
        ]);
    }

    println!("{table}");
    Ok(())
}

#[async_recursion]
async fn resolve(fetcher: &NugetDependencyFetcher, package_id: &str, version: &str, target_framework: &str, seen: &mut HashSet<String>, all_deps: &mut HashSet<Package>) -> Result<(), Box<dyn Error>>{
    let key = format!("{}@{}", package_id, version);
    if seen.contains(&key) {
        return Ok(());
    }

    seen.insert(key);

    let deps = fetcher.fetch_dependencies(package_id, version, target_framework).await?;

    for dep in deps {
        all_deps.insert(dep.clone());
        resolve(fetcher, &dep.package_id.to_lowercase(), &dep.version, target_framework, seen, all_deps).await?;
    }

    Ok(())
}



