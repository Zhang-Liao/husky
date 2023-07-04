use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub enum SelfParameterDeclPattern {
    Pure {
        self_value_token: SelfValueToken,
    },
    Owned {
        owned_token: OwnedToken,
        self_value_token: SelfValueToken,
    },
    Mut {
        mut_token: MutToken,
        self_value_token: SelfValueToken,
    },
    MutOwned {
        mut_token: MutToken,
        owned_token: OwnedToken,
        self_value_token: SelfValueToken,
    },
}

impl<'a, 'b> TryParseOptionalFromStream<ExprParseContext<'a, 'b>> for SelfParameterDeclPattern {
    type Error = ExprError;

    // needs more testing
    fn try_parse_stream_optional_from_without_guaranteed_rollback(
        ctx: &mut ExprParseContext<'a, 'b>,
    ) -> Result<Option<Self>, Self::Error> {
        if let Some(mut_token) = ctx.try_parse_optional::<MutToken>()? {
            if let Some(owned_token) = ctx.try_parse_optional::<OwnedToken>()? {
                if let Some(self_value_token) = ctx.try_parse_optional::<SelfValueToken>()? {
                    Ok(Some(Self::MutOwned {
                        mut_token,
                        owned_token,
                        self_value_token,
                    }))
                } else {
                    Ok(None)
                }
            } else if let Some(self_value_token) = ctx.try_parse_optional::<SelfValueToken>()? {
                Ok(Some(Self::Mut {
                    mut_token,
                    self_value_token,
                }))
            } else {
                Ok(None)
            }
        } else if let Some(owned_token) = ctx.try_parse_optional::<OwnedToken>()? {
            if let Some(self_value_token) = ctx.try_parse_optional::<SelfValueToken>()? {
                Ok(Some(Self::Owned {
                    owned_token,
                    self_value_token,
                }))
            } else {
                Ok(None)
            }
        } else if let Some(self_value_token) = ctx.try_parse_optional::<SelfValueToken>()? {
            Ok(Some(Self::Pure { self_value_token }))
        } else {
            Ok(None)
        }
    }
}