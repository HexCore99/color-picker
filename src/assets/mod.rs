use std::borrow::Cow;

use gpui::{AssetSource, Result, SharedString};
pub struct Assets;
impl AssetSource for Assets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        match path {
            "icons/pipette.svg" => Ok(Some(Cow::Borrowed(include_bytes!("icons/pipette.svg")))),
            "icons/pinn.svg" => Ok(Some(Cow::Borrowed(include_bytes!("icons/pinn.svg")))),
            "icons/unpin.svg" => Ok(Some(Cow::Borrowed(include_bytes!("icons/unpin.svg")))),
            _ => Ok(None),
        }
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        if path.is_empty() || path == "icons" {
            Ok(vec![
                "icons/pipette.svg".into(),
                "icons/pinn.svg".into(),
                "icons/unpin.svg".into(),
            ])
        } else {
            Ok(vec![])
        }
    }
}
