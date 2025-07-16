import { Transaction, SystemProgram, Connection, Keypair, LAMPORTS_PER_SOL, sendAndConfirmTransaction, PublicKey } from "@solana/web3.js"

import wallet from "./dev-wallet.json";

const from= Keypair.fromSecretKey(new Uint8Array(wallet));
// Turbin3 public key
const to= new PublicKey("3rLojDoVPUKBXnr2vybXJA3Lh3wKBxxDHx7fxJn8m3ZM");

const connection= new Connection("https://api.devnet.solana.com");

(async () => {
    try {
        //get balance of dev-wallet
        const balance=await connection.getBalance(from.publicKey);
        //create tx to calculate fees
        const transaction= new Transaction().add(
            SystemProgram.transfer({
                fromPubkey: from.publicKey,
                toPubkey: to,
                lamports: balance
            })
        );
        transaction.recentBlockhash = (await connection.getLatestBlockhash('confirmed')).blockhash;
        transaction.feePayer = from.publicKey;
        // Calculate exact tx fees so that all sol can be transferred minus tx fees
        const fee= (await connection.getFeeForMessage(transaction.compileMessage(),'confirmed')).value||0;
        transaction.instructions.pop(); 
        // Now add the instruction back with correct amount of lamports
        transaction.add(
            SystemProgram.transfer({
                fromPubkey: from.publicKey,
                toPubkey: to,
                lamports: balance - fee
            })
        );
        // Sign the transaction, broadcast and confirm
        const signature= await sendAndConfirmTransaction(connection, transaction, [from]);
        console.log(`Success! Check out your TX here: https://explorer.solana.com/tx/${signature}?cluster=devnet`);
    } catch (error) {
        console.error(`Oops, something went wrong: ${error}`);
    }
})();