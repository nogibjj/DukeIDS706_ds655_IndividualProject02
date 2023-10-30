"""
Code to test the ETL pipeline
"""

from ETL_Source.extract import extract
from ETL_Source.transform_load import load
from ETL_Source.query import query


def test_etl():
    # assert (query(load(extract()))) == "Success"
    assert (
        query(
            load(
                extract(
                    "https://gist.githubusercontent.com/netj/8836201/raw/6f9306ad21398ea43cba4f7d537619d0e07d5ae3/iris.csv",
                    "data/Iris_Data.csv",
                )
            )
        )
    ) == "Success"


if __name__ == "__main__":
    test_etl()
