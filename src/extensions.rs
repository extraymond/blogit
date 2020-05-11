mod image;
mod markdown;
use afterglow::prelude::*;

/// Provide render function inplace.
pub trait ContentExtension {
    fn render<'a>(&self, ctx: &mut RenderContext<'a>) -> Node<'a>;
}

/// Allow all container that contains the BlogContent trait to be rendered as ContentExtension
impl<T> ContentExtension for Container<T>
where
    T: BlogContent + LifeCycle,
{
    fn render<'a>(&self, ctx: &mut RenderContext<'a>) -> Node<'a> {
        self.render(ctx)
    }
}

/// Content filter.
pub trait BlogContent {}

impl BlogContent for markdown::Model {}
impl BlogContent for image::Model {}

/// Allow inserting lists of allowed extensions.
pub struct ExtensionContainer {
    blocks: Vec<Box<dyn ContentExtension>>,
}

impl LifeCycle for ExtensionContainer {
    fn new(render_tx: Sender<((), oneshot::Sender<()>)>) -> Self {
        let article = Container::default::<markdown::View>(render_tx.clone());
        let pic = Container::default::<image::View>(render_tx);
        // let md = markdown::Model::new(render_tx.clone());
        // let cont = Container::new(md, Box::new(markdown::View), render_tx);
        ExtensionContainer {
            blocks: vec![Box::new(article), Box::new(pic)],
        }
    }
}

#[derive(Default)]
pub struct View;

impl Renderer for View {
    type Target = ExtensionContainer;
    type Data = ExtensionContainer;

    fn view<'a>(
        &self,
        target: &Self::Target,
        ctx: &mut RenderContext<'a>,
        sender: &MessageSender<Self::Data>,
    ) -> Node<'a> {
        let bump = ctx.bump;

        let block_views = target.blocks.iter().map(|blk| blk.render(ctx));
        dodrio!(bump, <div>{ block_views }</div>)
    }
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);
    use super::*;

    #[wasm_bindgen_test]
    fn test_extensions() {
        Entry::init_app::<ExtensionContainer, View>(None);
    }
}
