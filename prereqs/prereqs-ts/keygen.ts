import {Connection,Keypair, LAMPORTS_PER_SOL    } from '@solana/web3.js';
import wallet from "./dev-wallet.json";
import { log } from 'node:console';

let keypair= Keypair.fromSecretKey(new Uint8Array(wallet));

console.log(`Publickey: ${keypair.publicKey.toBase58()}`);

//establish connection to sol devnet
const connection = new Connection("https://api.devnet.solana.com");

(async()=>{
    try {
        //request 2 airdrops SOL
        const txhash= await connection.requestAirdrop(keypair.publicKey, 2*LAMPORTS_PER_SOL);
        console.log(`Success! Check out your TX here:https://explorer.solana.com/tx/${txhash}?cluster=devnet`);
        
    } catch (error) {
        console.error(`Oops, something went wrong: ${error}`);
        
    }
})();


