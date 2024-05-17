pub mod account_login_view;
pub mod account_management;
pub mod anim;
pub mod global_popup;
pub mod mention;
pub mod note;
pub mod persist_state;
pub mod preview;
pub mod profile;
pub mod relay;
pub mod side_panel;
pub mod username;

pub use account_management::{AccountManagementView, AccountSelectionWidget};
pub use global_popup::DesktopGlobalPopup;
pub use mention::Mention;
pub use note::Note;
pub use preview::{Preview, PreviewApp};
pub use profile::{ProfilePic, ProfilePreview};
pub use relay::RelayView;
pub use side_panel::DesktopSidePanel;
pub use username::Username;

use egui::Margin;

/// This is kind of like the Widget trait but is meant for larger top-level
/// views that are typically stateful. The Widget trait forces us to add mutable
/// implementations at the type level, which screws us when generating Previews
/// for a Widget. I would have just Widget instead of making this Trait otherwise.
///
/// There is some precendent for this, it looks like there's a similar trait
/// in the egui demo library.
pub trait View {
    fn ui(&mut self, ui: &mut egui::Ui);
}

pub fn padding<R>(
    amount: impl Into<Margin>,
    ui: &mut egui::Ui,
    add_contents: impl FnOnce(&mut egui::Ui) -> R,
) -> egui::InnerResponse<R> {
    egui::Frame::none()
        .inner_margin(amount)
        .show(ui, add_contents)
}

pub fn hline(ui: &egui::Ui) {
    // pixel perfect horizontal line
    let rect = ui.available_rect_before_wrap();
    let resize_y = ui.painter().round_to_pixel(rect.top()) - 0.5;
    let stroke = ui.style().visuals.widgets.noninteractive.bg_stroke;
    ui.painter().hline(rect.x_range(), resize_y, stroke);
}

#[inline]
#[allow(unreachable_code)]
pub fn is_mobile() -> bool {
    #[cfg(feature = "emulate_mobile")]
    {
        return true;
    }

    #[cfg(any(target_os = "android", target_os = "ios"))]
    {
        true
    }
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    {
        false
    }
}
