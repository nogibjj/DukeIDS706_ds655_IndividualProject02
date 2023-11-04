"""
ETL-Query script
"""

from ETL_Source.extract import extract
from ETL_Source.transform_load import load
from ETL_Source.query import query
import argparse

parser = argparse.ArgumentParser(description="Extract, Transform, or Load data")
parser.add_argument(
    "--step", type=int, help="Step number to run (1 - Extract, 2 - Load, or 3 - Query)"
)
args = parser.parse_args()
# Extract
if args.step == 1:
    print("Extracting data...")
    extract("https://gist.githubusercontent.com/netj/8836201/raw/6f9306ad21398ea43cba4f7d537619d0e07d5ae3/iris.csv","../data/Iris_Data.csv",)

# Transform and load
elif args.step == 2:
    print("Transforming data...")
    load(extract(
                    "https://gist.githubusercontent.com/netj/8836201/raw/6f9306ad21398ea43cba4f7d537619d0e07d5ae3/iris.csv",
                    "../data/Iris_Data.csv",
                ))

# Query
elif args.step == 3:
    print("Querying data...")
    query(    load(extract(
                    "https://gist.githubusercontent.com/netj/8836201/raw/6f9306ad21398ea43cba4f7d537619d0e07d5ae3/iris.csv",
                    "../data/Iris_Data.csv",
                )))

else:
    print("Invalid step number")
