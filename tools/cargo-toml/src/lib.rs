//! Simple support for reading and modifying Cargo.toml files.
//!
//! Used throughout the teensy4-rs developer tooling.

pub use krate::Krate;
pub use workspace::Workspace;

type CatchAll = toml::value::Value;

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
    use std::error::Error;
    use std::fs;
    use std::path::{Path, PathBuf};

    #[derive(serde::Deserialize, serde::Serialize)]
    struct Complex {
        path: String,
        version: Option<String>,
    }

    #[derive(serde::Deserialize, serde::Serialize)]
    #[serde(untagged)]
    enum Dependency {
        JustVersion(String),
        Complex(Complex),
    }

    #[derive(serde::Deserialize, serde::Serialize)]
    struct Package {
        name: String,
        version: String,
        categories: Option<Vec<String>>,
        keywords: Option<Vec<String>>,
        description: Option<String>,
        license: Option<String>,
        repository: Option<String>,
        #[serde(flatten)]
        _catch_all: super::CatchAll,
    }

    #[derive(serde::Deserialize, serde::Serialize)]
    pub struct Krate {
        package: Package,
        // https://github.com/alexcrichton/toml-rs/issues/142#issuecomment-279009115
        #[serde(serialize_with = "toml::ser::tables_last")]
        dependencies: BTreeMap<String, Dependency>,
        #[serde(flatten)]
        _catch_all: super::CatchAll,
    }

    impl Krate {
        pub fn add_dependency<P: AsRef<Path>>(&mut self, name: &str, path: P) {
            self.dependencies.insert(
                name.to_string(),
                Dependency::Complex(Complex {
                    path: path.as_ref().display().to_string(),
                    version: None,
                }),
            );
        }

        pub fn add_versioned_dependency<P: AsRef<Path>, V: ToString>(
            &mut self,
            name: &str,
            path: P,
            version: V,
        ) {
            self.dependencies.insert(
                name.to_string(),
                Dependency::Complex(Complex {
                    path: path.as_ref().display().to_string(),
                    version: Some(version.to_string()),
                }),
            );
        }

        /// Sets the version for this crate to the supplied version, `ver`
        pub fn set_version<V: ToString>(&mut self, ver: V) {
            self.package.version = ver.to_string();
        }

        pub fn set_categories(&mut self, categories: &[&str]) {
            self.package.categories =
                Some(categories.into_iter().map(|s| String::from(*s)).collect());
        }

        pub fn set_keywords(&mut self, keywords: &[&str]) {
            self.package.keywords = Some(keywords.into_iter().map(|s| String::from(*s)).collect());
        }

        pub fn set_license(&mut self, license: &str) {
            self.package.license = Some(license.to_string());
        }

        pub fn set_repository(&mut self, repo: &str) {
            self.package.repository = Some(repo.to_string());
        }

        pub fn set_description(&mut self, desc: &str) {
            self.package.description = Some(desc.to_string());
        }

        /// Returns an iterator that describes the dependencies of the crate
        pub fn dependencies(&self) -> impl Iterator<Item = &String> {
            self.dependencies.keys()
        }

        pub fn update_dependency<P, F>(
            &mut self,
            dep: &str,
            relative_to: P,
            updater: F,
        ) -> Result<(), Box<dyn Error>>
        where
            P: AsRef<Path>,
            F: FnOnce(&mut Krate),
        {
            let path: PathBuf = match self.dependencies.get(dep) {
                None => {
                    return Err(Box::from(format!(
                        "Dependency '{}' is not a valid dependency!",
                        dep
                    )))
                }
                Some(Dependency::JustVersion(_)) => {
                    return Err(Box::from("Cannot find dependency on the local filesystem!"))
                }
                Some(Dependency::Complex(Complex { path, .. })) => PathBuf::from(path),
            };
            let cargo_toml_path = PathBuf::from(relative_to.as_ref())
                .join(&path)
                .join("Cargo.toml");
            let source = fs::read_to_string(&cargo_toml_path)?;
            let mut krate = ::toml::from_str(&source)?;
            updater(&mut krate);
            self.add_versioned_dependency(&krate.package.name, &path, &krate.package.version);
            let source = ::toml::to_string_pretty(&krate)?;
            fs::write(&cargo_toml_path, &source)?;
            Ok(())
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
        assert!(updated_crate.contains("[package]\nname = \"crate_name\"\nversion = \"1.2.3\""));
        assert!(updated_crate.contains("[dependencies]\nfoo = \"1.23\""));
        assert!(updated_crate.contains("[dependencies.a_new_dep]\npath = \"path/to/new/dep\""));
        assert!(updated_crate.contains("[dependencies.bar]\npath = \"path/to/bar\""));
    }
}
