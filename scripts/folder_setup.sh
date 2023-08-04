#!/bin/bash

# Create the Backend directory and its subdirectories
mkdir -p backend/src/{models,controllers}
touch backend/src/main.rs
cp Cargo.lock backend/
cp Cargo.toml backend/
cp -r src backend/
cp -r tests backend/
cp -r target backend/

# Create the Frontend directory
mkdir -p frontend/src/{components,views}
touch frontend/src/{App.vue,main.js}
touch frontend/package.json

# Create the Loan Underwriting directory
mkdir -p loan-underwriting/src/{models,algorithms,services}
touch loan-underwriting/src/main.rs
touch loan-underwriting/README.md

# Create Data Processing directory
mkdir -p data-processing
touch data-processing/{script1.py,script2.py}
touch data-processing/requirements.txt

# Create Documentation and Script directories
mkdir docs scripts
touch README.md .gitignore

echo "Directories and files created successfully."
