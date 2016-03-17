pub mod human;
pub mod mapmemory;

use std::sync::mpsc::Sender;
use common::{Position,EntityId,ChanError,EntityData};
use backend_traits::entity_thread::*;

pub struct EntityResponder {
    pub id: EntityId,
    pub sender: Sender<(EntityId, TickEvent)>,
    pub already_sent: bool,
}

impl EntityResponder {
    fn emit_update_pos(&mut self, pos: Position) -> Result<()>{
        try!(
            self.sender.send((self.id, TickEvent::Move(pos)))
                .map_err(|e| {
                    ChanError::SendError("emit_update_pos")
                })
        );
        self.already_sent = true;
        Ok(())
    }
    fn emit_idle(&mut self) -> Result<()>{
        try!(
            self.sender.send((self.id, TickEvent::Idle))
                .map_err(|e| {
                    ChanError::SendError("emit_idle")
                })
        );
        self.already_sent = true;
        Ok(())
    }
}

impl Drop for EntityResponder {
    fn drop(&mut self) {
        if !self.already_sent {
            self.emit_idle();
        }
    }
}
