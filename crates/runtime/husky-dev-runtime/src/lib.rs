#![feature(try_trait_v2_residual)]
#![feature(try_trait_v2)]
mod config;
mod eval;

pub use self::config::*;

use husky_dev_comptime::{DevComptime, DevComptimeTarget};
use husky_ki::{KiRuntimeConstant, KiRuntimeConstantData};
use husky_ki_repr::repr::KiRepr;
use husky_linkage::linkage::Linkage;
use husky_task::{
    devend::IsDevend,
    helpers::{DevendException, DevendValue},
};
use husky_task::{
    devend::IsRuntimeStorage,
    helpers::{DevendKiControlFlow, DevendValueResult},
};
use husky_task_interface::{
    ki_repr::{KiDomainReprInterface, KiReprInterface, KiRuntimeConstantInterface},
    HuskyIngredientIndex, HuskyJarIndex, IsDevRuntime, IsLinkageImpl, LinkageImplKiControlFlow,
};
use husky_vfs::{error::VfsResult, path::linktime_target_path::LinktimeTargetPath};
use std::{convert::Infallible, path::Path};

/// Dropping libraries or linkage_impls before runtime storage will lead to segmentation fault
///
/// so it's necessary to pub `storage` field before `comptime`
pub struct DevRuntime<Devend: IsDevend> {
    config: DevRuntimeConfig<Devend>,
    storage: Devend::RuntimeStorage,
    comptime: DevComptime<Devend>,
}

impl<Devend: IsDevend> DevRuntime<Devend> {
    pub fn new(
        target_crate: impl AsRef<Path>,
        config: Option<DevRuntimeConfig<Devend>>,
    ) -> VfsResult<Self> {
        Ok(Self {
            config: config.unwrap_or_default(),
            storage: Default::default(),
            comptime: DevComptime::new(target_crate)?,
        })
    }

    pub fn db(&self) -> &::salsa::Db {
        self.comptime.db()
    }

    pub fn comptime_target(&self) -> DevComptimeTarget {
        self.comptime.target()
    }

    pub fn linktime_target_path(&self) -> Option<LinktimeTargetPath> {
        self.comptime.linktime_target_path()
    }
}

impl<Devend: IsDevend> Default for DevRuntime<Devend>
where
    Devend::Linktime: Default,
{
    fn default() -> Self {
        Self {
            comptime: Default::default(),
            storage: Default::default(),
            config: Default::default(),
        }
    }
}

impl<Devend: IsDevend> IsDevRuntime<Devend::LinkageImpl> for DevRuntime<Devend> {
    type StaticSelf = Self;

    unsafe fn cast_to_static_self_static_ref(&self) -> &'static Self::StaticSelf {
        &*(unsafe { self as *const _ })
    }

    fn eval_ingredient_at_pedestal_with(
        &self,
        jar_index: HuskyJarIndex,
        ingredient_index: HuskyIngredientIndex,
        base_point: Devend::Pedestal,
        f: impl FnOnce() -> DevendValueResult<Devend>,
    ) -> DevendKiControlFlow<Devend> {
        self.storage.get_or_try_init_val_value(
            self.comptime.ingredient_val(jar_index, ingredient_index),
            base_point,
            f,
            self.db(),
        )
    }

    fn eval_ingredient_at_pedestal(
        &self,
        jar_index: HuskyJarIndex,
        ingredient_index: HuskyIngredientIndex,
        pedestal: Devend::Pedestal,
    ) -> DevendKiControlFlow<Devend> {
        self.eval_ki_repr_at_pedestal(
            self.comptime
                .ingredient_ki_repr(jar_index, ingredient_index),
            pedestal,
        )
    }

    fn eval_ki_repr_interface_at_pedestal(
        &self,
        ki_repr_interface: KiReprInterface,
        pedestal: Devend::Pedestal,
    ) -> DevendKiControlFlow<Devend> {
        self.eval_ki_repr_at_pedestal(ki_repr_interface.into(), pedestal)
    }

    fn eval_ki_domain_repr_interface_at_pedestal(
        &self,
        ki_domain_repr: KiDomainReprInterface,
        pedestal: Devend::Pedestal,
    ) -> husky_task_interface::ki_control_flow::KiControlFlow<(), Infallible, DevendException<Devend>>
    {
        self.eval_ki_domain_repr_at_pedestal(ki_domain_repr.into(), pedestal)
    }

    fn eval_ki_repr_with(
        &self,
        ki_repr: KiReprInterface,
        pedestal: Devend::Pedestal,
        f: impl FnOnce(KiDomainReprInterface) -> DevendKiControlFlow<Devend>,
    ) -> DevendKiControlFlow<Devend> {
        let db = self.db();
        let ki_repr: KiRepr = unsafe { std::mem::transmute(ki_repr) };
        let ki_domain_repr: KiDomainReprInterface =
            unsafe { std::mem::transmute(ki_repr.ki_domain_repr(db)) };
        self.storage
            .get_or_try_init_val_value(ki_repr.val(db), pedestal, || f(ki_domain_repr), db)
    }

    fn eval_memo_field_with(
        &self,
        jar_index: HuskyJarIndex,
        ingredient_index: HuskyIngredientIndex,
        pedestal: Devend::Pedestal,
        slf: &'static std::ffi::c_void,
        f: fn(&'static std::ffi::c_void) -> DevendKiControlFlow<Devend>,
    ) -> DevendKiControlFlow<Devend> {
        self.storage
            .get_or_try_init_memo_field_value(jar_index, ingredient_index, pedestal, slf, f)
    }

    fn eval_val_runtime_constant(
        &self,
        val_runtime_constant: KiRuntimeConstantInterface,
    ) -> DevendValue<Devend> {
        use husky_value_interface::IsValue;

        let db = self.db();
        let val_runtime_constant: KiRuntimeConstant =
            unsafe { std::mem::transmute(val_runtime_constant) };
        match val_runtime_constant.data(db) {
            KiRuntimeConstantData::TypeVariantPath(path) => {
                let presenter = self
                    .comptime
                    .linkage_impl(Linkage::new_enum_index_presenter(
                        path.parent_ty_path(db),
                        db,
                    ))
                    .enum_index_value_presenter();
                DevendValue::<Devend>::from_enum_index(path.index(db).raw(), presenter)
            }
        }
    }
}
