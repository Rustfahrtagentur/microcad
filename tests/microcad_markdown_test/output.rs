use std::path::PathBuf;

pub struct Output {
    name: String,
    input: PathBuf,
    banner: PathBuf,
    log: PathBuf,
}

impl Output {
    pub fn new(name: String, input: PathBuf, banner: PathBuf, log: PathBuf) -> Self {
        Self {
            name,
            input,
            banner,
            log,
        }
    }
    pub fn has_path(&self, path: &PathBuf) -> bool {
        self.banner == *path || self.log == *path
    }
}

impl std::fmt::Display for Output {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const M: &str = "make test list error";
        writeln!(
            f,
            "| [![test]({banner})]({log}) | [{name}]({path}) |",
            name = self.name,
            banner = self.banner.as_os_str().to_str().expect(M),
            path = self.input.as_os_str().to_str().expect(M),
            log = self.log.as_os_str().to_str().expect(M)
        )
    }
}

impl Eq for Output {}

impl PartialEq for Output {
    fn eq(&self, other: &Self) -> bool {
        self.name.to_lowercase().eq(&other.name.to_lowercase())
    }
}

#[allow(clippy::non_canonical_partial_ord_impl)]
impl PartialOrd for Output {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.name
            .to_lowercase()
            .partial_cmp(&other.name.to_lowercase())
    }
}

impl Ord for Output {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.to_lowercase().cmp(&other.name.to_lowercase())
    }
}
