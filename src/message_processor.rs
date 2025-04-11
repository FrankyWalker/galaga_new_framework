use crate::player::Player;
use crate::server::{PadType, PressurePadData};
use crate::ship::Ship;
use crate::structs::Cords;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::mpsc::Receiver;
use tokio::sync::Mutex;

//process received messages from server and executes like player shoot if message was shoot or move left, move right.
pub struct MessageProcessor {
    pressure_threshold: u32,
}

impl MessageProcessor {
    pub fn new(pressure_threshold: u32) -> Self {
        MessageProcessor {
            pressure_threshold,
        }
    }

    pub async fn process_message(
        &self,
        rx_arc: Receiver<PressurePadData>,
    ) -> Option<PressurePadData> {
        let mut rx = rx_arc;
        rx.recv().await
    }

    pub fn should_process_pad(&self, message: &PressurePadData) -> bool {
        message.pressure >= self.pressure_threshold as f32
    }

    pub fn handle_active_pad(
        &self,
        pad_type: &PadType,
        player: &mut Player,
        grid: &mut HashMap<Cords, Box<dyn Ship>>,
    ) {
        match pad_type {
            PadType::Left => player.move_left(),
            PadType::Right => player.move_right(),
            PadType::Shoot => {
                player.shoot(grid);
            },
        }
    }
}