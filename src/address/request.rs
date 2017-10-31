use tomorrow_recuperator::Request;

pub struct AddressRequest {
    pub query: String
}

impl AddressRequest {
    pub fn new(query: &str) -> Self {
        AddressRequest {
            query: String::from(query)
        }
    }
}

impl Request for AddressRequest {}