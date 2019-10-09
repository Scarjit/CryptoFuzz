rm -rf in
rm -rf out
mkdir out
mkdir in

cd in
./make_input.sh
cd ..

nohup cargo afl fuzz -i in -o out -M master_fuzzer target/debug/CryptoFuzz &
nohup cargo afl fuzz -i in -o out -S slave_1 target/debug/CryptoFuzz &
nohup cargo afl fuzz -i in -o out -S slave_2 target/debug/CryptoFuzz &
nohup cargo afl fuzz -i in -o out -S slave_3 target/debug/CryptoFuzz &
nohup cargo afl fuzz -i in -o out -S slave_4 target/debug/CryptoFuzz &
nohup cargo afl fuzz -i in -o out -S slave_5 target/debug/CryptoFuzz &
nohup cargo afl fuzz -i in -o out -S slave_6 target/debug/CryptoFuzz &
nohup cargo afl fuzz -i in -o out -S slave_7 target/debug/CryptoFuzz &
nohup cargo afl fuzz -i in -o out -S slave_8 target/debug/CryptoFuzz &
nohup cargo afl fuzz -i in -o out -S slave_9 target/debug/CryptoFuzz &
