use tomorrow_recuperator::Response;

use super::models::Record;

pub struct AddressResponse {
    pub record: Record
}

impl AddressResponse {
    pub fn new(record: Record) -> Self {
        AddressResponse {
            record: record
        }
    }
}

impl Response for AddressResponse {}