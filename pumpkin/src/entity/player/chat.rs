use std::collections::VecDeque;

use pumpkin_protocol::{
    bedrock::server::text::SText,
    java::client::play::{CDisguisedChatMessage, CSystemChatMessage},
};
use pumpkin_util::text::TextComponent;
use uuid::Uuid;

use crate::{
    entity::player::{LastSeen, Player},
    net::ClientPlatform,
};

const MAX_CACHED_SIGNATURES: u8 = 128; // Vanilla: 128
const MAX_PREVIOUS_MESSAGES: u8 = 20; // Vanilla: 20

impl Player {
    pub async fn send_message(
        &self,
        message: &TextComponent,
        chat_type: u8,
        sender_name: &TextComponent,
        target_name: Option<&TextComponent>,
    ) {
        self.client
            .enqueue_packet(&CDisguisedChatMessage::new(
                message,
                (chat_type + 1).into(),
                sender_name,
                target_name,
            ))
            .await;
    }

    pub async fn send_system_message(&self, text: &TextComponent) {
        match &self.client {
            ClientPlatform::Java(client) => {
                client
                    .enqueue_packet(&CSystemChatMessage::new(text, false))
                    .await;
            }
            ClientPlatform::Bedrock(client) => {
                client
                    .send_game_packet(&SText::system_message(text.clone().get_text()))
                    .await;
            }
        }
    }

    pub async fn send_system_message_raw(&self, text: &TextComponent, overlay: bool) {
        match &self.client {
            ClientPlatform::Java(client) => {
                client
                    .enqueue_packet(&CSystemChatMessage::new(text, overlay))
                    .await;
            }
            ClientPlatform::Bedrock(client) => {
                client
                    .send_game_packet(&SText::system_message(text.clone().get_text()))
                    .await;
            }
        }
    }
}

pub struct MessageCache {
    /// max 128 cached message signatures. Most recent FIRST.
    /// Server should (when possible) reference indexes in this (recipient's) cache instead of sending full signatures in last seen.
    /// Must be 1:1 with client's signature cache.
    pub(super) full_cache: VecDeque<Box<[u8]>>,
    /// max 20 last seen messages by the sender. Most Recent LAST
    pub last_seen: LastSeen,
}

impl Default for MessageCache {
    fn default() -> Self {
        Self {
            full_cache: VecDeque::with_capacity(MAX_CACHED_SIGNATURES as usize),
            last_seen: LastSeen::default(),
        }
    }
}

impl MessageCache {
    /// Not used for caching seen messages. Only for non-indexed signatures from senders.
    pub fn cache_signatures(&mut self, signatures: &[Box<[u8]>]) {
        for sig in signatures.iter().rev() {
            if self.full_cache.contains(sig) {
                continue;
            }
            // If the cache is maxed, and someone sends a signature older than the oldest in cache, ignore it
            if self.full_cache.len() < MAX_CACHED_SIGNATURES as usize {
                self.full_cache.push_back(sig.clone()); // Recipient never saw this message so it must be older than the oldest in cache
            }
        }
    }

    /// Adds a seen signature to `last_seen` and `full_cache`.
    pub fn add_seen_signature(&mut self, signature: &[u8]) {
        if self.last_seen.0.len() >= MAX_PREVIOUS_MESSAGES as usize {
            self.last_seen.0.remove(0);
        }
        self.last_seen.0.push(signature.into());
        // This probably doesn't need to be a loop, but better safe than sorry
        while self.full_cache.len() >= MAX_CACHED_SIGNATURES as usize {
            self.full_cache.pop_back();
        }
        self.full_cache.push_front(signature.into()); // Since recipient saw this message it will be most recent in cache
    }
}

/// Represents the player's chat mode settings.
#[derive(Debug, Clone)]
pub enum ChatMode {
    /// Chat is enabled for the player.
    Enabled,
    /// The player should only see chat messages from commands.
    CommandsOnly,
    /// All messages should be hidden.
    Hidden,
}

pub struct InvalidChatMode;

impl TryFrom<i32> for ChatMode {
    type Error = InvalidChatMode;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Enabled),
            1 => Ok(Self::CommandsOnly),
            2 => Ok(Self::Hidden),
            _ => Err(InvalidChatMode),
        }
    }
}

/// Player's current chat session
pub struct ChatSession {
    pub session_id: uuid::Uuid,
    pub expires_at: i64,
    pub public_key: Box<[u8]>,
    pub signature: Box<[u8]>,
    pub messages_sent: i32,
    pub messages_received: i32,
    pub signature_cache: Vec<Box<[u8]>>,
}

impl Default for ChatSession {
    fn default() -> Self {
        Self::new(Uuid::nil(), 0, Box::new([]), Box::new([]))
    }
}

impl ChatSession {
    #[must_use]
    pub fn new(
        session_id: Uuid,
        expires_at: i64,
        public_key: Box<[u8]>,
        key_signature: Box<[u8]>,
    ) -> Self {
        Self {
            session_id,
            expires_at,
            public_key,
            signature: key_signature,
            messages_sent: 0,
            messages_received: 0,
            signature_cache: Vec::new(),
        }
    }
}
