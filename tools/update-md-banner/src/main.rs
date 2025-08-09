// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use regex::Regex;
use scan_dir::ScanDir;
use std::fs;

fn main() -> std::io::Result<()> {
    let search_path = "./doc";
    let extensions = [".md"];

    let re_test = Regex::new(r"\[!\[test\].*\n\n").unwrap();
    let re_code_head = Regex::new(r"```µcad,([\w_.]+)(#(.+))?").unwrap();

    let files = ScanDir::files()
        .walk(search_path, |iter| {
            iter.filter(|(_, name)| extensions.iter().any(|extension| name.ends_with(extension)))
                .map(|(ref entry, _)| entry.path())
                .collect::<Vec<_>>()
        })
        .expect("scan_path failed");

    for path in files {
        let content = fs::read_to_string(&path)?;
        let content = re_test.replace_all(&content, "");

        // Verwenden Sie split_term, um den Text in Segmente zu unterteilen
        let mut result = String::new();
        let mut last_end = 0;

        for mat in re_code_head.find_iter(&content) {
            result.push_str(&content[last_end..mat.start()]);

            let replacement = mat.as_str();
            if let Some(cap) = re_code_head.captures(replacement) {
                result.push_str(&match (cap.get(1), cap.get(3)) {
                    (Some(name), Some(mode)) => format!(
                        "[![test](.test/{name}.svg)](.test/{name}.log)\n\n```µcad,{name}#{mode}",
                        name = name.as_str(),
                        mode = mode.as_str()
                    ),
                    (Some(name), None) => format!(
                        "[![test](.test/{name}.svg)](.test/{name}.log)\n\n```µcad,{name}",
                        name = name.as_str(),
                    ),
                    _ => replacement.to_string(),
                });
            } else {
                result.push_str(replacement);
            }

            last_end = mat.end();
        }

        // Fügen Sie den verbleibenden Text nach dem letzten Match hinzu
        result.push_str(&content[last_end..]);

        fs::write(&path, result)?;
    }

    Ok(())
}
