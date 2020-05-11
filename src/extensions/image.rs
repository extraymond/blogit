use afterglow::prelude::*;

pub struct Model {
    src: Option<String>,
}

impl LifeCycle for Model {
    fn new(render_tx: Sender<((), oneshot::Sender<()>)>) -> Self {
        let src = Some(
            "https://interactive-examples.mdn.mozilla.net/media/examples/surfer-240-200.jpg".into(),
        );
        Model { src }
    }
}

pub enum Events {
    NewSrc(String),
}
impl Messenger for Events {
    type Target = Model;

    fn update(
        &self,
        target: &mut Self::Target,
        sender: &MessageSender<Self::Target>,
        render_tx: &Sender<((), oneshot::Sender<()>)>,
    ) -> bool {
        match self {
            Events::NewSrc(src) => {
                target.src = Some(src.to_owned());
                true
            }
        }
    }
}

#[derive(Default)]
pub struct View;

impl Renderer for View {
    type Target = Model;
    type Data = Model;

    fn view<'a>(
        &self,
        target: &Self::Target,
        ctx: &mut RenderContext<'a>,
        sender: &MessageSender<Self::Data>,
    ) -> Node<'a> {
        let bump = ctx.bump;

        let src = bf!(in bump, "{}", target.src.clone().unwrap_or_default()).into_bump_str();
        dodrio!(bump,
            <img src={ src }></img>
        )
    }
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);
    use super::*;

    #[wasm_bindgen_test]
    fn test_image() {
        Entry::init_app::<Model, View>(None);
    }
}
