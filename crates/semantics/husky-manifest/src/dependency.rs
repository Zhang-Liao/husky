use crate::*;
use husky_manifest_ast::{HasPackageManifestAstSheet, ManifestDependencyAst};

#[derive(Debug, PartialEq, Eq)]
pub struct PackageDependency {
    package_path: PackagePath,
}

impl PackageDependency {
    pub fn package_path(&self) -> PackagePath {
        self.package_path
    }

    pub(crate) fn from_ast(
        db: &dyn ManifestDb,
        toolchain: Toolchain,
        registry_path: RegistryPath,
        ast: &ManifestDependencyAst,
    ) -> Self {
        // ad hoc
        // todo: check source
        PackageDependency {
            package_path: PackagePath::new_registry_package(
                db,
                toolchain,
                ast.name(),
                registry_path,
                semver::Version::new(0, 1, 0),
            ),
        }
    }
}

// #[derive(Debug, PartialEq, Eq)]
// pub struct PackageDependency {
//     name: Word,
//     source: PackageDependencySource,
//     options: PackageDependencyOptions,
// }

// #[derive(Debug, PartialEq, Eq)]
// pub enum PackageDependencySource {
//     Git,
//     Local,
//     Registry,
// }

// #[derive(Debug, PartialEq, Eq)]
// pub struct PackageDependencyOptions {
//     optional: bool,
// }