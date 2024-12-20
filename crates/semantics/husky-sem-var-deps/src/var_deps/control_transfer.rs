use super::*;

/// None indicates no control transfer
#[salsa::derive_debug_with_db]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct SemControlTransferVarDeps(Option<OrderedSmallVecSet<SemVarDep, 4>>);

#[test]
fn sem_control_transfer_static_var_deps_default_works() {
    assert_eq!(
        SemControlTransferVarDeps::default(),
        SemControlTransferVarDeps(None)
    );
}

impl SemControlTransferVarDeps {
    /// a deps that represents a control transfer that happens without any dependencies
    pub(crate) fn new_empty() -> Self {
        Self(Some(Default::default()))
    }
}

impl std::ops::Deref for SemControlTransferVarDeps {
    type Target = [SemVarDep];

    fn deref(&self) -> &Self::Target {
        match self.0 {
            Some(ref deps) => deps,
            None => &[],
        }
    }
}

impl SemControlTransferVarDeps {
    pub(crate) fn merge(&mut self, other: &Self) {
        match self.0 {
            Some(ref mut slf) => slf.extend(other),
            None => match other.0 {
                Some(ref other) => self.0 = Some(other.clone()),
                None => (),
            },
        }
    }

    /// use this when some control transfer is caused by the value
    ///
    /// this will result in .0 always being Some
    pub(crate) fn merge_with_value(&mut self, other: &value::SemValueVarDeps) {
        match self.0 {
            Some(ref mut slf) => slf.extend(other),
            None => self.0 = Some((**other).clone()),
        }
    }

    pub(crate) fn compose_with_value(&mut self, other: &value::SemValueVarDeps) {
        match self.0 {
            Some(ref mut slf) => slf.extend(other),
            None => (), // doing nothing because if the thing doesn't have a control transfer, so does its conditional version
        }
    }

    /// this is used when caching, to see whether there is any effective change
    pub(crate) fn merge_counted(&mut self, other: &Self, counter: &mut EffectiveMergeCounter) {
        match self.0 {
            Some(ref mut slf) => {
                let old_len = slf.len();
                slf.extend(other);
                if old_len != slf.len() {
                    counter.count += 1;
                }
            }
            None => match other.0 {
                Some(ref _other) => {
                    unreachable!("for the circumstance this function is going to be invoked, this case will not happen")
                    // self.0 = Some(other.clone());
                    // counter.count += 1;
                }
                None => (),
            },
        }
    }
}
