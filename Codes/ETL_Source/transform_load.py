"""
Transforms and Loads data into the local SQLite3 database
Example:
general name,
count_products,
ingred_FPro,
avg_FPro_products,
avg_distance_root,
ingred_normalization_term,
semantic_tree_name,
semantic_tree_node # noqa: E501
"""
import sqlite3
import csv

# import os
from pathlib import Path


# load the csv file and insert into a new sqlite3 database
def load(dataset="./data/GroceryDB.csv"):
    """ "Transforms and Loads data into the local SQLite3 database"""

    # print(os.getcwd())
    headers = ""
    #   Reading the provided file [default is GroceryDB] into 'payload'
    payload = csv.reader(open(dataset, newline=""), delimiter=",")
    for row in payload:
        headers = ", ".join(row)
        break
    dbname = Path(dataset).stem
    # dbname is the name of the database
    conn = sqlite3.connect(dbname + ".db")
    c = conn.cursor()
    c.execute("DROP TABLE IF EXISTS " + dbname)
    print("Table " + dbname + " has been dropped if it existed.")
    c.execute("CREATE TABLE " + dbname + "(" + headers.replace(".", "_") + ")")
    print("Table " + dbname + " has been created.")
    # insert
    count_columns = 1
    for char in headers:
        if char == ",":
            count_columns += 1
    questions = ",".join("?" for i in range(count_columns))
    c.executemany(
        "INSERT INTO " + str(dbname) + " VALUES (" + questions + ")",
        payload,
    )
    print(
        str(count_columns) + " columns have been added to the table: " + dbname + ".db"
    )
    conn.commit()
    conn.close()
    return dbname + ".db"


if __name__ == "__main__":
    load()

# load("./data/Iris_Data.csv")
# load()
