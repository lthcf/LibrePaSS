#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env, Symbol};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Fine(Symbol), // Maps unique Book ID to FineRecord
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FineRecord {
    pub student: Address,
    pub amount: i128,
    pub is_paid: bool,
}

#[contract]
pub struct LibrePassContract;

#[contractimpl]
impl LibrePassContract {
    /// Issues a new fine record for a student regarding an overdue book.
    pub fn issue_fine(env: Env, book_id: Symbol, student: Address, amount: i128) {
        let key = DataKey::Fine(book_id.clone());
        
        // Ensure fine does not already exist
        if env.storage().instance().has(&key) {
            panic!("Fine record already exists for this book.");
        }

        let record = FineRecord {
            student,
            amount,
            is_paid: false,
        };

        env.storage().instance().set(&key, &record);
    }

    /// Pays an outstanding library fine using an external token asset (e.g., USDC).
    pub fn pay_fine(env: Env, book_id: Symbol, payer: Address, token_address: Address) {
        payer.require_auth();

        let key = DataKey::Fine(book_id.clone());
        if !env.storage().instance().has(&key) {
            panic!("Fine record not found.");
        }

        let mut record: FineRecord = env.storage().instance().get(&key).unwrap();
        if record.is_paid {
            panic!("Fine has already been paid.");
        }

        if record.student != payer {
            panic!("Unauthorized: Payer must be the student assigned to the fine.");
        }

        // Initialize token client to handle payment movement
        let token_client = token::Client::new(&env, &token_address);
        
        // Transfer USDC from student to the library contract deployment context treasury
        token_client.transfer(&payer, &env.current_contract_address(), &record.amount);

        // Update record to reflect completed payment and update on-chain storage
        record.is_paid = true;
        env.storage().instance().set(&key, &record);
    }

    /// Read-only check to see if a specific book fine record has been cleared.
    pub fn is_cleared(env: Env, book_id: Symbol) -> bool {
        let key = DataKey::Fine(book_id);
        if !env.storage().instance().has(&key) {
            return true; // No record means clear
        }
        let record: FineRecord = env.storage().instance().get(&key).unwrap();
        record.is_paid
    }
}