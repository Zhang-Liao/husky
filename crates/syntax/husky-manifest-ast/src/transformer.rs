use husky_word::Word;

use crate::*;

pub(crate) type ManifestAstTransformer<'a, 'b, A: TomlAst> =
    TomlTransformer<'a, 'b, ManifestAstTransformContext, A>;

pub struct ManifestAstTransformContext;

impl TomlDeserializeContext for ManifestAstTransformContext {
    type Db<'a> = dyn ManifestAstDb + 'a;
    type Menu = ManifestAstMenu;
    type Error = ManifestAstError;
}