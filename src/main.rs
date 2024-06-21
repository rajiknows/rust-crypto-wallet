mod eth_wallet;
mod utils;
fn main() {
    let (secretkey, publickey) = eth_wallet::generate_key_pairs();
    println!("secretkey is {}", &secretkey.to_string());
    println!("public key is {} ", &publickey.to_string());

    let address = eth_wallet::public_key_address(&publickey);
    println!("address is {}", &address);

    let wallet = eth_wallet::Wallet::new(&secretkey, &publickey);
    println!("{:?}", wallet);

    wallet.save_to_file("mywallet.json").unwrap();
    let wallet2 = eth_wallet::Wallet::export_from_file("mywallet2.json").unwrap();

    println!("{:?}", wallet2);
}
