use crate::menu::{ln_term_menu, LnTermMenu};
use lean_item_path::menu::{ln_item_path_menu, LnItemPathMenu};
use smallvec::*;

use super::LnInstantiation;

#[derive(Debug, PartialEq, Eq)]
pub struct LnInstantiationMenu {
    pub int_pos: LnInstantiation,
    pub rat_pos: LnInstantiation,
    pub real_pos: LnInstantiation,
    pub complex_pos: LnInstantiation,
    pub int_neg: LnInstantiation,
    pub rat_neg: LnInstantiation,
    pub real_neg: LnInstantiation,
    pub complex_neg: LnInstantiation,
    pub int_sub: LnInstantiation,
    pub rat_sub: LnInstantiation,
    pub real_sub: LnInstantiation,
    pub complex_sub: LnInstantiation,
    pub rat_div: LnInstantiation,
    pub real_div: LnInstantiation,
    pub complex_div: LnInstantiation,
    pub nat_add: LnInstantiation,
    pub int_add: LnInstantiation,
    pub rat_add: LnInstantiation,
    pub real_add: LnInstantiation,
    pub complex_add: LnInstantiation,
    pub nat_mul: LnInstantiation,
    pub int_mul: LnInstantiation,
    pub rat_mul: LnInstantiation,
    pub real_mul: LnInstantiation,
    pub complex_mul: LnInstantiation,
    pub nat_to_the_power_of_nat: LnInstantiation,
    pub int_to_the_power_of_nat: LnInstantiation,
    pub rat_to_the_power_of_nat: LnInstantiation,
    pub real_to_the_power_of_nat: LnInstantiation,
    pub complex_to_the_power_of_nat: LnInstantiation,
    pub nat_eq: LnInstantiation,
    pub int_eq: LnInstantiation,
    pub rat_eq: LnInstantiation,
    pub real_eq: LnInstantiation,
    pub complex_eq: LnInstantiation,
    pub nat_ne: LnInstantiation,
    pub int_ne: LnInstantiation,
    pub rat_ne: LnInstantiation,
    pub real_ne: LnInstantiation,
    pub complex_ne: LnInstantiation,
    pub nat_lt: LnInstantiation,
    pub int_lt: LnInstantiation,
    pub rat_lt: LnInstantiation,
    pub real_lt: LnInstantiation,
    pub nat_gt: LnInstantiation,
    pub int_gt: LnInstantiation,
    pub rat_gt: LnInstantiation,
    pub real_gt: LnInstantiation,
    pub nat_le: LnInstantiation,
    pub int_le: LnInstantiation,
    pub rat_le: LnInstantiation,
    pub real_le: LnInstantiation,
    pub nat_ge: LnInstantiation,
    pub int_ge: LnInstantiation,
    pub rat_ge: LnInstantiation,
    pub real_ge: LnInstantiation,
}

impl LnInstantiationMenu {
    pub fn new(db: &::salsa::Db) -> Self {
        let LnItemPathMenu {
            ring_add,
            ring_mul,
            ring_pos,
            ring_neg,
            field_div,
            eq,
            le,
            ge,
            ..
        } = *ln_item_path_menu(db);
        let LnTermMenu {
            nat,
            int,
            rat,
            real,
            complex,
            ..
        } = *ln_term_menu(db);
        let t = |path, arguments| LnInstantiation::new(db, path, arguments);
        Self {
            int_pos: t(ring_pos, smallvec![int]),
            rat_pos: t(ring_pos, smallvec![rat]),
            real_pos: t(ring_pos, smallvec![real]),
            complex_pos: t(ring_pos, smallvec![complex]),
            int_neg: t(ring_neg, smallvec![int]),
            rat_neg: t(ring_neg, smallvec![rat]),
            real_neg: t(ring_neg, smallvec![real]),
            complex_neg: t(ring_neg, smallvec![complex]),
            int_sub: t(ring_add, smallvec![int]),
            rat_sub: t(ring_add, smallvec![rat]),
            real_sub: t(ring_add, smallvec![real]),
            complex_sub: t(ring_add, smallvec![complex]),
            rat_div: t(field_div, smallvec![rat]),
            real_div: t(field_div, smallvec![real]),
            complex_div: t(field_div, smallvec![complex]),
            nat_add: t(ring_add, smallvec![nat]),
            int_add: t(ring_add, smallvec![int]),
            rat_add: t(ring_add, smallvec![rat]),
            real_add: t(ring_add, smallvec![real]),
            complex_add: t(ring_add, smallvec![complex]),
            nat_mul: t(ring_mul, smallvec![nat]),
            int_mul: t(ring_mul, smallvec![int]),
            rat_mul: t(ring_mul, smallvec![rat]),
            real_mul: t(ring_mul, smallvec![real]),
            complex_mul: t(ring_mul, smallvec![complex]),
            nat_to_the_power_of_nat: t(ring_mul, smallvec![nat, nat]),
            int_to_the_power_of_nat: t(ring_mul, smallvec![int, nat]),
            rat_to_the_power_of_nat: t(ring_mul, smallvec![rat, nat]),
            real_to_the_power_of_nat: t(ring_mul, smallvec![real, nat]),
            complex_to_the_power_of_nat: t(ring_mul, smallvec![complex, nat]),
            nat_eq: t(eq, smallvec![nat]),
            int_eq: t(eq, smallvec![int]),
            rat_eq: t(eq, smallvec![rat]),
            real_eq: t(eq, smallvec![real]),
            complex_eq: t(eq, smallvec![complex]),
            nat_ne: t(eq, smallvec![nat]),
            int_ne: t(eq, smallvec![int]),
            rat_ne: t(eq, smallvec![rat]),
            real_ne: t(eq, smallvec![real]),
            complex_ne: t(eq, smallvec![complex]),
            nat_lt: t(le, smallvec![nat]),
            int_lt: t(le, smallvec![int]),
            rat_lt: t(le, smallvec![rat]),
            real_lt: t(le, smallvec![real]),
            nat_gt: t(ge, smallvec![nat]),
            int_gt: t(ge, smallvec![int]),
            rat_gt: t(ge, smallvec![rat]),
            real_gt: t(ge, smallvec![real]),
            nat_le: t(le, smallvec![nat]),
            int_le: t(le, smallvec![int]),
            rat_le: t(le, smallvec![rat]),
            real_le: t(le, smallvec![real]),
            nat_ge: t(ge, smallvec![nat]),
            int_ge: t(ge, smallvec![int]),
            rat_ge: t(ge, smallvec![rat]),
            real_ge: t(ge, smallvec![real]),
        }
    }
}

#[salsa::tracked(return_ref)]
pub fn ln_instantiation_menu(db: &::salsa::Db) -> LnInstantiationMenu {
    LnInstantiationMenu::new(db)
}
