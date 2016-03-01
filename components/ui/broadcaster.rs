pub trait Broadcaster {
    fn broadcast(&mut self);
}

pub struct PrintBroadcaster {
    msg: String,
}

impl PrintBroadcaster {
    pub fn new(msg: String) -> Self {
        PrintBroadcaster{
            msg: msg,
        }
    }
}

impl Broadcaster for PrintBroadcaster {
    fn broadcast(&mut self) {
        println!("BCAST: {}", self.msg);
    }
}
