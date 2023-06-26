use sqlite::Connection;

pub struct AccountManager {
    connection: Connection,
}

impl AccountManager {
    pub fn new(connection: Connection) -> AccountManager {
        AccountManager { connection }
    }


    /// # Arguments
    /// This function update the balance of an account
    ///
    /// * `account`: The account to update
    /// * `new_balance`: The new balance of the account
    ///
    /// # Examples
    ///
    /// ```
    /// self.update_balance(&account, account.balance);
    /// ```
    fn update_balance(&mut self, account: &Account, new_balance: f64) {
        self.connection
            .execute(
                "UPDATE bank SET balance = ".to_string()
                    + new_balance.to_string().as_str()
                    + " WHERE id = "
                    + account.get_id().to_string().as_str()
                    + ";",
            )
            .unwrap();
    }

    pub fn debit(&mut self, account: &mut Account, amount: f64) {
        account.balance -= amount;
        self.update_balance(&account, account.balance);
    }

    pub fn credit(&mut self, account: &mut Account, amount: f64) {
        account.balance += amount;
        self.update_balance(&account, account.balance);
    }



    /// # Arguments
    /// Return an Account with the id given in parameter
    /// * `id`: i32 that represents the account id (unique)
    ///
    /// returns: Account
    ///
    /// # Examples
    ///
    /// ```
    /// let mut account = manager.get_account_with_id(1);
    /// ```
    pub fn get_account_with_id(&self ,id: i32) -> Account {
        let mut owner_name = String::from("");
        let mut balance = 0.0;
        self.connection
            .iterate(
                "SELECT * FROM bank WHERE id = ".to_string() + id.to_string().as_str() + ";",
                |pairs| {
                    for &(name, value) in pairs.iter() {
                        match name {
                            "owner_name" => owner_name = value.unwrap().to_string(),
                            "balance" => balance = value.unwrap().parse::<f64>().unwrap(),
                            _ => {}
                        }
                    }
                    true
                },
            )
            .unwrap();

        Account::new(id, owner_name, balance)
    }
    /// Return a Vec<Account> with all the Accounts with the name given in parameter in the database
    ///
    /// # Arguments
    ///
    /// * `name`: &str that represents the account owner name
    ///
    /// returns: Vec<Account> with all the Accounts with the name given in parameter in the database, or an empty Vec<Account> if the name is not present in the database
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```



    pub fn get_accounts_with_name(&self, name: &str, ) -> Vec<Account> {
        let mut result: Vec<Account> = Vec::new();

        let _ = self.connection
            .iterate(
                "SELECT * FROM bank WHERE owner_name = '".to_string() + name + "';",
                |pairs| {
                    let mut id = 0;
                    let mut balance = 0.0;

                    for &(name, value) in pairs.iter() {
                        match name {
                            "id" => id = value.unwrap().parse().unwrap(),
                            "balance" => balance = value.unwrap().parse::<f64>().unwrap(),
                            _ => {}
                        }
                    }
                    result.push(Account::new(id, String::from(name), balance));
                    true
                },
            );
        result
    }
}

/// Account struct:
///
/// id : i32 that represents the account id (unique)
/// owner_name : String that represents the account owner name
/// balance : f64 that represents the account balance
#[derive(Debug, Clone)]
pub struct Account {
    id: i32,
    owner_name: String,
    balance: f64,
}
impl Account {
    pub fn new(id: i32, owner_name: String, balance: f64) -> Account {
        Account {
            id,
            owner_name,
            balance,
        }
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }
    pub fn get_owner_name(&self) -> &str {
        self.owner_name.as_str()
    }
    pub fn get_balance(&self) -> f64 {
        self.balance
    }
}

