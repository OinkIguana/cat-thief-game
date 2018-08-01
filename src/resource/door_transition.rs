use crate::component::door::DoorID;

#[derive(Copy, Clone, Default, Debug)]
pub struct DoorTransition(Option<DoorID>);

impl DoorTransition {
    pub fn new(id: &'static str) -> Self {
        DoorTransition(Some(DoorID(id)))
    }

    pub fn to(&mut self, id: DoorID) {
        self.0 = Some(id);
    }

    pub fn take_target(&mut self) -> Option<DoorID> {
        self.0.take()
    }
}
