// use crate::global::structs::Account;
// use dotenv::dotenv;
//
// #[test]
// fn account_creation() {
//     dotenv().ok();
//
//     let account = Account::new(String::from("test"), String::from("Password"));
//     assert!(!Account::exists(account.user_id.to_string().as_str()));
//     assert!(!Account::exists_username(&account.username));
//     account.save().unwrap();
//     account.save_username().unwrap();
//     assert_eq!(Account::load(&account.user_id.to_string()).unwrap(), account);
// }

// #[test]
// fn access_account() {
//     dotenv().ok();
//
//     let account = Account::new(String::from("test"), String::from("Password"));
//     eprintln!("{}", &Account::load_username(&account.username).unwrap());
//     let _loaded_account = Account::load(&Account::load_username(&account.username).unwrap()).unwrap();
// }
//
// #[test]
// fn username_taken() {
//     dotenv().ok();
//     let username = "test";
//     assert!(Account::exists_username(username));
// }
//
// #[test]
// fn id_taken() {
//     dotenv().ok();
//     let id = "17601865071171904111";
//     assert!(Account::exists(id));
// }
