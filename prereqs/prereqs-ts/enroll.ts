import { Connection, Keypair, PublicKey } from "@solana/web3.js"
import { Program, Wallet, AnchorProvider, Idl } from "@coral-xyz/anchor"
import { IDL, Turbin3Prereq } from "./programs/Turbin3_prereq";
import wallet from "./turbin3-wallet.json"
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";


const MPL_CORE_PROGRAM_ID= new PublicKey("CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d");



const keypair= Keypair.fromSecretKey(new Uint8Array(wallet));
const connection = new Connection("https://api.devnet.solana.com");

//create anchor provider
const provider=new AnchorProvider(connection,new Wallet(keypair),{commitment:'confirmed'});
console.log(`${provider.publicKey}`)

const program: Program<Idl> = new Program(IDL,  provider);
console.log(`${program.programId}`);

// Create the PDA for our enrollment account
const account_seeds = [
    Buffer.from("prereqs"),
    keypair.publicKey.toBuffer(),
];

const [account_key, account_bump]= PublicKey.findProgramAddressSync(account_seeds,program.programId);
const mintCollection=new PublicKey("5ebsp5RChCGK7ssRZMVMufgVZhd2kFbNaotcZ5UvytN2");
// / Derive the authority PDA
const authority_seeds = [
  Buffer.from("collection"),
  mintCollection.toBuffer(),
];
const [authority_key, authority_bump] = PublicKey.findProgramAddressSync(authority_seeds, program.programId);
const mintTs=  Keypair.generate();
//execute initialize
// (async ()=>{
//     try {
//         const txhash= await program.methods.initialize("Worldkilas").accountsPartial({
//             user: keypair.publicKey,
//             account: account_key,
//             system_program: SYSTEM_PROGRAM_ID
//         }).signers([keypair]).rpc();
//         console.log(`Success! Check out your TX here: https://explorer.solana.com/tx/${txhash}?cluster=devnet`);
//     } catch (error) {
//         console.error(`Oops, something went wrong: ${error}`);
//     }
// })();

//execute submit ts
(async ()=>{
    try {
        const txhash= await program.methods.submitTs().accountsPartial({
            user: keypair.publicKey,
            account: account_key,
            authority: authority_key,
            mint: mintTs.publicKey,
            collection: mintCollection,
            mpl_core_program: MPL_CORE_PROGRAM_ID,
            system_program: SYSTEM_PROGRAM_ID,
          
            
        }).signers([keypair, mintTs]).rpc();
        console.log(`Success! Check out your TX here: https://explorer.solana.com/tx/${txhash}?cluster=devnet`);
    } catch (error) {
        console.error(`Oops, something went wrong: ${error}`);
    }
})();