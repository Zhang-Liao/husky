use husky_hir_ty::ritchie::HirEagerContract;
use husky_place::place::EthPlace;
use husky_sema_place_contract::site::SemaPlaceContractSite;
use husky_term_prelude::Contract;
use vec_like::SmallVecPairMap;

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct HirEagerPlaceContractSite {
    place_contracts: SmallVecPairMap<EthPlace, HirEagerContract, 2>,
}

impl HirEagerPlaceContractSite {
    pub(crate) fn from_sema(sema_site: &SemaPlaceContractSite) -> Self {
        HirEagerPlaceContractSite {
            place_contracts: SmallVecPairMap::from_iter(
                sema_site
                    .place_contracts()
                    .iter()
                    .copied()
                    .filter_map(|(place, contract)| {
                        (contract != Contract::At)
                            .then_some((place, HirEagerContract::from_contract(contract)))
                    }),
            ),
        }
    }
}

impl HirEagerPlaceContractSite {
    pub fn place_contracts(&self) -> &[(EthPlace, HirEagerContract)] {
        &self.place_contracts
    }
}

impl std::ops::Index<EthPlace> for HirEagerPlaceContractSite {
    type Output = HirEagerContract;

    fn index(&self, place: EthPlace) -> &Self::Output {
        &self.place_contracts[place].1
    }
}