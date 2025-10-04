cargo build
if [ -f $1.ppm ]; then
    rm $1.ppm
fi
./target/debug/rust-ray-tracing >> $1.ppm
