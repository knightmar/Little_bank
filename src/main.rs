mod account_manager;

fn main() {
    let connection = sqlite::open("bank.sqlite").unwrap();
    let mut manager = account_manager::AccountManager::new(connection);

    let mut account = manager.get_accounts_with_name("banane");
    println!("{}",account.len());
    let mut account = account[0].clone();

    manager.credit(&mut account, 100.0);
    println!("Account id : {}", &account.get_id());
    println!("Account owner name : {}", account.get_owner_name());
    println!("Account balance : {}", account.get_balance());
}
