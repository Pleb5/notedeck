pub mod picture;
pub mod preview;

use egui::{ScrollArea, Widget};
use enostr::Pubkey;
use nostrdb::{Ndb, Transaction};
pub use picture::ProfilePic;
pub use preview::ProfilePreview;

use crate::{
    actionbar::NoteActionResponse,
    imgcache::ImageCache,
    notecache::NoteCache,
    timeline::{TimelineCache, TimelineCacheKey},
};

use super::timeline::{tabs_ui, TimelineTabView};

pub struct ProfileView<'a> {
    pubkey: &'a Pubkey,
    col_id: usize,
    timeline_cache: &'a mut TimelineCache,
    ndb: &'a Ndb,
    note_cache: &'a mut NoteCache,
    img_cache: &'a mut ImageCache,
}

impl<'a> ProfileView<'a> {
    pub fn new(
        pubkey: &'a Pubkey,
        col_id: usize,
        timeline_cache: &'a mut TimelineCache,
        ndb: &'a Ndb,
        note_cache: &'a mut NoteCache,
        img_cache: &'a mut ImageCache,
    ) -> Self {
        ProfileView {
            pubkey,
            col_id,
            timeline_cache,
            ndb,
            note_cache,
            img_cache,
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) -> NoteActionResponse {
        let scroll_id = egui::Id::new(("profile_scroll", self.col_id, self.pubkey));

        ScrollArea::vertical()
            .id_source(scroll_id)
            .show(ui, |ui| {
                let txn = Transaction::new(self.ndb).expect("txn");
                if let Ok(profile) = self.ndb.get_profile_by_pubkey(&txn, self.pubkey.bytes()) {
                    ProfilePreview::new(&profile, self.img_cache).ui(ui);
                }
                let profile = self
                    .timeline_cache
                    .notes(
                        self.ndb,
                        self.note_cache,
                        &txn,
                        &TimelineCacheKey::pubkey(self.pubkey.bytes()),
                    )
                    .get_ptr();

                profile.timeline_mut().selected_view = tabs_ui(ui);

                TimelineTabView::new(
                    profile.timeline().current_view(),
                    false,
                    false,
                    &txn,
                    self.ndb,
                    self.note_cache,
                    self.img_cache,
                )
                .show(ui)
            })
            .inner
    }
}
