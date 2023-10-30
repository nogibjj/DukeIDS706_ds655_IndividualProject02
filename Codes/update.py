import sqlite3
from query import query


def db_update(db="GroceryDB.db", colname="id"):
    tablename = db.split(".")[0]
    conn = sqlite3.connect(db)
    cursor = conn.cursor()
    cursor.execute("SELECT * FROM " + tablename)
    # r_all = cursor.fetchall()
    print("Before update: ")
    query(db)
    cursor.execute("UPDATE " + tablename + " SET " + colname + "=" + colname + "+ 1")
    # r_all = cursor.fetchall()
    conn.commit()
    print("After update: ")
    query(db)
    conn.close()
    return "Success"
    pass


if __name__ == "__main__":
    assert (db_update()) == "Success"
    assert (db_update("Iris_Data.db", "petal_length")) == "Success"
