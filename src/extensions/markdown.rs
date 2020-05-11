use afterglow::prelude::*;
use comrak::{markdown_to_html, ComrakOptions};
use uuid::Uuid;

pub struct Model {
    content: Option<String>,
    id: Uuid,
}

#[cfg(feature = "hljs")]
#[wasm_bindgen]
extern "C" {

    /// Highlight a given block, a function exposed from hljs.
    #[wasm_bindgen(js_namespace=hljs)]
    fn highlightBlock(block: &web_sys::Node);
}

impl LifeCycle for Model {
    fn new(render_tx: Sender<((), oneshot::Sender<()>)>) -> Self {
        let doc = web_sys::window().unwrap().document().unwrap();
        let id: Uuid = loop {
            let candidate: Uuid = Uuid::new_v4();
            if doc.get_element_by_id(&candidate.to_string()).is_none() {
                break candidate;
            }
        };

        let content = include_str!("../../assets/test.md");

        Model {
            content: Some(content.to_string()),
            id,
        }
    }

    fn rendererd(
        &self,
        sender: MessageSender<Self>,
        render_tx: &Sender<((), oneshot::Sender<()>)>,
    ) {
        if let Some(content) = self.content.as_ref() {
            spawn_local(Event::NewContent(content.clone()).dispatch(&sender));
        }
    }
}

pub enum Event {
    NewContent(String),
}

impl Messenger for Event {
    type Target = Model;

    fn update(
        &self,
        target: &mut Self::Target,
        sender: &MessageSender<Self::Target>,
        render_tx: &Sender<((), oneshot::Sender<()>)>,
    ) -> bool {
        match self {
            Event::NewContent(content) => {
                let md = markdown_to_html(
                    content,
                    &ComrakOptions {
                        ext_footnotes: true,
                        ..ComrakOptions::default()
                    },
                );
                let doc = web_sys::window().unwrap().document().unwrap();
                let node = doc.get_element_by_id(&target.id.to_string()).unwrap();
                node.set_inner_html(&md);

                if let Ok(code_blocks) = node.query_selector_all("pre code") {
                    for idx in 0..code_blocks.length() {
                        let block = code_blocks.get(idx).unwrap();

                        #[cfg(feature = "hljs")]
                        highlightBlock(&block);
                    }
                }
            }
        }

        false
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
        let id = bf!(in bump, "{}", target.id.to_string()).into_bump_str();
        dodrio!(bump, <div data-id={ id }></div>)
    }
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);
    use super::*;

    #[wasm_bindgen_test]
    fn test_model() {
        Entry::init_app::<Model, View>(None);
    }
}
