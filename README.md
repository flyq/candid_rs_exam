# IC_RS

## build
```sh
cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/ic_rs`
type A = record { a : nat16; b : nat16 };
service : {
  add : (nat16, nat16) -> (A) query;
  greeting : (text, nat16) -> (text) query;
  sum : (nat16, nat16) -> (nat16, nat16) query;
  sum2 : (nat16, nat16) -> (nat16) query;
  sum3 : (nat16, nat16) -> (record { nat16; nat16 }) query;
}

cargo run > ic_rs.did
    Finished dev [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/ic_rs`

cargo build --release --target wasm32-unknown-unknown
   Compiling ic_rs v0.1.0 (/home/peter/ic_rs)
    Finished release [optimized] target(s) in 0.71s

ic-cdk-optimizer target/wasm32-unknown-unknown/release/ic_rs.wasm -o ./ic_rs_opt.wasm
Original:          1.79 MiB
Stripping Unused Data Segments...
    Size:          311.13 KiB (83.0% smaller)
Execute a binaryen optimization pass on your WASM....
    Size:          280.72 KiB (9.8% smaller)

Final Size: 280.72 KiB (84.7% smaller)
```

## install

```sh
sudo dfx start --clean
```


```sh
sudo dfx canister --no-wallet create --all
Creating canister "ic_rs"...
"ic_rs" canister created with canister id: "rwlgt-iiaaa-aaaaa-aaaaa-cai"

sudo dfx canister --no-wallet install --all
Creating UI canister on the local network.
The UI canister on the "local" network is "rrkah-fqaaa-aaaaa-aaaaq-cai"
Installing code for canister ic_rs, with canister_id rwlgt-iiaaa-aaaaa-aaaaa-cai

dfx canister --no-wallet call ic_rs greeting '("test", 6)'
("Hello test, you are 6 years old")

dfx canister --no-wallet call ic_rs sum '(1,2)'
Error deserializing blob 0x4449444c016c02007a017a01000300ffff
Invalid data: Invalid IDL blob.

dfx canister --no-wallet call ic_rs sum2 '(1,2)'
(3)

dfx canister --no-wallet call ic_rs sum3 '(1,2)'
(record { 3; 1 })

sudo dfx canister --no-wallet call ic_rs add '(1,2)'
(record { a = 1; b = 2 })
```



