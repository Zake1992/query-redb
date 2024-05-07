use redb::{Database, ReadableTable, TableDefinition};
use std::env;
use std::io::{self, Write};

const IMU_DATA_TABLE: TableDefinition<String, String> = TableDefinition::new("imu_data");

fn main() -> redb::Result<()> {
    let mut db_path = String::new();

    loop {
        let args: Vec<String>;

        if db_path.is_empty() {
            // eprintln!("Usage: {} <database_path>", args[0]);
            args = env::args().collect();
        } else {
            args = vec![String::from("redb"), db_path.clone()];
        }

        // 检查参数数量
        if args.len() < 2 {
            eprintln!("Usage: {} <database_path>", args[0]);
            print!("Please enter the database path: ");
            io::stdout().flush().unwrap(); // Flush stdout to display the prompt immediately
            io::stdin().read_line(&mut db_path).unwrap();
            db_path = db_path.trim().to_string(); // Remove trailing newline
        } else {
            // 获取数据库路径
            db_path = args[1].clone();
            break;
        }
    }

    // 打开数据库
    let db = Database::open(db_path).expect("Failed to open database");

    // 开启只读事务
    let txn = db.begin_read().expect("Failed to begin read transaction");
    let table = txn
        .open_table(IMU_DATA_TABLE)
        .expect("Failed to open table");

    // 遍历表中的键值对并打印
    for entry in table.iter()? {
        let (key, value) = entry?;
        println!("Key: {:?}, Value: {:?}", key.value(), value.value());
    }

    Ok(())
}
