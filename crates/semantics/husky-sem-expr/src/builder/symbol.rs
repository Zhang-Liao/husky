use super::*;
use husky_eth_term::term::svar::EthSymbolicVariable;

impl<'a> SemaExprBuilder<'a> {
    pub(super) fn infer_current_parameter_symbols(&mut self) {
        for (current_syn_symbol_idx, current_syn_symbol_entry) in self
            .syn_expr_region_data
            .symbol_region()
            .indexed_current_syn_symbols()
        {
            let Some(signature) = self
                .dec_term_region
                .dec_symbol_region()
                .current_parameter_variable_signature(current_syn_symbol_idx)
            else {
                return;
            };
            if let Some(symbol) = signature.term_symbol() {
                if let Ok(symbol) = EthSymbolicVariable::from_dec(self.db, symbol) {
                    self.symbol_terms
                        .insert_new(current_syn_symbol_idx, symbol.into())
                }
            }
            if let Ok(symbol_ty) =
                SymbolType::new_parameter_ty_from_signature(self, current_syn_symbol_idx, signature)
            {
                self.symbol_tys
                    .insert_new(current_syn_symbol_idx, symbol_ty)
            }
        }
    }

    // fn parameter_pattern_ty(&self, pattern_expr_idx: SynPatternExprIdx) -> EthTerm {
    //     match self
    //         .expr_region_data
    //         .symbol_region()
    //         .regular_parameter_pattern_ty_constraint(pattern_expr_idx)
    //     {
    //         Some(_) => todo!(),
    //         None => todo!(),
    //     }
    //     todo!()
    // }
}
