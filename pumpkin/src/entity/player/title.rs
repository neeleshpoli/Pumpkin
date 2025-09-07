use pumpkin_protocol::java::client::play::{CActionBar, CSubtitle, CTitleText};
use pumpkin_util::text::TextComponent;

use crate::entity::player::Player;

impl Player {
    pub async fn show_title(&self, text: &TextComponent, mode: &TitleMode) {
        match mode {
            TitleMode::Title => self.client.enqueue_packet(&CTitleText::new(text)).await,
            TitleMode::SubTitle => self.client.enqueue_packet(&CSubtitle::new(text)).await,
            TitleMode::ActionBar => self.client.enqueue_packet(&CActionBar::new(text)).await,
        }
    }
}

#[derive(Debug)]
pub enum TitleMode {
    Title,
    SubTitle,
    ActionBar,
}
