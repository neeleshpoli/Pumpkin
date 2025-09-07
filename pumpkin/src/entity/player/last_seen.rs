use std::sync::Arc;

use pumpkin_protocol::{codec::var_int::VarInt, java::client::play::PreviousMessage};

use crate::entity::player::Player;

#[derive(Clone, Default)]
pub struct LastSeen(pub(super) Vec<Box<[u8]>>);

impl From<LastSeen> for Vec<Box<[u8]>> {
    fn from(seen: LastSeen) -> Self {
        seen.0
    }
}

impl AsRef<[Box<[u8]>]> for LastSeen {
    fn as_ref(&self) -> &[Box<[u8]>] {
        &self.0
    }
}

impl LastSeen {
    /// The sender's `last_seen` signatures are sent as ID's if the recipient has them in their cache.
    /// Otherwise, the full signature is sent. (ID:0 indicates full signature is being sent)
    pub async fn indexed_for(&self, recipient: &Arc<Player>) -> Box<[PreviousMessage]> {
        let mut indexed = Vec::new();
        for signature in &self.0 {
            let index = recipient
                .signature_cache
                .lock()
                .await
                .full_cache
                .iter()
                .position(|s| s == signature);
            if let Some(index) = index {
                indexed.push(PreviousMessage {
                    // Send ID reference to recipient's cache (index + 1 because 0 is reserved for full signature)
                    id: VarInt(1 + index as i32),
                    signature: None,
                });
            } else {
                indexed.push(PreviousMessage {
                    // Send ID as 0 for full signature
                    id: VarInt(0),
                    signature: Some(signature.clone()),
                });
            }
        }
        indexed.into_boxed_slice()
    }
}
