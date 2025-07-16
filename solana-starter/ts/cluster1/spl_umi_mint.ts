import {
    createFungible,mplTokenMetadata
  } from '@metaplex-foundation/mpl-token-metadata'
  import {
    createTokenIfMissing,
    findAssociatedTokenPda,
    getSplAssociatedTokenProgramId,
    mintTokensTo,
    mplToolbox,
  } from '@metaplex-foundation/mpl-toolbox'
  import {
    generateSigner,
    percentAmount,
    createGenericFile,
    signerIdentity,
    sol,
    createSignerFromKeypair,
    keypairIdentity,
  } from '@metaplex-foundation/umi';

import wallet from '/home/void/.config/solana/turbin3-wallet.json';

import { createUmi } from '@metaplex-foundation/umi-bundle-defaults'
import { irysUploader } from '@metaplex-foundation/umi-uploader-irys'
import { base58 } from '@metaplex-foundation/umi/serializers'
import fs from 'fs';

const RPC_ENDPOINT = 'https://api.devnet.solana.com';
const umi = createUmi(RPC_ENDPOINT).use(irysUploader()).use(mplTokenMetadata()).use(mplToolbox());

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
// console.log("Keypair Public Key: ", keypair.publicKey.toString());
umi.use(keypairIdentity(keypair));
// generate new keypair signer for mint from umi
const mint = generateSigner(umi);

   

(async () => {
    const imageFileBuffer= fs.readFileSync('/home/void/Downloads/lockedin.jpeg')
    console.log('Image loaded, size:', imageFileBuffer.length, 'bytes');
    const umiImageFile = createGenericFile(imageFileBuffer,'lockedin.jpeg',{
        tags: [{name: 'Content-Type', value: 'image/jpeg'}],
    })

    const imageUri= await umi.uploader.upload([umiImageFile]).catch((err)=> {
        console.error(`Error uploading file:${err}`)
        throw new Error();
    })

    const token_decimals = 1_000_000_000n;
    
    const tokenMetadata={
        name: 'Locked In',
        symbol: '🔒LCK',
        description: 'Stay locked in folks. Also pray for Muzan while you are at it',
        imageUrl: imageUri,
    }
    const metadataUri = await umi.uploader.uploadJson(tokenMetadata).catch((err) => {
        console.error(`Error uploading metadata: ${err}`);
        throw new Error();
    })

    console.log(metadataUri);

    const createMintIx= await createFungible(
        umi,
        {
            mint: mint,
            name: 'Locked In',
            uri: metadataUri,
            sellerFeeBasisPoints: percentAmount(4),
            decimals: 9 

        }
    )
    const createTokenIx= createTokenIfMissing(
        umi,
        {
            mint: mint.publicKey,
            owner: umi.identity.publicKey,
            ataProgram: getSplAssociatedTokenProgramId(umi),
        }
    )

    const mintTokensIx = mintTokensTo(umi, {
        mint: mint.publicKey,
        token: findAssociatedTokenPda(umi, {
          mint: mint.publicKey,
          owner: umi.identity.publicKey,
        }),
        amount: BigInt(1000),
      })
      // chain the instructions together with .add() then send with .sendAndConfirm()

const tx = await createMintIx
.add(createTokenIx)
.add(mintTokensIx)
.sendAndConfirm(umi)

// finally we can deserialize the signature that we can check on chain.
// import { base58 } from "@metaplex-foundation/umi/serializers";

console.log(base58.deserialize(tx.signature)[0])
    
   
})()


