mod extensions;
use afterglow::prelude::*;
use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "wee_alloc")] {
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    if cfg!(feature = "console_error_panic_hook") {
        console_error_panic_hook::set_once();
    }

    if cfg!(debug_assertions) {
        femme::start(log::LevelFilter::Debug).expect("unable to start logger");
        log::info!("debug build");
    } else {
        femme::start(log::LevelFilter::Warn).expect("unable to start logger");
    }

    Entry::init_app::<extensions::ExtensionContainer, extensions::View>(None);
}
