import sqlite3


def db_delete(db="GroceryDB.db", rowid=3):
    tablename = db.split(".")[0]
    conn = sqlite3.connect(db)
    cursor = conn.cursor()
    cursor.execute("SELECT * FROM " + tablename)
    r_all = cursor.fetchall()
    print("# Rows BEFORE delete: ", len(r_all))
    print("Deleting row # " + str(rowid) + "...")
    cursor.execute("DELETE FROM " + tablename + " WHERE ROWID = " + str(rowid))
    cursor.execute("SELECT * FROM " + tablename)
    r_all = cursor.fetchall()
    print("# Rows AFTER delete: ", len(r_all))
    conn.commit()

    conn.close()
    return "Success"
    pass


if __name__ == "__main__":
    assert (db_delete()) == "Success"
    assert (db_delete("Iris_Data.db", 3)) == "Success"
    # db_delete()

# db_delete()
