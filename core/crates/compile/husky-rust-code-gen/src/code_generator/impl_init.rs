mod impl_ty_linkage_entries;

use husky_entity_route::entity_route_menu;
use husky_entity_route::{make_subroute, make_type_as_trait_member_route};
use husky_entity_semantics::{DefinitionRepr, FieldDefnVariant, MethodDefnKind};
use husky_word::RootIdentifier;
use infer_decl::{CallFormDecl, OutputDecl, ParameterDecl, TyDecl, VariadicTemplate};

use super::*;

impl<'a> RustCodeGenerator<'a> {
    pub(crate) fn gen_init_content(&mut self) {
        self.write(
            r#"use crate::*;
use __husky::init::*;

#[no_mangle]
pub extern "C" fn get_linkages() -> &'static [(__StaticLinkageKey, __Linkage)] {
    LINKAGES
}

pub static LINKAGES: &[(__StaticLinkageKey, __Linkage)] = &["#,
        );
        let main_module = self.db.module(self.package_main).unwrap();
        let entity_link_dependees = self.db.entity_link_dependees(main_module);
        for entity_route in entity_link_dependees.iter() {
            if !entity_route.is_generic() {
                let entity_defn = self.db.entity_defn(*entity_route).unwrap();
                self.gen_linkage_entry(*entity_route, &entity_defn);
            }
        }
        self.write(
            r#"
];
"#,
        );
    }

    fn gen_linkage_entry(&mut self, entity_route: EntityRoutePtr, entity_defn: &EntityDefn) {
        if self.db.is_defn_static(entity_route)
            && !self.db.contains_spatial_parameters(entity_route)
        {
            return;
        }
        match entity_defn.variant {
            EntityDefnVariant::Module { .. } => (),
            EntityDefnVariant::Feature { ref defn_repr } => match **defn_repr {
                DefinitionRepr::LazyExpr { ref expr } => (),
                DefinitionRepr::LazyBlock { ref stmts, ty } => (),
                DefinitionRepr::FuncBlock { route, ty, .. } => {
                    self.gen_eager_feature_linkage_entry(route, ty.route)
                }
                DefinitionRepr::ProcBlock {
                    file,
                    range,
                    ref stmts,
                    ty,
                } => todo!(),
            },
            EntityDefnVariant::Function {
                ref spatial_parameters,
                ref parameters,
                output,
                ref source,
            } => todo!(),
            EntityDefnVariant::Method {
                method_defn_kind, ..
            } => {
                self.gen_method_linkage_entry(method_defn_kind, entity_route);
            }
            EntityDefnVariant::Func {
                ref spatial_parameters,
                ref parameters,
                output,
                ref stmts,
            } => {
                self.write(&format!(
                    r#"
    (
        __StaticLinkageKey::Routine {{
            route: "{entity_route}",
        }},"#,
                ));
                let call_form_decl = self.db.entity_call_form_decl(entity_route).unwrap();
                msg_once!("keyword_parameters");
                self.gen_specific_routine_linkage(
                    None,
                    |this| this.gen_entity_route(entity_route, EntityRouteRole::Caller),
                    |this| this.gen_entity_route(entity_route, EntityRouteRole::Caller),
                    &call_form_decl,
                );
                self.write(
                    r#"
    ),"#,
                );
            }
            EntityDefnVariant::Proc { .. } => {
                self.write(&format!(
                    r#"
    (
        __StaticLinkageKey::Routine {{
            route: "{entity_route}",
        }},"#,
                ));
                let call_form_decl = self.db.entity_call_form_decl(entity_route).unwrap();
                msg_once!("keyword_parameters");
                self.gen_specific_routine_linkage(
                    None,
                    |this| this.gen_entity_route(entity_route, EntityRouteRole::Caller),
                    |this| this.gen_entity_route(entity_route, EntityRouteRole::Caller),
                    &call_form_decl,
                );
                self.write(
                    r#"
    ),"#,
                );
            }
            EntityDefnVariant::Ty {
                spatial_parameters: ref generic_parameters,
                ref ty_members,
                ref variants,
                ty_kind,
                ref trait_impls,
                ref members,
                ref opt_type_call,
                ref opt_visual_stmts,
                ..
            } => match ty_kind {
                TyKind::Record => (),
                _ => self.gen_ty_linkages(ty_kind, opt_type_call, entity_route, members),
            },
            EntityDefnVariant::Trait {
                spatial_parameters: ref generic_parameters,
                ref members,
            } => todo!(),
            EntityDefnVariant::EnumVariant { ident, ref variant } => todo!(),
            EntityDefnVariant::Builtin => todo!(),
            EntityDefnVariant::TyField {
                ty,
                ref field_variant,
                liason,
                opt_linkage,
            } => panic!(),
            EntityDefnVariant::TraitAssociatedTypeImpl { trai, ty } => {}
            EntityDefnVariant::TraitAssociatedConstSizeImpl { value } => todo!(),
            EntityDefnVariant::Input { .. } => todo!(),
        }
    }

    fn gen_method_linkage_entry(
        &mut self,
        method_defn_kind: MethodDefnKind,
        entity_route: EntityRoutePtr,
    ) {
        self.write("\n    (\n");
        match method_defn_kind {
            MethodDefnKind::TypeMethod { .. } => {
                self.write(&format!(
                    r#"
        __StaticLinkageKey::Routine {{ route: "{entity_route}" }},"#,
                ));
                let call_form_decl = self.db.entity_call_form_decl(entity_route).unwrap();
                let this_liason = call_form_decl.this_liason();
                match this_liason {
                    ParameterLiason::MemberAccess => {
                        self.write(&format!(
                            r#"
        method_elem_linkage!("#
                        ));
                        self.gen_entity_route(entity_route.parent(), EntityRouteRole::Decl);
                        let method_name = entity_route.ident().as_str();
                        let mangled_output_ty_vtable =
                            self.db.mangled_ty_vtable(call_form_decl.output.ty);
                        self.write(&format!(
                            ", __registration__::{mangled_output_ty_vtable}, {method_name})"
                        ))
                    }
                    _ => self.gen_specific_routine_linkage(
                        Some((this_liason, entity_route.parent())),
                        |this| {
                            this.write(&format!("__this.{}", entity_route.ident().as_str()));
                        },
                        |this| this.gen_entity_route(entity_route, EntityRouteRole::Caller),
                        &call_form_decl,
                    ),
                }
            }
            MethodDefnKind::TraitMethod { trai } => todo!(),
            MethodDefnKind::TraitMethodImpl { trai } => {
                if trai.kind == entity_route_menu().std_ops_index_trai.kind {
                    match entity_route.kind {
                        EntityRouteKind::Root { ident } => todo!(),
                        EntityRouteKind::Package { main, ident } => todo!(),
                        EntityRouteKind::Child { parent, ident } => todo!(),
                        EntityRouteKind::TypeAsTraitMember { ty, trai, ident } => {
                            self.gen_index_linkage(ty)
                        }
                        EntityRouteKind::Input { main } => todo!(),
                        EntityRouteKind::Generic { ident, entity_kind } => todo!(),
                        EntityRouteKind::ThisType => todo!(),
                    }
                } else {
                    todo!()
                }
            }
        }
        self.write("\n    ),");
    }

    fn gen_specific_routine_linkage(
        &mut self,
        opt_this: Option<(ParameterLiason, EntityRoutePtr)>,
        gen_caller: impl FnOnce(&mut Self),
        gen_call_route: impl FnOnce(&mut Self),
        decl: &CallFormDecl,
    ) {
        let argidx_base = opt_this.map(|_| 1).unwrap_or(0);
        self.write(&format!(
            r#"
        __Linkage::Transfer(__LinkageFp {{
            dev_src: static_dev_src!(),
            wrapper: {{
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {{"#
        ));
        if let Some((this_liason, this_ty)) = opt_this {
            match this_liason {
                ParameterLiason::Pure => {
                    self.write(&format!(
                        r#"
                    let __this: "#
                    ));
                    if self.db.is_copyable(this_ty).unwrap() {
                        todo!()
                    } else {
                        self.write("&");
                        self.gen_entity_route(this_ty, EntityRouteRole::Decl);
                        self.write(&format!(" = __arguments[0].downcast_temp_ref();"))
                    }
                }
                ParameterLiason::Move => todo!(),
                ParameterLiason::MoveMut => todo!(),
                ParameterLiason::MemberAccess => panic!(),
                ParameterLiason::EvalRef => {
                    self.write(&format!(
                        r#"
                    let __this: "#
                    ));
                    if self.db.is_copyable(this_ty).unwrap() {
                        todo!()
                    } else {
                        self.write("&'eval ");
                        self.gen_entity_route(this_ty.deref_route(), EntityRouteRole::Decl);
                        self.write(&format!(" = __arguments[0].downcast_eval_ref();"))
                    }
                }
                ParameterLiason::TempRef => todo!(),
                ParameterLiason::TempRefMut => {
                    self.write(&format!(
                        r#"
                    let __this: "#
                    ));
                    self.write("&mut ");
                    self.gen_entity_route(this_ty, EntityRouteRole::Decl);
                    self.write(&format!(
                        " = unsafe {{ __arb_ref(&__arguments[0]) }}.downcast_temp_mut();"
                    ))
                }
            }
        }
        for (i, parameter) in decl.primary_parameters.iter().enumerate() {
            self.gen_parameter_downcast(i + argidx_base, parameter)
        }
        msg_once!("keyword parameter overrides");
        for (i, parameter) in decl.keyword_parameters.iter().enumerate() {
            let parameter_name = parameter.ident;
            let parameter_ty = parameter.ty;
            self.write(&format!(
                r#"
                    let {parameter_name}: "#
            ));
            self.gen_entity_route(parameter_ty, EntityRouteRole::Decl);
            self.write(&format!(" = todo!();"))
        }
        match decl.variadic_template {
            VariadicTemplate::None => (),
            VariadicTemplate::SingleTyped { ty } => {
                let variadic_start = decl.variadic_start();
                let move_or_copy = match self.db.is_copyable(ty).unwrap() {
                    true => "copy",
                    false => "move",
                };
                self.write(&format!(
                    r#"
                    let __variadics = 
                        __arguments[{variadic_start}..]
                            .iter_mut()
                            .map(|v|v.downcast_{move_or_copy}())
                            .collect();"#,
                ));
            }
        }
        self.write(&format!(
            r#"
                    "#
        ));
        let is_output_ty_primitive = decl.output.ty.is_primitive();
        if !is_output_ty_primitive {
            self.write("__Register::new_box(");
        }
        gen_caller(self);
        self.write("(");
        for (i, parameter) in decl.primary_parameters.iter().enumerate() {
            if i > 0 {
                self.write(", ")
            }
            self.write(&parameter.ident)
        }
        for (i, parameter) in decl.keyword_parameters.iter().enumerate() {
            if i + decl.primary_parameters.len() > 0 {
                self.write(", ");
                if i == 0 {
                    self.write("/* keyword arguments */ ");
                }
            }
            self.write(&parameter.ident)
        }
        match decl.variadic_template {
            VariadicTemplate::None => (),
            VariadicTemplate::SingleTyped { .. } => {
                if decl.primary_parameters.len() > 0 || decl.keyword_parameters.len() > 0 {
                    self.write(", ")
                }
                self.write("__variadics")
            }
        }
        let mangled_output_ty_vtable = self.db.mangled_ty_vtable(decl.output.ty);
        if is_output_ty_primitive {
            self.write(&format!(
                r#").to_register()
                }}
                __wrapper
            }},
            opt_fp: Some("#
            ));
        } else {
            self.write(&format!(
                r#"), &__registration__::{mangled_output_ty_vtable})
                }}
                __wrapper
            }},
            opt_fp: Some("#
            ));
        }
        gen_call_route(self);
        self.write(
            r#" as *const ()),
        }),"#,
        );
    }

    fn gen_eager_feature_linkage_entry(
        &mut self,
        route: EntityRoutePtr,
        output_ty: EntityRoutePtr,
    ) {
        self.write(&format!(
            r#"
    (
        __StaticLinkageKey::FeatureEagerBlock {{
            route: "{route}"
        }},
        feature_linkage!("#
        ));
        self.gen_entity_route(route, EntityRouteRole::Caller);
        self.write(", __registration__::");
        let entity_defn = self.db.entity_defn(route).unwrap();
        self.write(&self.db.mangled_ty_vtable(output_ty));
        self.write(
            r#"),
    ),"#,
        );
    }

    fn gen_index_linkage(&mut self, ty: EntityRoutePtr) {
        msg_once!("todo: generic indexing");
        self.write(&format!(
            r#"
        __StaticLinkageKey::Index {{
            opd_tys: &["{ty:?}", "i32"],
        }},"#,
        ));
        self.write(&format!(
            r#"
        index_linkage!("#
        ));
        self.gen_entity_route(ty, EntityRouteRole::Decl);
        self.write(")")
    }

    fn gen_parameter_downcast(&mut self, i: usize, parameter: &ParameterDecl) {
        let parameter_name = parameter.ident;
        let parameter_ty = parameter.ty;
        match parameter.liason {
            ParameterLiason::Pure => {
                if parameter_ty.is_ref() {
                    todo!()
                } else {
                    if self.db.is_copyable(parameter_ty).unwrap() {
                        self.gen_parameter_downcast_copy(i, parameter)
                    } else {
                        self.gen_parameter_downcast_temp_ref(i, parameter)
                    }
                }
            }
            ParameterLiason::Move => self.gen_parameter_downcast_move(i, parameter),
            ParameterLiason::MoveMut => todo!(),
            ParameterLiason::MemberAccess => todo!(),
            ParameterLiason::EvalRef => self.gen_parameter_downcast_eval_ref(i, parameter),
            ParameterLiason::TempRef => todo!(),
            ParameterLiason::TempRefMut => todo!(),
        }
    }

    fn gen_parameter_downcast_copy(&mut self, i: usize, parameter: &ParameterDecl) {
        let parameter_name = parameter.ident;
        let parameter_ty = parameter.ty;
        self.write(&format!(
            r#"
                let {parameter_name}: "#
        ));
        self.gen_entity_route(parameter_ty, EntityRouteRole::Decl);
        match parameter_ty {
            EntityRoutePtr::Root(root_identifier) => match root_identifier {
                RootIdentifier::Void
                | RootIdentifier::I32
                | RootIdentifier::I64
                | RootIdentifier::F32
                | RootIdentifier::F64
                | RootIdentifier::B32
                | RootIdentifier::B64 => self.write(&format!(
                    " = __arguments[{i}].downcast_{root_identifier}();"
                )),
                RootIdentifier::Bool => todo!(),
                RootIdentifier::True => todo!(),
                RootIdentifier::False => todo!(),
                RootIdentifier::Vec => todo!(),
                RootIdentifier::Tuple => todo!(),
                RootIdentifier::Debug => todo!(),
                RootIdentifier::Std => todo!(),
                RootIdentifier::Core => todo!(),
                RootIdentifier::Mor => todo!(),
                RootIdentifier::Fp => todo!(),
                RootIdentifier::Fn => todo!(),
                RootIdentifier::FnMut => todo!(),
                RootIdentifier::FnOnce => todo!(),
                RootIdentifier::Array => todo!(),
                RootIdentifier::Domains => todo!(),
                RootIdentifier::DatasetType => todo!(),
                RootIdentifier::VisualType => todo!(),
                RootIdentifier::TypeType => todo!(),
                RootIdentifier::TraitType => todo!(),
                RootIdentifier::ModuleType => todo!(),
                RootIdentifier::CloneTrait => todo!(),
                RootIdentifier::CopyTrait => todo!(),
                RootIdentifier::PartialEqTrait => todo!(),
                RootIdentifier::EqTrait => todo!(),
                RootIdentifier::Ref => todo!(),
                RootIdentifier::Option => todo!(),
            },
            EntityRoutePtr::Custom(_) => {
                let parameter_ty_decl: Arc<TyDecl> = self.db.ty_decl(parameter_ty).unwrap();
                match parameter_ty_decl.kind {
                    TyKind::Enum => self.write(&format!(
                        " = __arguments[{i}].downcast_temp_ref::<__VirtualEnum>().kind_idx.into();"
                    )),
                    TyKind::Record => todo!(),
                    TyKind::Struct => todo!(),
                    TyKind::Primitive => todo!(),
                    TyKind::Vec => todo!(),
                    TyKind::Array => todo!(),
                    TyKind::Other => todo!(),
                }
            }
            EntityRoutePtr::ThisType => todo!(),
        }
    }

    fn gen_parameter_downcast_move(&mut self, i: usize, parameter: &ParameterDecl) {
        let parameter_name = parameter.ident;
        let parameter_ty = parameter.ty;
        self.write(&format!(
            r#"
                let {parameter_name}: "#
        ));
        self.gen_entity_route(parameter_ty, EntityRouteRole::Decl);
        self.write(&format!(
            " = unsafe {{ __arb_ref(&__arguments[{i}]) }}.downcast_move();"
        ))
    }

    fn gen_parameter_downcast_temp_ref(&mut self, i: usize, parameter: &ParameterDecl) {
        let parameter_name = parameter.ident;
        let parameter_ty = parameter.ty;
        self.write(&format!(
            r#"
                let {parameter_name}: &"#
        ));
        self.gen_entity_route(parameter_ty, EntityRouteRole::Decl);
        self.write(&format!(" = __arguments[{i}].downcast_temp_ref();"))
    }

    fn gen_parameter_downcast_eval_ref(&mut self, i: usize, parameter: &ParameterDecl) {
        let parameter_name = parameter.ident;
        let parameter_ty = parameter.ty;
        self.write(&format!(
            r#"
                let {parameter_name}: &'eval "#
        ));
        self.gen_entity_route(parameter_ty.deref_route(), EntityRouteRole::Decl);
        self.write(&format!(" = __arguments[{i}].downcast_eval_ref();"))
    }
}
