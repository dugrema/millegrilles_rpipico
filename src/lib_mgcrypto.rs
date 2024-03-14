use core::str::from_utf8;
use chrono::DateTime;
use defmt::{debug, info, unwrap};
use heapless::Vec;
use millegrilles_cryptographie::ed25519_dalek::SigningKey;
use millegrilles_cryptographie::hachages::{HachageCode, hacher_bytes_into};
use hex;
use millegrilles_cryptographie::ed25519::{signer_into, verifier};
use millegrilles_cryptographie::generateur::MessageMilleGrillesBuilderDefault;
use millegrilles_cryptographie::messages_structs::{MessageMilleGrillesBufferHeapless, CONST_BUFFER_MESSAGE_MIN, CONST_NOMBRE_CERTIFICATS_MAX, RoutageMessage, MessageKind};

const MESSAGE_1: &str = r#"{
      "id": "d49a375c980f1e70cdea697664610d70048899d1428909fdc29bd29cfc9dd1ca",
      "pubkey": "d1d9c2146de0e59971249489d971478050d55bc913ddeeba0bf3c60dd5b2cd31",
      "estampille": 1710338722,
      "kind": 5,
      "contenu": "{\"domaine\":\"CoreMaitreDesComptes\",\"exchanges_routing\":null,\"instance_id\":\"f861aafd-5297-406f-8617-f7b8809dd448\",\"primaire\":true,\"reclame_fuuids\":false,\"sous_domaines\":null}",
      "routage": {
        "action": "presenceDomaine",
        "domaine": "CoreMaitreDesComptes"
      },
      "sig": "9ff0c6443c9214ab9e8ee2d26b3ba6453e7f4f5f59477343e1b0cd747535005b13d453922faad1388e65850a1970662a69879b1b340767fb9f4bda6202412204",
      "certificat": [
        "-----BEGIN CERTIFICATE-----\nMIIClDCCAkagAwIBAgIUQuFP9EOrsQuFkWnXEH8UQNZ1EN4wBQYDK2VwMHIxLTAr\nBgNVBAMTJGY4NjFhYWZkLTUyOTctNDA2Zi04NjE3LWY3Yjg4MDlkZDQ0ODFBMD8G\nA1UEChM4emVZbmNScUVxWjZlVEVtVVo4d2hKRnVIRzc5NmVTdkNUV0U0TTQzMml6\nWHJwMjJiQXR3R203SmYwHhcNMjQwMjIwMTE0NjUzWhcNMjQwMzIyMTE0NzEzWjCB\ngTEtMCsGA1UEAwwkZjg2MWFhZmQtNTI5Ny00MDZmLTg2MTctZjdiODgwOWRkNDQ4\nMQ0wCwYDVQQLDARjb3JlMUEwPwYDVQQKDDh6ZVluY1JxRXFaNmVURW1VWjh3aEpG\ndUhHNzk2ZVN2Q1RXRTRNNDMyaXpYcnAyMmJBdHdHbTdKZjAqMAUGAytlcAMhANHZ\nwhRt4OWZcSSUidlxR4BQ1VvJE93uugvzxg3Vss0xo4HdMIHaMCsGBCoDBAAEIzQu\nc2VjdXJlLDMucHJvdGVnZSwyLnByaXZlLDEucHVibGljMAwGBCoDBAEEBGNvcmUw\nTAYEKgMEAgREQ29yZUJhY2t1cCxDb3JlQ2F0YWxvZ3VlcyxDb3JlTWFpdHJlRGVz\nQ29tcHRlcyxDb3JlUGtpLENvcmVUb3BvbG9naWUwDwYDVR0RBAgwBoIEY29yZTAf\nBgNVHSMEGDAWgBRQUbOqbsQcXmnk3+moqmk1PXOGKjAdBgNVHQ4EFgQU4+j+8rBR\nK+WeiFzo6EIR+t0C7o8wBQYDK2VwA0EAab2vFykbUk1cWugRd10rGiTKp/PKZdG5\nX+Y+lrHe8AHcrpGGtUV8mwwcDsRbw2wtRq2ENceNlQAcwblEkxLvCA==\n-----END CERTIFICATE-----\n",
        "-----BEGIN CERTIFICATE-----\nMIIBozCCAVWgAwIBAgIKAnY5ZhNJUlVzaTAFBgMrZXAwFjEUMBIGA1UEAxMLTWls\nbGVHcmlsbGUwHhcNMjQwMTMwMTM1NDU3WhcNMjUwODEwMTM1NDU3WjByMS0wKwYD\nVQQDEyRmODYxYWFmZC01Mjk3LTQwNmYtODYxNy1mN2I4ODA5ZGQ0NDgxQTA/BgNV\nBAoTOHplWW5jUnFFcVo2ZVRFbVVaOHdoSkZ1SEc3OTZlU3ZDVFdFNE00MzJpelhy\ncDIyYkF0d0dtN0pmMCowBQYDK2VwAyEAPUMU7tlz3HCEB+VzG8NVFQ/nFKjIOZmV\negt+ub3/7SajYzBhMBIGA1UdEwEB/wQIMAYBAf8CAQAwCwYDVR0PBAQDAgEGMB0G\nA1UdDgQWBBRQUbOqbsQcXmnk3+moqmk1PXOGKjAfBgNVHSMEGDAWgBTTiP/MFw4D\nDwXqQ/J2LLYPRUkkETAFBgMrZXADQQB6S4tids+r9e5d+mwpdkrAE2k3+8H0x65z\nWD5eP7A2XeEr0LbxRPNyaO+Q8fvnjjCKasn97MTPSCXnU/4JbWYK\n-----END CERTIFICATE-----\n"
      ]
    }"#;

pub fn test_hachage_1() {
    let data = b"Test Data4";
    let mut hachage = [0u8; 32];  // 32 bytes Blake2s
    hacher_bytes_into(&data[..], HachageCode::Blake2s256, &mut hachage);
    let mut buf_hachage = [0u8; 64];
    hex::encode_to_slice(hachage, &mut buf_hachage).unwrap();
    let hex_hachage = from_utf8(&buf_hachage).unwrap();
    info!("Hex hachage : {}", hex_hachage);
}

pub fn test_signer_into() {
    let data_str = "7497da22a374d7ab092b8a6fa89709739f3fe0d07921a738d376079d4632a102";
    let mut data_bytes = [0u8; 32];
    hex::decode_to_slice(data_str, &mut data_bytes).unwrap();

    // Charger private key
    let signing_key = SigningKey::from_bytes(b"01234567890123456789012345678901");
    let verifying_key = signing_key.verifying_key();

    let mut signature_buffer = [0u8; 128];
    let signature_str = signer_into(&signing_key, &data_bytes, &mut signature_buffer);
    info!("Signature {}", signature_str);

    // Verifier
    let resultat = verifier(&verifying_key, &data_bytes, &signature_str);
    info!("Resultat verif : {}", resultat);

    // Signature corrompue
    // let signature_invalide_str = "c532ba9f1ab2c19eea526baf5c865c98894c4fa06952987192e815a5c000357437bd6d6faa95423b6264fcf9dc0e2019294c9f9dc501261e7324ecf2603b2e0a";
    // let resultat = verifier(&verifying_key, &data_bytes, &signature_invalide_str);
    // info!("Test Signature corrompue (doit etre false): {}", resultat);
}

pub fn test_buffer_heapless<const B: usize, const C: usize>(buffer: &mut MessageMilleGrillesBufferHeapless<B, C>) {
    buffer.buffer.clear();
    buffer.buffer.extend_from_slice(MESSAGE_1.as_bytes()).unwrap();
    let mut parsed = buffer.parse().unwrap();
    parsed.verifier_signature().unwrap();
    info!("Parsed id: {}", parsed.id);
}

pub fn test_build_into_u8<const B: usize, const C: usize>(buffer: &mut MessageMilleGrillesBufferHeapless<B, C>) {
    let contenu = "Le contenu a inclure";
    let estampille = DateTime::from_timestamp(1710338722, 0).unwrap();
    let signing_key = SigningKey::from_bytes(b"01234567890123456789012345678901");
    let routage = RoutageMessage::for_action("Test", "test");
    let mut certificat: Vec<&str, CONST_NOMBRE_CERTIFICATS_MAX> = Vec::new();
    certificat.push("CERTIFICAT 1");
    certificat.push("CERTIFICAT 2");

    let generateur = MessageMilleGrillesBuilderDefault::new(
        MessageKind::Commande, contenu, estampille, &signing_key)
        .routage(routage)
        .certificat(certificat);

    // let mut buffer: Vec<u8, CONST_BUFFER_MESSAGE_MIN> = Vec::new();
    buffer.buffer.clear();
    let message = generateur.build_into(&mut buffer.buffer).unwrap();

    info!("Message genere id : {}", message.id);
}
