rm -rf raves
mkdir raves
cargo objcopy --release -- -O binary raves/payload.bin
cp ../manifest.json raves
/usr/bin/zip -r -m -q raves.zip raves
mv raves.zip raves.ntkdigunit
