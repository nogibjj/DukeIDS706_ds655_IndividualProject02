use prettytable::{Table, Row, Cell};
use rusqlite::{params, Connection, Result};
use std::error::Error;
use std::fs::File;
use std::io::Write;
use csv::Reader;
use std::path::Path;


fn db_delete(db: &str, rowid: i32) -> Result<()> {
    let conn = Connection::open(db)?;

    let tablename = db.split(".").into_iter().next().unwrap();

    let mut stmt1 = conn.prepare(&format!("SELECT * FROM {}", tablename))?;
    let rows_before_delete = stmt1.query_map(rusqlite::NO_PARAMS, |_| Ok(()))?.count();
    println!("# Rows BEFORE delete: {}", rows_before_delete);

    println!("Deleting row # {}...", rowid);
    conn.execute(&format!("DELETE FROM {} WHERE ROWID = ?", tablename), params![rowid])?;

    let mut stmt2 = conn.prepare(&format!("SELECT * FROM {}", tablename))?;
    let rows_after_delete = stmt2.query_map(rusqlite::NO_PARAMS, |_| Ok(()))?.count();
    println!("# Rows AFTER delete: {}", rows_after_delete);

    Ok(())
}


async fn extract(url: &str, file_path: &str) -> Result<(), Box<dyn Error>> {
    let response = reqwest::get(url).await?;
    let bytes = response.bytes().await?;
    let mut file = File::create(file_path)?;

    if let Some(first_byte) = bytes.first() {
        if *first_byte == b',' {
            file.write_all(b"id")?;
        }
    }

    file.write_all(&bytes)?;

    Ok(())
}


fn load(dataset: &str) -> Result<String, Box<dyn Error>> {
    let mut rdr = Reader::from_path(dataset)?;
    let headers = rdr.headers()?.iter().map(|s| s.replace(".", "_")).collect::<Vec<_>>().join(", ");
    let dbname = Path::new(dataset).file_stem().unwrap().to_str().unwrap();

    let conn = Connection::open(format!("{}.db", dbname))?;

    conn.execute(&format!("DROP TABLE IF EXISTS {}", dbname), params![])?;

    println!("Table {} has been dropped if it existed.", dbname);

    conn.execute(&format!("CREATE TABLE {} ({})", dbname, headers), params![])?;

    println!("Table {} has been created.", dbname);

    let mut count_columns = 0;
    for record in rdr.records() {
        let record = record?;
        let values = record.iter().map(|s| format!("'{}'", s)).collect::<Vec<_>>().join(", ");
        conn.execute(&format!("INSERT INTO {} VALUES ({})", dbname, values), params![])?;
        count_columns = record.len();
    }

    println!("{} columns have been added to the table: {}.db", count_columns, dbname);

    Ok(format!("{}.db", dbname))
}



fn db_update(db: &str, colname: &str) -> Result<(), Box<dyn Error>> {
    let tablename = db.split(".").next().unwrap();

    let conn = Connection::open(db)?;

    let mut stmt2 = conn.prepare(&format!("SELECT * FROM {}", tablename))?;
    let rows = stmt2.query_map(params![], |row| {
        let value: String = row.get(0)?;
        Ok(value)
    })?;

    println!("Before update: ");
    for row in rows {
        println!("row: {:?}", row);
    }

    conn.execute(&format!("UPDATE {} SET {} = {} + 1", tablename, colname, colname), params![])?;

    let mut stmt3 = conn.prepare(&format!("SELECT * FROM {}", tablename))?;
    let rows = stmt3.query_map(params![], |row| {
        let value: String = row.get(0)?;
        Ok(value)
    })?;

    println!("After update: ");
    for row in rows {
        println!("row: {:?}", row);
    }

    Ok(())
}



fn query(db: &str) -> Result<(), Box<dyn Error>> {
    let tablename = db.split(".").next().unwrap();

    let conn = Connection::open(db)?;

    // Get column names
    let mut stmt = conn.prepare(&format!("PRAGMA table_info({})", tablename))?;
    let column_names: Result<Vec<String>, _> = stmt.query_map(params![], |row| {
        let name: String = row.get(1)?; // The column name is in the second column of the result set
        Ok(name)
    })?.collect();
    let column_names = column_names?;

    // Prepare the SELECT * statement
    let mut stmt = conn.prepare(&format!("SELECT * FROM {} LIMIT 5", tablename))?;

    let rows = stmt.query_map(params![], |row| {
        let col1: String = row.get(0)?; // sepal.length
        let col2: String = row.get(1)?; // sepal.width
        let col3: String = row.get(2)?; // petal.length
        let col4: String = row.get(3)?; // petal.width
        let col5: String = row.get(4)?; // variety
        Ok((col1, col2, col3, col4, col5))
    })?;
    
    let mut table = Table::new();
    
    for row_result in rows.skip(1) {
        let (col1, col2, col3, col4, col5) = row_result?;
        let row = Row::new(vec![Cell::new(&col1), Cell::new(&col2), Cell::new(&col3), Cell::new(&col4), Cell::new(&col5)]);
        table.add_row(row);
    }
    
    table.printstd();
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // assert!(db_delete("GroceryDB.db", 3).is_ok());
    // extract("https://raw.githubusercontent.com/Barabasi-Lab/GroceryDB/main/data/GroceryDB_IgFPro.csv", "data/GroceryDB.csv").await?;
    extract("https://gist.githubusercontent.com/netj/8836201/raw/6f9306ad21398ea43cba4f7d537619d0e07d5ae3/iris.csv", "../data/Iris_Data.csv").await?;
    load("../data/Iris_Data.csv")?;
    
    match db_delete("Iris_Data.db", 3) {
        Ok(_) => println!("Delete operation successful."),
        Err(e) => println!("Delete operation failed with error: {}", e),
    }

    // load("./data/GroceryDB.csv")?;

    // db_update("GroceryDB.db", "id")?;
    // db_update("Iris_Data.db", "petal_length")?;

    // query("GroceryDB.db")?;
    query("Iris_Data.db")?;

    Ok(())
}