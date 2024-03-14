// use defmt::debug;
// use serde::{Deserialize, Serialize};
//
// #[derive(Serialize, Deserialize)]
// pub struct FormatMessageTest1<'a> {
//     pub valeur_text: &'a str,
//     pub valeur_num: i32,
// }
//
// // pub fn test_parse(contenu: &str) -> Result<(), Error> {
// pub fn test_parse(contenu: &str) {
//     debug!("test_parse");
//     let valeur: (FormatMessageTest1, usize) = match(serde_json_core::from_str(contenu)) {
//         Ok(inner) => inner,
//         Err(e) => {
//             panic!("Erreur {}", e);
//         }
//     };
//     debug!("Valeur parsed : string = {}, num = {}", valeur.0.valeur_text, valeur.0.valeur_num);
// }
//
// pub fn test_stringify(valeur: FormatMessageTest1) {
//     debug!("test_stringify");
//     match serde_json_core::to_string::<FormatMessageTest1, 16384>(&valeur) {
//         Ok(inner) => debug!("String : {}", inner.as_str()),
//         Err(e) => panic!("Erreur stringify {}", e)
//     }
// }
