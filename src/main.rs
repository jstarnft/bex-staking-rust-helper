pub mod signature;
use signature::SigningUtils;

use dotenv::dotenv;
use std::env;

use ethers::signers::LocalWallet;

async fn async_main() {
    // Load wallet from .env
    dotenv().ok();
    let wallet;
    if let Ok(private_key) = env::var("PRIVATE_KEY_ADMIN") {
        wallet = private_key.parse::<LocalWallet>().unwrap();
        println!("{:?}", wallet);
    } else {
        panic!("Variable `PRIVATE_KEY_ADMIN` not in `.env`!");
    }

    // Define the test variables
    let name = "hi";
    let amount = 7000_000000;
    let user = "0xb54e978a34Af50228a3564662dB6005E9fB04f5a";

    // Sign renew-ownership requests
    println!("\n========== Test renew-ownership request ==========");
    let (time, message, sig) = SigningUtils::sign_renew_ownership(name, amount, user, wallet).await;

    println!("Time: {}", time);
    println!("Message: {}", message);
    println!("Signature: {}", sig);
}

fn main() {
    // The signing functions are async while the main function must be sync.
    // So we have to wrap the signing test functions in a tokio runtime.

    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(async_main());
    // rt.block_on(async {
    //     match async_main().await {
    //         Ok(_) => println!("Async operation completed successfully"),
    //         Err(err) => eprintln!("Error: {}", err),
    //     }
    // });
}
