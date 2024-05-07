use redb::{Database, ReadableTable, TableDefinition};
use std::io::{self, BufRead};

const IMU_DATA_TABLE: TableDefinition<String, String> = TableDefinition::new("imu_data");

fn main() -> redb::Result<()> {
    // 获取用户输入的数据库路径
    let mut db_path = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut db_path).unwrap();
    let db_path = db_path.trim(); // 去除多余的换行符

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
