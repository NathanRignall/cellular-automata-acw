#!/bin/bash

# Run texcount command and capture the output
expression=$(texcount -inc -1 -relaxed report.tex)

# Extracting the number from the expression
result=$(echo "$expression" | awk -F'+' '{print $1}')

echo "$result"

