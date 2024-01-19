pub trait IsUiComponent<Ui, UiComponentSettings, ParentActionBuffer> {
    fn render_dyn(
        &mut self,
        ui: &mut Ui,
        settings: &mut UiComponentSettings,
        action_buffer: &mut ParentActionBuffer,
    );
}

pub struct UiComponent<Ui, UiComponentConfig, ParentActionBuffer>(
    Box<dyn IsUiComponent<Ui, UiComponentConfig, ParentActionBuffer>>,
);

impl<Ui, UiComponentSettings, ParentActionBuffer>
    UiComponent<Ui, UiComponentSettings, ParentActionBuffer>
{
    pub fn ui(
        &mut self,
        ui: &mut Ui,
        settings: &mut UiComponentSettings,
        action_buffer: &mut ParentActionBuffer,
    ) {
        self.0.render_dyn(ui, settings, action_buffer)
    }
}

impl<Ui, UiComponentConfig, ParentActionBuffer>
    UiComponent<Ui, UiComponentConfig, ParentActionBuffer>
{
    pub fn new<UiComponentImpl>(ui_component: UiComponentImpl) -> Self
    where
        UiComponentImpl: IsUiComponent<Ui, UiComponentConfig, ParentActionBuffer> + 'static,
    {
        Self(Box::new(ui_component))
    }
}
