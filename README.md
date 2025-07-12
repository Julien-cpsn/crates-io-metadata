# crates.io metadata

Parse (and scrap) crates.io to retrieve crate informations & metadata

## Scrap (optionnal)

Use feature `scrap` (uses Reqwest)

```rust
use crates_io_metadata::types::CratesIoResponse;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let metadata: CratesIoResponse = scrap("tokio").await?;
    Ok(())
}
```

## Provided types

```rust
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct CratesIoResponse {
    #[serde(rename = "crate")]
    pub crate_info: Crate,
    pub versions: Vec<Version>,
    pub keywords: Vec<Keyword>,
    pub categories: Vec<Category>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Crate {
    pub id: String,
    pub name: String,
    pub updated_at: String,
    pub versions: Vec<u64>,
    pub keywords: Vec<String>,
    pub categories: Vec<String>,
    pub badges: Vec<String>,
    pub created_at: String,
    pub downloads: u64,
    pub recent_downloads: Option<u64>,
    pub default_version: String,
    pub num_versions: u32,
    pub yanked: bool,
    pub max_version: String,
    pub newest_version: String,
    pub max_stable_version: String,
    pub description: Option<String>,
    pub homepage: Option<String>,
    pub documentation: Option<String>,
    pub repository: Option<String>,
    pub links: CrateLinks,
    pub exact_match: bool,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct CrateLinks {
    pub version_downloads: String,
    pub versions: Option<String>,
    pub owners: String,
    pub owner_team: String,
    pub owner_user: String,
    pub reverse_dependencies: String,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Version {
    pub id: u64,
    #[serde(rename = "crate")]
    pub crate_name: String,
    pub num: String,
    pub dl_path: String,
    pub readme_path: String,
    pub updated_at: String,
    pub created_at: String,
    pub downloads: u64,
    pub features: HashMap<String, Vec<String>>,
    pub yanked: bool,
    pub yank_message: Option<String>,
    pub lib_links: Option<String>,
    pub license: Option<String>,
    pub links: VersionLinks,
    pub crate_size: u64,
    pub published_by: Option<User>,
    pub audit_actions: Vec<AuditAction>,
    pub checksum: String,
    pub rust_version: Option<String>,
    pub has_lib: Option<bool>,
    pub bin_names: Option<Vec<String>>,
    pub edition: Option<String>,
    pub description: Option<String>,
    pub homepage: Option<String>,
    pub documentation: Option<String>,
    pub repository: Option<String>,
    pub trustpub_data: Option<String>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct VersionLinks {
    pub dependencies: String,
    pub version_downloads: String,
    pub authors: String,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: u64,
    pub login: String,
    pub name: Option<String>,
    pub avatar: String,
    pub url: String,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct AuditAction {
    pub action: String,
    pub user: User,
    pub time: String,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Keyword {
    pub id: String,
    pub keyword: String,
    pub created_at: String,
    pub crates_cnt: u32,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Category {
    pub id: String,
    pub category: String,
    pub slug: String,
    pub description: String,
    pub created_at: String,
    pub crates_cnt: u32,
}
```
