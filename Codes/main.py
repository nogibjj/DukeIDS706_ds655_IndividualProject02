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
    extract()

# Transform and load
elif args.step == 2:
    print("Transforming data...")
    load()

# Query
elif args.step == 3:
    print("Querying data...")
    query()

else:
    print("Invalid step number")
