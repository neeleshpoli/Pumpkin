use std::sync::{Arc, atomic::Ordering};

use async_trait::async_trait;
use pumpkin_inventory::{
    player::player_screen_handler::PlayerScreenHandler,
    screen_handler::{
        ScreenHandler, ScreenHandlerBehaviour, ScreenHandlerFactory, ScreenHandlerListener,
    },
};
use pumpkin_protocol::java::client::play::{CCloseContainer, COpenScreen};
use pumpkin_world::item::ItemStack;
use tokio::sync::Mutex;

use crate::entity::player::Player;

impl Player {
    pub fn increment_screen_handler_sync_id(&self) {
        let current_id = self.screen_handler_sync_id.load(Ordering::Relaxed);
        self.screen_handler_sync_id
            .store(current_id % 100 + 1, Ordering::Relaxed);
    }

    pub async fn close_handled_screen(&self) {
        self.client
            .enqueue_packet(&CCloseContainer::new(
                self.current_screen_handler
                    .lock()
                    .await
                    .lock()
                    .await
                    .sync_id()
                    .into(),
            ))
            .await;
        self.on_handled_screen_closed().await;
    }

    pub async fn on_handled_screen_closed(&self) {
        self.current_screen_handler
            .lock()
            .await
            .lock()
            .await
            .on_closed(self)
            .await;

        let player_screen_handler: Arc<Mutex<dyn ScreenHandler>> =
            self.player_screen_handler.clone();
        let current_screen_handler: Arc<Mutex<dyn ScreenHandler>> =
            self.current_screen_handler.lock().await.clone();

        if !Arc::ptr_eq(&player_screen_handler, &current_screen_handler) {
            player_screen_handler
                .lock()
                .await
                .copy_shared_slots(current_screen_handler)
                .await;
        }

        *self.current_screen_handler.lock().await = self.player_screen_handler.clone();
    }

    pub async fn on_screen_handler_opened(&self, screen_handler: Arc<Mutex<dyn ScreenHandler>>) {
        let mut screen_handler = screen_handler.lock().await;

        screen_handler
            .add_listener(self.screen_handler_listener.clone())
            .await;

        screen_handler
            .update_sync_handler(self.screen_handler_sync_handler.clone())
            .await;
    }

    pub async fn open_handled_screen(
        &self,
        screen_handler_factory: &dyn ScreenHandlerFactory,
    ) -> Option<u8> {
        if !self
            .current_screen_handler
            .lock()
            .await
            .lock()
            .await
            .as_any()
            .is::<PlayerScreenHandler>()
        {
            self.close_handled_screen().await;
        }

        self.increment_screen_handler_sync_id();

        if let Some(screen_handler) = screen_handler_factory
            .create_screen_handler(
                self.screen_handler_sync_id.load(Ordering::Relaxed),
                &self.inventory,
                self,
            )
            .await
        {
            let screen_handler_temp = screen_handler.lock().await;
            self.client
                .enqueue_packet(&COpenScreen::new(
                    screen_handler_temp.sync_id().into(),
                    (screen_handler_temp
                        .window_type()
                        .expect("Can't open PlayerScreenHandler") as i32)
                        .into(),
                    &screen_handler_factory.get_display_name(),
                ))
                .await;
            drop(screen_handler_temp);
            self.on_screen_handler_opened(screen_handler.clone()).await;
            *self.current_screen_handler.lock().await = screen_handler;
            Some(self.screen_handler_sync_id.load(Ordering::Relaxed))
        } else {
            //TODO: Send message if spectator

            None
        }
    }
}

pub(super) struct ScreenListener;

#[async_trait]
impl ScreenHandlerListener for ScreenListener {
    async fn on_slot_update(
        &self,
        _screen_handler: &ScreenHandlerBehaviour,
        _slot: u8,
        _stack: ItemStack,
    ) {
        //println!("Slot updated: {slot:?}, {stack:?}");
    }
}
