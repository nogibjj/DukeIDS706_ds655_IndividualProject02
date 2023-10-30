# Rust CLI Binary with SQLite 

*Status Badges*

[![Build](https://github.com/nogibjj/DukeIDS706_ds655_IndividualProject02/actions/workflows/01_Install.yml/badge.svg)](https://github.com/nogibjj/DukeIDS706_ds655_IndividualProject02/actions/workflows/01_Install.yml) [![Lint](https://github.com/nogibjj/DukeIDS706_ds655_IndividualProject02/actions/workflows/03_Lint.yml/badge.svg)](https://github.com/nogibjj/DukeIDS706_ds655_IndividualProject02/actions/workflows/03_Lint.yml) [![Black Formatter](https://github.com/nogibjj/DukeIDS706_ds655_IndividualProject02/actions/workflows/02_Format.yml/badge.svg)](https://github.com/nogibjj/DukeIDS706_ds655_IndividualProject02/actions/workflows/02_Format.yml) [![Test](https://github.com/nogibjj/DukeIDS706_ds655_IndividualProject02/actions/workflows/04_Test.yml/badge.svg)](https://github.com/nogibjj/DukeIDS706_ds655_IndividualProject02/actions/workflows/04_Test.yml) [![Build and Test](https://github.com/nogibjj/DukeIDS706_ds655_IndividualProject02/actions/workflows/05_Rust.yml/badge.svg)](https://github.com/nogibjj/DukeIDS706_ds655_IndividualProject02/actions/workflows/05_Rust.yml)


## 1. Rust Source Code
The `main.rs` code in the `Rust_Codes` folder demonstrates Rust's syntax for declaring variables, constants, arrays, vectors, structs, enums, and functions. It also demonstrates Rust's unique features such as type inference, mutability, shadowing, and pattern matching.

## 2. Using Github Copilot
## 3. SQLite Database 
CRUD/ETL operations on the iris dataset are being performed using the python files (extract, transform_load, update, delete, and query) in the `ETL_Source` part of the `Codes` folder.  
## 4. Optimized Rust Binary [Download Binary](https://github.com/nogibjj/DukeIDS706_ds655_IndividualProject02/releases/latest/download/my-binary)

To use this project, follow these steps:

1. Clone the repository
2. Build the binary using `cargo build --release`
3. Run the binary using `./target/release/my-binary`


## 5. README
## 6. Github Actions
Status badges for the workflows can be found at the top of the README file
## 7. Video Demo
Repo Summary video - [Link]()







Files in this repository include:


### 1. Readme
  The `README.md` file is a markdown file that contains basic information about the repository, what files it contains, and how to consume them


### 2. Requirements
  The `requirements.txt` file has a list of packages to be installed for any required project. Currently, my requirements file contains some basic python packages.


### 3. Codes
  This folder contains all the code files used in this repository - the files named "Test_" will be used for testing and the remaining will define certain functions
  * `extract.py` - Extracts a dataset from a CSV url (currently testing on the iris dataset)
  * `transform_load.py` - Transorms and Loads the data read from the above code into a .db file
  * `update.py` - updates a certain value in the table (based on the colname argument)
  * `delete.py` - deletes a certain row in the table (based on the rowid argument)
  * `query.py` - prints the top 5 rows of the dataset
  * `main.py` - Runs all the above codes in order


### 4. Resources
  -  This folder contains any other files relevant to this project. Currently, I have added -


### 5. CI/CD Automation Files


  #### 5(a). Makefile
  The `Makefile` contains instructions for installing packages (specified in `requirements.txt`), formatting the code (using black formatting), testing the code (running all the sample python code files starting with the term *'Check...'* ), and linting the code using pylint


  #### 5(b). Github Actions
  Github Actions uses the `main.yml` file to call the functions defined in the Makefile based on triggers such as push or pull. Currently, every time a change is pushed onto the repository, it runs the install packages, formatting the code, linting the code, and then testing the code functions


  #### 5(c). Devcontainer
  
  The `.devcontainer` folder mainly contains two files - 
  * `Dockerfile` defines the environment variables - essentially it ensures that all collaborators using the repository are working on the same environment to avoid conflicts and version mismatch issues
  * `devcontainer.json` is a json file that specifies the environment variables including the installed extensions in the virtual environment
