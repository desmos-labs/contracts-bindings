#!/bin/bash

# This script checks if the given file contains "f64" or "f32" operations and returns an error if found.

# Check if file path is provided
if [[ $# -eq 0 ]]; then
    echo "Error: File path not provided"
    exit 1
fi

# Check if file exists
if [[ ! -f $1 ]]; then
    echo "Error: File does not exist"
    exit 1
fi

# Check if file contains "f64." or "f32."
if grep -qE '\<(f64|f32)\.' $1; then
    echo "Error: File contains \"f64\" or \"f32\" operations"
    exit 1
fi

# If file does not contain "f64." or "f32.", return success
echo "File does not contain \"f64\" or \"f32\" operations"
exit 0
