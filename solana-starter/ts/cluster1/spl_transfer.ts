import { Commitment, Connection, Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js"
import wallet from "/home/void/.config/solana/turbin3-wallet.json"
import { getOrCreateAssociatedTokenAccount, transfer } from "@solana/spl-token";

// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

// Mint address
const mint = new PublicKey("5t9uaCxiHdCYHWUn7tTtzwQmWqVgfYV81druNLm8oZcP");

// Recipient address
const to = new PublicKey("UjrPReZo5dq19a2LwGfAY5MJc8V2F8gnHSgoDgXP6Y3");

(async () => {
    try {
        // Get the token account of the fromWallet address, and if it does not exist, create it
        const fromTokenAcct=await getOrCreateAssociatedTokenAccount(
            connection,
            keypair,
            mint,
            keypair.publicKey
        )

        // Get the token account of the toWallet address, and if it does not exist, create it
        const toTokenAccount= await getOrCreateAssociatedTokenAccount(
            connection,
            keypair,
            mint,
            to
        )

        // Transfer the new token to the "toTokenAccount" we just created
        const sig=await transfer(
            connection,
            keypair,
            fromTokenAcct.address,
            toTokenAccount.address,
            keypair.publicKey,
            1000
        )
        console.log(`Check your transfer at: https://explorer.solana.com/tx/${sig}?cluster=devnet`);
        
    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();