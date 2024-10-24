use crate::models::KeyStuff;

pub trait KeyContainer {
    fn key_stuff(&self) -> KeyStuff;
    fn rekey(&mut self, p: KeyStuff);
}
