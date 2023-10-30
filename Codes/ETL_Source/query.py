"""Query the database"""

import sqlite3
from prettytable import PrettyTable  # For formatting


def query(db="GroceryDB.db"):
    """Query the database for the top 5 rows of the db table"""
    tablename = db.split(".")[0]
    conn = sqlite3.connect(db)
    cursor = conn.cursor()
    # Getting top 5 rows of the table
    cursor.execute("SELECT * FROM " + tablename + " LIMIT 5")
    print("Top 5 rows of the " + tablename + " table:")
    # print([i[0] for i in cursor.description])  # Printing Headers
    r_all = cursor.fetchall()
    x = PrettyTable()
    x.field_names = [i[0] for i in cursor.description]
    for r in r_all:
        x.add_row(r)
    print(x)
    # print(cursor.fetchall())  # Printing query output
    conn.close()
    return "Success"


if __name__ == "__main__":
    query()
# query()
# query("Iris_Data.db")
