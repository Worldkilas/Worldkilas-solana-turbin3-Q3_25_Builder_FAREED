import wallet from "/home/void/.config/solana/turbin3-wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createGenericFile, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi"

import { readFile } from "fs/promises"
import { readFileSync } from "fs"
import { create } from "domain"
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys"

// Create a devnet connection
const umi = createUmi('https://devnet.helius-rpc.com/?api-key=aaa57db2-bd9b-435c-9e78-86fc74dc7dcb');

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader({address: 'https://devnet.irys.xyz/'}));
umi.use(signerIdentity(signer));

(async () => {
    try {
        //1. Load image
        let imageBuffer= readFileSync('/home/void/Downloads/lockedin.jpeg')

        console.log("Ronaldo");
        
        //2. Convert image to generic file.
        const image= createGenericFile(imageBuffer, 'lockedin.jpeg', {
            contentType: 'image/jpeg'
        });
        //3. Upload image
        const [imageUri]= await umi.uploader.upload([image]);

        
        

        

       
        console.log("Your image URI: ", imageUri);
    }
    catch(error) {
        console.log("Oops.. Something went wrong", error);
    }
})();
