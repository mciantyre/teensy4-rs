//! Simple support for reading and modifying Cargo.toml files.
//!
//! Used throughout the teensy4-rs developer tooling.

pub use krate::Krate;
pub use workspace::Workspace;

/// Support for Cargo.toml workspaces
mod workspace {
    use std::path::Path;

    #[derive(serde::Deserialize, serde::Serialize)]
    struct Table {
        members: Vec<String>,
        exclude: Vec<String>,
    }

    /// A `serde` serializable and deserializable definition of a Cargo workspace
    #[derive(serde::Deserialize, serde::Serialize)]
    pub struct Workspace {
        workspace: Table,
    }

    impl Workspace {
        /// Add a new member to the workspace, then sort the collection of
        /// members.
        pub fn add_member<P: AsRef<Path>>(&mut self, member: P) {
            let member = member.as_ref().display().to_string();
            if self.workspace.members.contains(&member) {
                return;
            }
            self.workspace.members.push(member);
            self.workspace.members.sort_unstable();
        }
    }
}

/// Support for Cargo.toml crate specs
mod krate {
    use std::collections::BTreeMap;
    use std::path::Path;

    #[derive(serde::Deserialize, serde::Serialize)]
    struct Complex {
        path: String,
    }

    #[derive(serde::Deserialize, serde::Serialize)]
    #[serde(untagged)]
    enum Dependency {
        JustVersion(String),
        Complex(Complex),
    }

    #[derive(serde::Deserialize, serde::Serialize)]
    pub struct Krate {
        /// Catch-all for the rest of the file
        #[serde(flatten)]
        _rest: toml::value::Value,
        // https://github.com/alexcrichton/toml-rs/issues/142#issuecomment-279009115
        #[serde(serialize_with = "toml::ser::tables_last")]
        dependencies: BTreeMap<String, Dependency>,
    }

    impl Krate {
        pub fn add_dependency<P: AsRef<Path>>(&mut self, name: &str, path: P) {
            self.dependencies.insert(
                name.to_string(),
                Dependency::Complex(Complex {
                    path: path.as_ref().display().to_string(),
                }),
            );
        }
    }
}

//
// TODO these tests are brittle, relying on the
// implementation details of serde & toml. We should
// fix this to ensure that there are matching substrings...
//

#[cfg(test)]
mod tests {

    use super::Krate;
    use super::Workspace;

    #[test]
    fn add_workspace_member() {
        let ws_str = r#"[workspace]
members = ["foo", "bar", "baz"]
exclude = ["ignore"]
"#;
        let mut ws: Workspace = toml::de::from_str(ws_str).unwrap();
        ws.add_member("abc");
        let updated_ws = toml::ser::to_string(&ws).unwrap();
        assert_eq!(
            updated_ws,
            r#"[workspace]
members = ["abc", "bar", "baz", "foo"]
exclude = ["ignore"]
"#
        )
    }

    #[test]
    fn add_crate_dependency() {
        let crate_str = r#"[package]
name = "crate_name"
version = "1.2.3"

[dependencies]
foo = "1.23"
bar = { path = "path/to/bar" }
"#;
        let mut krate: Krate = toml::de::from_str(crate_str).unwrap();
        krate.add_dependency("a_new_dep", "path/to/new/dep");
        let updated_crate = toml::ser::to_string(&krate).unwrap();
        assert_eq!(
            updated_crate,
            r#"[package]
name = "crate_name"
version = "1.2.3"

[dependencies]
foo = "1.23"

[dependencies.a_new_dep]
path = "path/to/new/dep"

[dependencies.bar]
path = "path/to/bar"
"#
        );
    }
}
