use std::{error::Error, panic};

use anyhow::Result;
use color_eyre::config::HookBuilder;

use super::tui::Tui;

pub fn install_hooks() -> Result<()> {
    let (panic_hook, eyre_hook) = HookBuilder::default().into_hooks();

    let panic_hook = panic_hook.into_panic_hook();
    panic::set_hook(Box::new(move |panic_info| {
        Tui::restore().unwrap();
        panic_hook(panic_info);
    }));

    let eyre_hook = eyre_hook.into_eyre_hook();
    color_eyre::eyre::set_hook(Box::new(move |error: &(dyn Error + 'static)| {
        Tui::restore().unwrap();
        eyre_hook(error)
    }))?;

    Ok(())
}
