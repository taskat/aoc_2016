#!/bin/bash
set -e;

add_input() {
    cd inputs;
    CURRENT=$(($(ls -la | wc -l) -  3));
    DIRNAME="day$((CURRENT + 1))";
    mkdir $DIRNAME;
    cd $DIRNAME;
    touch data.txt data1.txt;
    cd ../..;
}


create_mod() {
    cd src;
    NEW_FILE_NAME="day$((CURRENT + 1)).rs";
    cp raw.rs $NEW_FILE_NAME;
    CURRENT_DAY=$((CURRENT+  1));
    sed -i "s/__/$CURRENT_DAY/g" $NEW_FILE_NAME;
    cd ..;
}

add_mod_decl() {
    cd src;
    LINE="mod day$CURRENT;";
    NEXT_LINE="mod day$((CURRENT + 1));";
    sed -i "/$LINE/a $NEXT_LINE" main.rs;
    cd ..;
}

use_mod() {
    cd src;
    CURRENT_DAY="day$CURRENT";
    NEXT_DAY="day$((CURRENT + 1))";
    sed -i "0,/$CURRENT_DAY/{s/$CURRENT_DAY/$CURRENT_DAY, $NEXT_DAY/}" common.rs;
    LINE="$CURRENT =>.*";
    NEXT_DAY=$((CURRENT + 1));
    NEXT_LINE="??$NEXT_DAY => Box::new(day$NEXT_DAY::Puzzle{}),";
    sed -i "/$LINE/a $NEXT_LINE" common.rs
    sed -i "s/??/        /" common.rs;
    cd ..;
}


add_input;
create_mod;
add_mod_decl;
use_mod;
echo "Succesfully created new day";
