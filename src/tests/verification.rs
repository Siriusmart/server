// use crate::global::structs::{verification_code_hash, VerificationStore, VerificationType};
//
// #[test]
// fn save_and_load() {
//     dotenv::dotenv().ok();
//
//     let code = String::from("test");
//
//     let hashed_code = verification_code_hash(&code);
//
//     let store = VerificationStore {
//         r#type: VerificationType::AccountCreate { id: String::new(), email: String::new() },
//         expire: 0,
//     };
//
//     store.save(&hashed_code).unwrap();
//
//     let loaded = VerificationStore::load(&code).unwrap();
//
//     assert_eq!(store, loaded);
// }
