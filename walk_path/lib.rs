use anyhow::Result;

pub type Child<T> = std::rc::Rc<std::cell::RefCell<WalkPath<T>>>;
pub type Children<T> = std::collections::HashMap<std::path::PathBuf, Child<T>>;

/// tree catching markdown tests into a valid rust module structure
#[derive(Debug)]
pub enum WalkPath<T> {
    Root(Children<T>),
    Dir(std::path::PathBuf, Children<T>),
    File(std::path::PathBuf, T),
}

impl<T> Default for WalkPath<T> {
    fn default() -> Self {
        WalkPath::new()
    }
}

impl<T> WalkPath<T> {
    /// create empty tree
    pub fn new() -> Self {
        Self::Root(Children::new())
    }

    /// recursive directory scanner
    /// returns `false` if no code was generated
    pub fn scan(
        &mut self,
        path: &std::path::Path,
        extension: &str,
        f: &dyn Fn(&mut WalkPath<T>, &std::path::Path) -> Result<bool>,
    ) -> Result<bool> {
        // prepare return value
        let mut found = false;
        // read given directory
        for entry in std::fs::read_dir(path)?.flatten() {
            // get file type
            if let Ok(file_type) = entry.file_type() {
                let file_name = entry.file_name().into_string().unwrap();
                // check if directory or Markdown file
                if file_type.is_dir() && ![".", ".."].contains(&file_name.as_str()) {
                    // scan deeper
                    if self.scan(&entry.path(), extension, f)? {
                        // generated code
                        found = true;
                    }
                } else if file_type.is_file()
                    && file_name.ends_with(&format!(".{extension}"))
                    && !f(self, &entry.path())?
                {
                    // tell cargo to watch this file
                    println!("cargo:rerun-if-changed={}", entry.path().display());
                    // generated code
                    found = true;
                }
            }
        }
        Ok(found)
    }
}
