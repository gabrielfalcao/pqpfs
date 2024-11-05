use super::core::Data;

impl From<Vec<u8>> for Data {
    fn from(data: Vec<u8>) -> Data {
        Data::new(data)
    }
}
impl From<&Vec<u8>> for Data {
    fn from(data: &Vec<u8>) -> Data {
        let data = data.clone();
        Data::new(data)
    }
}
impl From<&[u8]> for Data {
    fn from(data: &[u8]) -> Data {
        let data = data.to_vec();
        Data::new(data)
    }
}
impl From<security_framework::os::macos::passwords::SecKeychainItemPassword> for Data {
    fn from(data: security_framework::os::macos::passwords::SecKeychainItemPassword) -> Data {
        let data = data.to_vec();
        Data::new(data)
    }
}
