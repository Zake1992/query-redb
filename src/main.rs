use redb::{Database, ReadableTable, TableDefinition};
use std::env;
use std::process;

const IMU_DATA_TABLE: TableDefinition<String, String> = TableDefinition::new("imu_data");

fn main() -> redb::Result<()> {
    // 获取命令行参数
    let args: Vec<String> = env::args().collect();

    // 检查参数数量
    if args.len() < 2 {
        eprintln!("Usage: {} <database_path>", args[0]);
        process::exit(1);
    }

    // 获取数据库路径
    let db_path = &args[1];

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
