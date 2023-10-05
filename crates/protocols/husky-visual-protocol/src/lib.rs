#[cfg(feature = "mock")]
pub mod mock;
pub trait IsVisualProtocol {
    type VisualComponent;

    type Visual: IsVisual;
}

pub trait IsVisual {
    type Component;

    fn from_components(components: &[Self::Component]) -> Self;
}

pub type VisualComponent<VisualProtocol> = <VisualProtocol as IsVisualProtocol>::VisualComponent;

pub struct VisualActionBuffer<VisualAction> {
    actions: smallvec::SmallVec<[VisualAction; 2]>,
}

impl<VisualAction> Default for VisualActionBuffer<VisualAction> {
    fn default() -> Self {
        Self {
            actions: Default::default(),
        }
    }
}

impl<Action> VisualActionBuffer<Action> {
    pub fn push(&mut self, action: Action) {
        self.actions.push(action)
    }
}