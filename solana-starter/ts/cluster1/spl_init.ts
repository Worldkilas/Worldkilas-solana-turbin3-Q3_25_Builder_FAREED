import { Keypair, Connection, Commitment } from "@solana/web3.js";
import { createMint } from '@solana/spl-token';
import wallet from "/home/void/.config/solana/turbin3-wallet.json"

// Import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));
console.log(keypair.publicKey.toBase58());


//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

(async () => {
    try {
        // Start here
        const mint = await createMint(
            connection,
            keypair,
            keypair.publicKey,
            null,
            6,
);
        console.log(`Mint created: ${mint.toBase58()}`);

        // Optionally, you can also get the mint's info
        const mintInfo = await connection.getParsedAccountInfo(mint);
        console.log(`Mint info: ${JSON.stringify(mintInfo.value?.data, null, 2)}`);
    } catch(error) {
        console.log(`Oops, something went wrong: ${error}`)
    }
})()
