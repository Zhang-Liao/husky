use file::FilePtr;
use instruction_gen::new_func_instruction_sheet;
use semantics_entity::DefinitionRepr;
use vm::{AnyValueDyn, EvalRef, EvalResult};

use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FeatureRepr {
    Value {
        value: EvalRef<'static>,
        ty: EntityRoutePtr,
        file: FilePtr,
        range: TextRange,
        feature: FeaturePtr,
    },
    Expr(Arc<FeatureLazyExpr>),
    LazyBlock(Arc<FeatureLazyBlock>),
    FuncBlock(Arc<FeatureFuncBlock>),
    ProcBlock(Arc<FeatureProcBlock>),
}

impl FeatureRepr {
    pub fn ty(&self) -> EntityRoutePtr {
        match self {
            FeatureRepr::Value { ty, .. } => *ty,
            FeatureRepr::Expr(expr) => expr.expr.ty(),
            FeatureRepr::LazyBlock(block) => todo!(),
            FeatureRepr::FuncBlock(block) => todo!(),
            FeatureRepr::ProcBlock(block) => todo!(),
        }
    }
    pub fn feature(&self) -> FeaturePtr {
        match self {
            FeatureRepr::Value { feature, .. } => *feature,
            FeatureRepr::Expr(expr) => expr.feature,
            FeatureRepr::LazyBlock(block) => block.feature,
            FeatureRepr::FuncBlock(block) => block.feature,
            FeatureRepr::ProcBlock(block) => block.feature,
        }
    }

    pub fn file(&self) -> FilePtr {
        match self {
            FeatureRepr::Value { .. } => todo!(),
            FeatureRepr::Expr(expr) => expr.expr.file,
            FeatureRepr::LazyBlock(block) => block.file,
            FeatureRepr::FuncBlock(block) => block.file,
            FeatureRepr::ProcBlock(block) => block.file,
        }
    }

    pub fn text_range(&self) -> TextRange {
        match self {
            FeatureRepr::Value { .. } => todo!(),
            FeatureRepr::Expr(expr) => expr.expr.range,
            FeatureRepr::LazyBlock(block) => block.range,
            FeatureRepr::FuncBlock(block) => block.range,
            FeatureRepr::ProcBlock(block) => block.range,
        }
    }

    pub fn from_defn(
        db: &dyn FeatureGenQueryGroup,
        opt_this: Option<FeatureRepr>,
        defn_repr: &DefinitionRepr,
        features: &FeatureInterner,
    ) -> Self {
        match defn_repr {
            DefinitionRepr::LazyExpr { expr } => FeatureRepr::Expr(FeatureLazyExpr::new(
                db,
                opt_this,
                expr.clone(),
                &[],
                features,
            )),
            DefinitionRepr::LazyBlock { stmts } => {
                FeatureRepr::LazyBlock(FeatureLazyBlock::new(db, opt_this, stmts, &[], features))
            }
            DefinitionRepr::FuncBlock {
                stmts,
                file,
                range,
                route,
            } => FeatureRepr::FuncBlock(Arc::new(FeatureFuncBlock {
                file: *file,
                range: *range,
                eval_id: Default::default(),
                stmts: stmts.clone(),
                instruction_sheet: new_func_instruction_sheet(
                    db.upcast(),
                    [].into_iter(),
                    stmts,
                    opt_this.is_some(),
                ),
                feature: features.intern(match opt_this {
                    Some(ref this) => Feature::FieldAccess {
                        this: this.feature(),
                        field_ident: route.ident().custom(),
                    },
                    None => Feature::EntityFeature {
                        route: *route,
                        uid: db.compile_time().entity_uid(*route),
                    },
                }),
                opt_this,
            })),
            DefinitionRepr::ProcBlock { stmts, file, range } => todo!(),
        }
    }
}

impl<'eval> From<Arc<FeatureLazyExpr>> for FeatureRepr {
    fn from(expr: Arc<FeatureLazyExpr>) -> Self {
        Self::Expr(expr)
    }
}

impl<'eval> From<Arc<FeatureLazyBlock>> for FeatureRepr {
    fn from(block: Arc<FeatureLazyBlock>) -> Self {
        Self::LazyBlock(block)
    }
}
