mod arena;
mod tab;

pub(crate) use self::tab::*;

use self::arena::*;
use super::*;
use egui_dock::DockState;
use husky_trace_view_doc::doc::{MockTraceViewDoc, TraceViewDoc};
use ui::{IsUiComponent, UiComponent};

pub struct Doc {
    title: String,
    component: UiComponent<egui::Ui, NotebookSettings, NotebookActionBuffer>,
}

impl Doc {
    pub fn title(&self) -> &str {
        self.title.as_ref()
    }
}

#[derive(Default)]
pub(crate) struct Docs {
    doc_arena: DocArena,
}

impl Docs {
    pub(crate) fn doc_arena(&self) -> &DocArena {
        &self.doc_arena
    }

    pub(crate) fn component_mut(
        &mut self,
        id: DocId,
    ) -> &mut UiComponent<egui::Ui, NotebookSettings, NotebookActionBuffer> {
        &mut self.doc_arena[id].component
    }
}

impl NotebookApp {
    pub fn add_default_docs(&mut self) {
        self.add_doc(Doc {
            title: "mock trace view doc".to_string(),
            component: UiComponent::new(MockTraceViewDoc::new_mock()),
        });
        self.add_doc(Doc {
            title: "Settings".to_string(),
            component: UiComponent::new(NotebookSettingsView),
        })
    }

    pub(crate) fn add_doc(&mut self, doc: Doc) {
        let id = self.docs.doc_arena.alloc(doc);
        self.dock_state.push_to_focused_leaf(DocTab::new(id))
    }
}