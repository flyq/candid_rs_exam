# IC_RS

## build
```sh
cargo run
   Compiling ic_rs v0.1.0 (/home/peter/ic_rs)
    Finished dev [unoptimized + debuginfo] target(s) in 1.69s
     Running `target/debug/ic_rs`
service : {
  greeting : (text, nat16) -> (text) query;
  sum : (nat16, nat16, nat16, nat16) -> (nat16, nat16) query;

cargo run > ic_rs.did
    Finished dev [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/ic_rs`

cargo build --release --target wasm32-unknown-unknown
   Compiling ic_rs v0.1.0 (/home/peter/ic_rs)
    Finished release [optimized] target(s) in 0.71s
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



