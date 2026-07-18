#!/bin/bash

INPUTS_TXT="./tests/inputs/text_files"
OUT_DIR="./tests/expected"

[[ ! -d "$OUT_DIR" ]] && mkdir -p "$OUT_DIR"

for FILE in $INPUTS_TXT/*.txt; do
    BASENAME=$(basename $FILE)
    cat $FILE > ${OUT_DIR}/${BASENAME}.out
done