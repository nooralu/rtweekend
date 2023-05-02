#!/bin/bash

if [ -f image.png ]; then
    cp image.png image.backup.png
    rm image.png
fi

cargo run --release | ffmpeg -i pipe:0 image.png -loglevel quiet
