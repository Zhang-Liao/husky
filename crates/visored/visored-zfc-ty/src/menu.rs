use crate::{
    term::literal::{VdZfcLiteral, VdZfcLiteralData},
    ty::{VdZfcType, VdZfcTypeData},
};
use smallvec::{smallvec, SmallVec};
use visored_item_path::path::VdItemPath;

#[derive(Debug, PartialEq, Eq)]
pub struct VdZfcTypeMenu {
    zero_literal: VdZfcLiteral,
    one_literal: VdZfcLiteral,
    two_literal: VdZfcLiteral,
    /// natural numbers as a type
    natural_number_ty: VdZfcType,
    /// the category of sets as a type
    set_category_ty: VdZfcType,
    /// the category of propositions as a type
    proposition_ty: VdZfcType,
}

impl VdZfcTypeMenu {
    fn new(db: &::salsa::Db) -> Self {
        Self {
            zero_literal: VdZfcLiteral::new(VdZfcLiteralData::NaturalNumber("0".to_string()), db),
            one_literal: VdZfcLiteral::new(VdZfcLiteralData::NaturalNumber("1".to_string()), db),
            two_literal: VdZfcLiteral::new(VdZfcLiteralData::NaturalNumber("2".to_string()), db),
            natural_number_ty: VdZfcType::new(
                db,
                VdZfcTypeData::ItemPath(VdItemPath::NATURAL_NUMBER),
                smallvec![],
            ),
            set_category_ty: VdZfcType::new(
                db,
                VdZfcTypeData::ItemPath(VdItemPath::SET),
                smallvec![],
            ),
            proposition_ty: VdZfcType::new(
                db,
                VdZfcTypeData::ItemPath(VdItemPath::PROPOSITION),
                smallvec![],
            ),
        }
    }
}

impl VdZfcTypeMenu {
    pub fn zero_literal(&self) -> VdZfcLiteral {
        self.zero_literal
    }

    pub fn one_literal(&self) -> VdZfcLiteral {
        self.one_literal
    }

    pub fn two_literal(&self) -> VdZfcLiteral {
        self.two_literal
    }

    pub fn natural_number_ty(&self) -> VdZfcType {
        self.natural_number_ty
    }

    pub fn proposition_ty(&self) -> VdZfcType {
        self.proposition_ty
    }

    pub fn set_category_ty(&self) -> VdZfcType {
        self.set_category_ty
    }
}

#[salsa::tracked(return_ref)]
pub fn vd_zfc_ty_menu(db: &::salsa::Db) -> VdZfcTypeMenu {
    VdZfcTypeMenu::new(db)
}
