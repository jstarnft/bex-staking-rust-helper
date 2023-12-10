pub mod signature;

use std::env;
use dotenv::dotenv;

use ethers::signers::LocalWallet;

async fn async_main() -> Result<(), Box<dyn std::error::Error>> {
    // Load wallet from .env
    dotenv().ok();
    let wallet;
    if let Ok(private_key) = env::var("PRIVATE_KEY_ADMIN") {
        wallet = private_key.parse::<LocalWallet>()?;
        println!("{:?}", wallet);
    } else {
        panic!("Variable `PRIVATE_KEY_ADMIN` not in `.env`!");
    }

    // Sign base request
    let (time, message, sig) = signature::sign_base_requset("cb62320d", "hi", 70000000000000000, "0xb54e978a34Af50228a3564662dB6005E9fB04f5a", signature::now_timestamp(), wallet).await;

    println!("time: {}", time);
    println!("message: {}", message);
    println!("sig: {}", sig);

    Ok(())
}


fn main() {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(async {
        match async_main().await {
            Ok(_) => println!("Async operation completed successfully"),
            Err(err) => eprintln!("Error: {}", err),
        }
    });
}
