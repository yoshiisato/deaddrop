# Latest DeadDrop Prototype: A Privacy-Preserving Bug Bounty Platform for Smart Contracts on the Ethereum Blockchain

### Complete setup script to run tests end-to-end
Script assumes you have at least 8 cores and more than 64GB of memory (using n4/n2-standard-16 GCP instance in practice)

```
sudo apt update
sudo apt install autoconf automake libtool git cmake g++ libgmp3-dev libntl-dev

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env


mkdir -p ~/deaddrop
cd ~/deaddrop
git clone --branch eu_dos_perfomr_anony --single-branch https://github.com/yoshiisato/ObliviousMessageRetrieval.git


OMRDIR=~/deaddrop
BUILDDIR=$OMRDIR/ObliviousMessageRetrieval/build
cd $OMRDIR && git clone https://github.com/yoshiisato/palisade-release
cd palisade-release
mkdir build
cd build
cmake .. -DCMAKE_INSTALL_PREFIX=$BUILDDIR
make -j$(nproc)
make install


cd $OMRDIR && git clone -b OpenSSL_1_1_1w https://github.com/openssl/openssl
cd openssl
./config --prefix=$BUILDDIR
make -j$(nproc)
make install -j$(nproc)


cd $OMRDIR && git clone https://github.com/wyunhao/SEAL
cd SEAL
cmake -S . -B build -DCMAKE_INSTALL_PREFIX=$BUILDDIR -DSEAL_USE_INTEL_HEXL=ON
cmake --build build -j$(nproc)
cmake --install build


cd $OMRDIR/ObliviousMessageRetrieval/build
mkdir ../data
mkdir ../data/payloads
mkdir ../data/clues
cmake .. -DCMAKE_PREFIX_PATH=$BUILDDIR
make -j$(nproc)
./http_server


# on a separate terminal
cd $OMRDIR && git clone https://github.com/yoshiisato/deaddrop.git
cd deaddrop/deadbug/rust_omr
cargo test (for test)
cargo run --release (for timed run)
```

If ```make -j$(nproc)``` fails under palisades-release with a ```error: ‘numeric_limits’ is not a member of ‘std’``` error, enter this fix:

```
sed -i '/#include "check.h"/a #include <limits>' third-party/google-benchmark/src/benchmark_register.h
```