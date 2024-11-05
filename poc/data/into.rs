use super::core::Data;

impl Into<Vec<u8>> for Data {
    fn into(self) -> Vec<u8> {
        self.bytes()
    }
}
