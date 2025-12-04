use anyhow::Result;
use crate::context::Ecosystem;
use chrono::{NaiveDate, DateTime, Utc};
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
struct NpmPackage {
    time: HashMap<String, String>,
}

#[derive(Deserialize)]
struct CratesIoPackage {
    versions: Vec<CratesIoVersion>,
}

#[derive(Deserialize)]
struct CratesIoVersion {
    num: String,
    created_at: String,
}

#[derive(Deserialize)]
struct PypiPackage {
    releases: HashMap<String, Vec<PypiRelease>>,
}

#[derive(Deserialize)]
struct PypiRelease {
    upload_time: String,
}

pub async fn get_release_date(client: &Client, ecosystem: &Ecosystem, package: &str, version: &str) -> Result<Option<NaiveDate>> {
    match ecosystem {
        Ecosystem::Node => get_npm_date(client, package, version).await,
        Ecosystem::Rust => get_crates_io_date(client, package, version).await,
        Ecosystem::Python => get_pypi_date(client, package, version).await,
    }
}

async fn get_npm_date(client: &Client, package: &str, version: &str) -> Result<Option<NaiveDate>> {
    let url = format!("https://registry.npmjs.org/{}", package);
    let resp = client.get(&url)
        .header("User-Agent", "Context-Research-Agent")
        .send()
        .await?;

    if !resp.status().is_success() {
        return Ok(None);
    }

    let data: NpmPackage = resp.json().await?;

    if let Some(time_str) = data.time.get(version) {
        // NPM time format: "2023-11-29T12:00:00.000Z"
        if let Ok(dt) = DateTime::parse_from_rfc3339(time_str) {
            return Ok(Some(dt.with_timezone(&Utc).date_naive()));
        }
    }

    Ok(None)
}

async fn get_crates_io_date(client: &Client, package: &str, version: &str) -> Result<Option<NaiveDate>> {
    let url = format!("https://crates.io/api/v1/crates/{}", package);
    let resp = client.get(&url)
        .header("User-Agent", "Context-Research-Agent (github.com/iberi22/Git-Core-Protocol)")
        .send()
        .await?;

    if !resp.status().is_success() {
        return Ok(None);
    }

    let data: CratesIoPackage = resp.json().await?;

    for v in data.versions {
        if v.num == version {
            // Crates.io format: "2023-11-29T12:00:00.000000+00:00"
            if let Ok(dt) = DateTime::parse_from_rfc3339(&v.created_at) {
                return Ok(Some(dt.with_timezone(&Utc).date_naive()));
            }
        }
    }

    Ok(None)
}

async fn get_pypi_date(client: &Client, package: &str, version: &str) -> Result<Option<NaiveDate>> {
    let url = format!("https://pypi.org/pypi/{}/json", package);
    let resp = client.get(&url)
        .header("User-Agent", "Context-Research-Agent")
        .send()
        .await?;

    if !resp.status().is_success() {
        return Ok(None);
    }

    let data: PypiPackage = resp.json().await?;

    if let Some(releases) = data.releases.get(version) {
        if let Some(first_release) = releases.first() {
            // PyPI format: "2023-11-29T12:00:00" (sometimes missing Z, usually ISO 8601)
            // We'll try standard parsing
            let time_str = format!("{}Z", first_release.upload_time); // Append Z to assume UTC if missing
            if let Ok(dt) = DateTime::parse_from_rfc3339(&time_str).or_else(|_| DateTime::parse_from_rfc3339(&first_release.upload_time)) {
                return Ok(Some(dt.with_timezone(&Utc).date_naive()));
            }
            // Fallback for simple date
            if let Ok(d) = NaiveDate::parse_from_str(&first_release.upload_time, "%Y-%m-%dT%H:%M:%S") {
                return Ok(Some(d));
            }
        }
    }

    Ok(None)
}
