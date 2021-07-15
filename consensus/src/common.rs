use mina_types::protocol_state::{Header, ProtocolState};

pub type ProtocolStateChain = Vec<ProtocolState>;

pub trait Chain<T>
where
    T: Header,
{
    fn push(&self, new: T) -> Result<(), &'static str>;
}

impl Chain<ProtocolState> for ProtocolStateChain {
    fn push(&self, new: ProtocolState) -> Result<(), &'static str> {
        match self.len() {
            0 => self.push(new),
            n => {
                if new.get_height() < self[n - 1].get_height() {
                    return Err("cannot push header with lower height than top");
                }

                self.push(new)
            }
        }
    }
}
