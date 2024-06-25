#!bin/sh
(cargo run --release | tee image.ppm > image.txt)