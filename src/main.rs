use std::{collections::HashSet, error::Error};

use async_recursion::async_recursion;
use clap::{Parser};
use crossterm::event::{self, Event, KeyCode};

use crate::{fetchers::NugetDependencyFetcher, osv::VulnFetcher, schemas::Package, tui::{App, draw}};
mod schemas;
mod helpers;
mod fetchers;
mod osv;
mod tui;

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

    all_deps.insert(Package { package_id: cli.package_id.clone(), version: cli.version.clone() });

    let client = NugetDependencyFetcher::new()?;

    println!("Gathering data...");

    resolve(&client, &cli.package_id.to_lowercase(), &cli.version, &cli.target_framework, &mut seen, &mut all_deps).await?;

    println!("Checking for vulnerabilites...");
    let vuln_fetcher = VulnFetcher::new();
    let vulns = vuln_fetcher.fetch_vulnerabilities(all_deps.clone()).await?;

    let mut terminal = ratatui::init();
    let mut app = App::new(vulns);

    loop {
        terminal.draw(|f| draw(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Down | KeyCode::Char('j') => app.next(),
                KeyCode::Up   | KeyCode::Char('k') => app.prev(),
                KeyCode::Char('q') | KeyCode::Esc  => break,
                _ => {}
            }
        }
    }

    ratatui::restore();
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



