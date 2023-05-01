if (Test-Path image.png) { 
    Copy-Item image.png image.backup.png
    Remove-Item image.png 
}

cargo run | ffmpeg -i pipe:0 image.png -loglevel quiet
