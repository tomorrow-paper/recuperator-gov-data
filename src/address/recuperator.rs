use tomorrow_core::Result;
use tomorrow_recuperator::Recuperator;
use tomorrow_http::*;

use super::models::Record;
use super::{AddressRequest, AddressResponse};

const DATA_GOV_API_URL: &'static str = "https://api-adresse.data.gouv.fr/search";

pub struct AddressRecuperator<T: Requester> {
    requester: T
}

impl <T: Requester> AddressRecuperator<T> {

    pub fn new(requester: T) -> Self {
        AddressRecuperator {
            requester: requester
        }
    }
}

impl <T: Requester> Recuperator<AddressRequest, AddressResponse> for AddressRecuperator<T> {

    fn compute(&self, request: AddressRequest) -> Result<AddressResponse> {
        let query = format!("?q={}", request.query);
        let record = self.requester.request::<Record>(&query)?;

        Ok(AddressResponse::new(record))
    }
}

impl Default for AddressRecuperator<Client> {

    fn default() -> Self {
        let client = Builder::new(DATA_GOV_API_URL).into();
        AddressRecuperator::new(client)
    }
}

#[cfg(test)]
mod tests {
    use ::address::*;
    use tomorrow_recuperator::Recuperator;

    #[test]
    fn test() {
        let query = "42 rue d'Ulm";
        let request = AddressRequest::new(query);

        let recuperator = AddressRecuperator::default();
        let result = recuperator.compute(request);

        assert!(result.is_ok());

        let record = result.unwrap().record;

        assert_eq!(record.query, query);
    }
}