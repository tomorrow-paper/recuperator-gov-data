pub mod models;

mod request;
pub use self::request::AddressRequest;

mod response;
pub use self::response::AddressResponse;

mod recuperator;
pub use self::recuperator::AddressRecuperator;