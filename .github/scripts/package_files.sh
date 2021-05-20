#!/bin/bash
OS="linux"
BIN_PATH="./bin"
OUTPUT_PATH="./build"
ZIP_NAME="build.zip"

# Read in args
while [ $# -gt 0 ]; do
    case "$1" in
        -s) shift && OS=$1 ;;
        -b) shift && BIN_PATH=$1 ;;
        -o) shift && OUTPUT_PATH=$1 ;;
        -z) shift && ZIP_NAME=$1 ;;
        *) echo "Error: Unexpected argument \"$1\"" && exit 1
    esac
    shift
done

ZIP_PATH="$OUTPUT_PATH/$ZIP_NAME"

# Clear previous build
echo ">> Clearing old files at $OUTPUT_PATH"
rm $OUTPUT_PATH -rf

# Create directory
echo ">> Creating output directory at $OUTPUT_PATH"
mkdir -p $OUTPUT_PATH

# Copy licences + README
echo ">> Copying licenses and README"
cp ./LICENSE $OUTPUT_PATH/LICENSE -f
cp ./THIRDPARTY $OUTPUT_PATH/THIRDPARTY -f
cp ./README.md $OUTPUT_PATH/README.md -f

# Copy executables
echo ">> Finding executables at $BIN_PATH"
EXES=$(find $BIN_PATH -maxdepth 1 -type f -executable -print)
for exe in $EXES; do
    echo "\t>> Copying $(basename $exe)"
    cp $exe $OUTPUT_PATH/$(basename $exe) -f
done

# Zip everything up
echo ">> Zipping everything up in $ZIP_PATH"
zip $ZIP_PATH $OUTPUT_PATH -jr