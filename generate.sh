#!/bin/bash

if [ -f image.png ]; then
    cp image.png image.backup.png
fi

cargo run | ffmpeg -i pipe:0 image.png -loglevel quiet
