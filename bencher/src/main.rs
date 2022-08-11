use wasi_crypto_guest::symmetric::Hash;

fn main() {
    let res = Hash::hash("SHA-256", b"test", 32, None).unwrap();
    dbg!(res);
}
