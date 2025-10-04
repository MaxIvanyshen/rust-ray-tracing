cargo build

filename="image"

if [[ !$# -eq 0 ]] ; then
    filename=$1
fi
if [ -f $1.ppm ]; then
    rm $1.ppm
fi

./target/debug/rust-ray-tracing >> "$filename.ppm"
echo "Successfully created $filename.ppm"
