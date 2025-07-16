import wallet from "/home/void/.config/solana/turbin3-wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createGenericFile, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi"
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys"

// Create a devnet connection
const umi = createUmi('https://api.devnet.solana.com');

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));

(async () => {
    try {
        // Follow this JSON structure
        // https://docs.metaplex.com/programs/token-metadata/changelog/v1.0#json-structure

        const image = "https://devnet.irys.xyz/EztHsF8DZTWEGg3xLknapb97NVDrdRNNW7oGireYgvQo"
        const metadata = {
            name: "LOCKED IN",
            symbol: "🔒LCK",
            description: "Stay locked in folks. Also pray for Muzan while you are at it",
            image: image,
            attributes: [
                {trait_type: 'Rarity', value: 'Common'},
                { trait_type: 'Pattern', value: 'Geometric' },
                { trait_type: 'Style', value: 'Modern' },
                { trait_type: 'Collection', value: 'Genesis' }
            ],
            properties: {
                files: [
                    {
                        type: "image/png",
                        uri: "https://devnet.irys.xyz/EztHsF8DZTWEGg3xLknapb97NVDrdRNNW7oGireYgvQo"
                    },
                ]
            },
            creators: [
                {
                    address: signer.publicKey,
                    share: 100,
                }
            ]
        };
        const myUri = await umi.uploader.uploadJson(metadata)
        console.log("Your metadata URI: ", myUri);
    }
    catch(error) {
        console.log("Oops.. Something went wrong", error);
    }
})();
