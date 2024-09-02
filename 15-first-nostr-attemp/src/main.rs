// https://crates.io/crates/nostr-sdk

use nostr_sdk::prelude::*;

fn main() { 

    // Create keys
    // let my_keys: Keys = Keys::generate();
    // let hex_pubkey: String = my_keys.public_key().to_hex();
    // let nsec = my_keys.secret_key();
    // match nsec{
    //     Ok(nsec) => println!("Secret key: {:?}", nsec.to_bech32()),
    //     Err(error) => eprintln!("Error: {:?}", error),
    // }
    // println!("Hex Pub: {}" , hex_pubkey);

    // -----------

    // or use these already generated ones for testing purposes
    // nsec: nsec1ddvx2kul4frw0l4nsl0np3r7nc6m2kpgrqzxz7lr84u4mucgnntqds9xqx
    // nPub: npub1hnguns63ppjyx997f86gnqhmw88jdhtejyxtcstc6ly6celj8rgq9w95et
    // Hex: bcd1c9c35108644314be49f48982fb71cf26dd79910cbc4178d7c9ac67f238d0

    let my_keys = match Keys::parse("nsec1ddvx2kul4frw0l4nsl0np3r7nc6m2kpgrqzxz7lr84u4mucgnntqds9xqx") {
        Ok(keys) => keys,
        Err(error) => {
            eprintln!("Error parsing keys: {:?}", error);
            return; // Exit program on error
        }
    };

    let bech32_pubkey = match my_keys.public_key().to_bech32() {
        Ok(pubkey) => pubkey,
        Err(error) => {
            eprintln!("Error converting to Bech32: {:?}", error);
            return; // Exit program on error
        }
    };

    println!("Bech32 PubKey: {}", bech32_pubkey);

    let client = Client::new(&my_keys);
    // client.add_relay("wss://relay.damus.io").await.unwrap();

}
