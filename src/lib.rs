extern crate tomorrow_core;
extern crate tomorrow_recuperator;
extern crate tomorrow_http;

extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

pub mod address;

#[cfg(test)]
mod tests {
    
    use tomorrow_http::json::mock::MockClient;
    use tomorrow_recuperator::Recuperator;

    use ::address::*;

    #[test]
    fn test() {
        let query = "42 rue d'Ulm";
        let request = AddressRequest::new(query);

        let client = MockClient::with_json(r#"{"attribution": "BAN", "licence": "ODbL 1.0", "version": "draft", "limit": 5, "features": [{"properties": {"street": "Rue d'Ulm", "city": "Paris", "name": "42 Rue d'Ulm", "label": "42 Rue d'Ulm 75005 Paris", "postcode": "75005", "id": "ADRNIVX_0000000270800083", "x": 651871.8, "y": 6860483.1, "housenumber": "42", "context": "75, Paris, \u00cele-de-France", "type": "housenumber", "importance": 0.3705, "citycode": "75105", "score": 0.8518636363636364}, "geometry": {"coordinates": [2.344236, 48.842597], "type": "Point"}, "type": "Feature"}, {"properties": {"street": "Rue d'Ulm", "city": "Montlu\u00e7on", "name": "42 Rue d'Ulm", "label": "42 Rue d'Ulm 03100 Montlu\u00e7on", "postcode": "03100", "id": "ADRNIVX_0000000285215629", "x": 668435.8, "y": 6583981.5, "housenumber": "42", "context": "03, Allier, Auvergne-Rh\u00f4ne-Alpes (Auvergne)", "type": "housenumber", "importance": 0.2724, "citycode": "03185", "score": 0.8429454545454546}, "geometry": {"coordinates": [2.589507, 46.355021], "type": "Point"}, "type": "Feature"}, {"properties": {"id": "85191_XXXX_2a070e", "name": "Rue d'Ulm", "label": "Rue d'Ulm 85000 La Roche-sur-Yon", "postcode": "85000", "x": 361698.6, "y": 6629736.4, "importance": 0.2973, "score": 0.6220685950413223, "type": "street", "city": "La Roche-sur-Yon", "citycode": "85191", "context": "85, Vend\u00e9e, Pays-de-la-Loire"}, "geometry": {"coordinates": [-1.428419, 46.682316], "type": "Point"}, "type": "Feature"}, {"properties": {"id": "11069_XXXX_30b1b8", "name": "Rue d'Ulm", "label": "Rue d'Ulm 11000 Carcassonne", "postcode": "11000", "x": 647599.0, "y": 6236650.2, "importance": 0.2096, "score": 0.6140958677685949, "type": "street", "city": "Carcassonne", "citycode": "11069", "context": "11, Aude, Occitanie (Languedoc-Roussillon)"}, "geometry": {"coordinates": [2.355407, 43.22713], "type": "Point"}, "type": "Feature"}, {"properties": {"id": "60159_XXXX_c55e84", "name": "Rue d'Ulm", "label": "Rue d'Ulm 60200 Compi\u00e8gne", "postcode": "60200", "x": 687738.4, "y": 6924504.4, "importance": 0.2068, "score": 0.6138413223140495, "type": "street", "city": "Compi\u00e8gne", "citycode": "60159", "context": "60, Oise, Hauts-de-France (Picardie)"}, "geometry": {"coordinates": [2.831062, 49.419881], "type": "Point"}, "type": "Feature"}], "query": "42 rue d'Ulm", "type": "FeatureCollection"}"#);
        let recuperator = AddressRecuperator::new(client);
        
        let response = recuperator.compute(request);
        assert!(response.is_ok());

        let record = response.unwrap().record;
        assert_eq!(record.query, query);
        assert_eq!(record.features.len(), 5);
    }
}