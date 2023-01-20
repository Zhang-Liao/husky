mod db;
mod error;
#[cfg(test)]
mod tests;

pub use db::*;
pub use error::*;

use husky_signature::*;
use husky_term::*;

#[salsa::jar(db=TypeDb)]
pub struct TypeJar();