import { publicKey } from './../node_modules/@solana/web3.js/src/layout';
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { HueMarketplaceCapstone } from "../target/types/hue_marketplace_capstone";
import { createMint, getAssociatedTokenAddressSync, getOrCreateAssociatedTokenAccount, mintTo, mintToChecked } from "@solana/spl-token";
import { TOKEN_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";
import { assert, expect } from "chai";
import { createUmi } from '@metaplex-foundation/umi-bundle-defaults';
import { MPL_CORE_PROGRAM_ID, mplCore } from '@metaplex-foundation/mpl-core';
import { SendTransactionError, SystemProgram, Transaction } from "@solana/web3.js";
import path from 'path';
import {
  createGenericFile,
  createSignerFromKeypair,
  signerIdentity,
  Umi,
} from "@metaplex-foundation/umi";

import { readFile, readFileSync } from 'fs';
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys";
import { it } from 'mocha';



describe("hue-marketplace-capstone", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.HueMarketplaceCapstone as Program<HueMarketplaceCapstone>;

  const connection= anchor.getProvider().connection;


  const programId= program.programId;

  const marketplaceAuthority=anchor.Wallet.local();

  const tokenDecimal=6

  const withdrawFeesBps=500;

  const commitFeesBps=1000;

   const dropName= "Messi";

   let tokenMint: anchor.web3.PublicKey;

   let umi: Umi;

   let treasuryAta: anchor.web3.PublicKey;


   const KEYPAIR_PATH= "/home/void/.config/solana/test-wallet.json";

   const secretKeyString=  readFileSync(KEYPAIR_PATH,{encoding: "utf-8"})
   
   const secretKey = Uint8Array.from(JSON.parse(secretKeyString));

   // using 3 supporters for testing one of which is my personal wallet
   const [creator, supporter1,supporter2, supporter3]= [anchor.web3.Keypair.fromSecretKey(secretKey), anchor.getProvider().wallet, anchor.web3.Keypair.generate(),anchor.web3.Keypair.generate()];


 
  let [marketplacePda]= anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("config"), marketplaceAuthority.publicKey.toBuffer()],
    programId
  );

  let [treasuryPda]=anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("treasury"), marketplacePda.toBuffer(),],
    programId
  );

  let  [dropCampaignPda]= anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("drop_campaign"),marketplacePda.toBuffer(),creator.publicKey.toBuffer(),Buffer.from(dropName, "utf-8")],
    programId
  )

  const [supporter1Pda, supporter2Pda, suporter3Pda]=[supporter1,supporter2, supporter3].map(
    (supporter)=> anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("supporter"),dropCampaignPda.toBuffer(), supporter.publicKey.toBuffer()],
      programId
    )[0]
   )
   console.log("here");
   




  let dropCampaignVault: anchor.web3.PublicKey;


  const log = async (signature: string): Promise<string> => {
    console.log(
      `Your transaction signature: https://explorer.solana.com/transaction/${signature}?cluster=custom&customUrl=${connection.rpcEndpoint}`
    );
    return signature;
  };


  before(async () => {
    
    
    tokenMint= await createMint(
      connection,
      marketplaceAuthority.payer,
      marketplaceAuthority.publicKey,
      marketplaceAuthority.publicKey,
      tokenDecimal,
    );

    umi=createUmi(anchor.getProvider().connection);
    let umiKeypair=umi.eddsa.createKeypairFromSecretKey(secretKey);
    const umiSigner=createSignerFromKeypair(umi, umiKeypair);

    umi.use(mplCore());
    umi.use(signerIdentity(umiSigner));
    umi.use(irysUploader())
     
    
  dropCampaignVault=getAssociatedTokenAddressSync(tokenMint,dropCampaignPda,true);
  treasuryAta= getAssociatedTokenAddressSync(tokenMint,treasuryPda, true);

  //helper for transfering sol instead of airdropping it
  //! Make sure there is enough SOL in the config wallet to transfer
  async function transferSolFromConfigWallet(to:anchor.web3.PublicKey,amountSol: number) {
    const tx= new anchor.web3.Transaction().add(
      SystemProgram.transfer({
        fromPubkey: anchor.getProvider().wallet.publicKey,
        toPubkey: to,
        lamports: amountSol*anchor.web3.LAMPORTS_PER_SOL
        
      })
    );
    await (anchor.getProvider()).sendAndConfirm(tx, [])
  }

  await transferSolFromConfigWallet(supporter1.publicKey, 0.05);
  await transferSolFromConfigWallet(supporter2.publicKey, 0.05)
  await transferSolFromConfigWallet(supporter3.publicKey, 0.05)
  await transferSolFromConfigWallet(creator.publicKey,3)


 const supporterAta1= await getOrCreateAssociatedTokenAccount(connection, supporter1.payer, tokenMint, supporter1.publicKey).then(account => account.address);
 const supporterAta2= await getOrCreateAssociatedTokenAccount(connection, marketplaceAuthority.payer, tokenMint, supporter2.publicKey).then(account => account.address)
 const supporterAta3=await getOrCreateAssociatedTokenAccount(connection, marketplaceAuthority.payer, tokenMint, supporter3.publicKey).then(account => account.address)



  await mintTo(connection,supporter1.payer,tokenMint,supporterAta1,marketplaceAuthority.publicKey, 1000e6);
  await mintTo(connection,marketplaceAuthority.payer,tokenMint,supporterAta2,marketplaceAuthority.publicKey, 1000e6);
  await mintTo(connection,marketplaceAuthority.payer,tokenMint,supporterAta3,marketplaceAuthority.publicKey, 1000e6);

  });

  describe("Mocking a succesful drop", ()=>{
    it("Is initialized!", async () => {

      let lamports= await connection.getMinimumBalanceForRentExemption(0);
    console.log(tokenMint.toBase58());
    
    console.log(treasuryPda.toBase58());
    
      
      // Add your test here.
      await program.methods.initializeMarketplace(commitFeesBps,withdrawFeesBps).accountsPartial({
        authority: marketplaceAuthority.publicKey,
        tokenMint: tokenMint,
        treasury: treasuryPda,
        treasuryTokenAccount: treasuryAta,
        marketplaceConfig: marketplacePda,
        tokenProgram: TOKEN_PROGRAM_ID,
      }).signers([marketplaceAuthority.payer]).rpc().then(log);
  
      const treasuryState= await connection.getBalance(treasuryPda);
  
      console.log(`Treasury PDA balance: ${treasuryState}`);
  

    });
  
   
    it("should fetch all marketplace accounts", async () => {
      try {
        let marketplace_accounts = await program.account.marketplaceConfig.all();
        console.log(marketplace_accounts);
      } catch (error) {
        console.log("failed to fetch  accounts from contract state");
        console.log(error);
      }
      const treasuryState= await connection.getBalance(treasuryPda);
  
      
    });
  
    it("should initialize campaign", async()=>{
     
  
      const filePath=path.join(__dirname, "merchs.jpeg");
      const fileContent=await readFileSync(filePath);
      const convertedFile= createGenericFile(fileContent,"merchs.jpeg",{
        contentType: "img/jpeg"
      });
  
      const [my_uri]= await umi.uploader.upload([convertedFile]);
  
      console.log(`Uploaded file URI: ${my_uri}`);
  
      const metadata = {
        name: "A merchandise",
        symbol: "MRCH",
        description: "A dope T-shirt",
        image: my_uri,
        properties: {
          files: [
            {
              type: "image/jpeg",
              uri: my_uri,
            },
          ],
        },
        creators: [creator.publicKey.toBase58()],
      };
      const metadataUri = await umi.uploader.uploadJson(metadata);
      
      console.log(`metadataUri: ${metadataUri}`);
      
      const goalOrders = 15;
      const allowedUnitsPerSupporter = 5;
      const price = new anchor.BN(10e6); // 100 tokens, assuming token has 6 decimals
    
      const daysUntilEnd = 1;
      const collectionMint= anchor.web3.Keypair.generate();
  
      const nowInSeconds = Math.floor(Date.now() / 1000);
      
       try {
        await program.methods.initializeCampaign({
          name: dropName,
          goalOrders,
          price,
          startTimestamp: new anchor.BN(nowInSeconds),
          daysUntilEnd: new anchor.BN(daysUntilEnd),
          uri: metadataUri,
          pledgedOrders: 0,
          allowedUnitsPerSupporter
        }).accountsPartial({
          creator: creator.publicKey,
          tokenMint,
          collectionMint: collectionMint.publicKey,
          marketplaceConfig: marketplacePda,
          dropCampaign: dropCampaignPda,
          tokenProgram: TOKEN_PROGRAM_ID,
          campaignVault: dropCampaignVault,
          mplCoreProgram: MPL_CORE_PROGRAM_ID,
        }).signers([ collectionMint, creator]).rpc()
       } catch (error) {
        console.log(error);
        if (error.logs) {
          console.log(error.logs);
          throw Error("error occured");
        }
        throw Error(error);}
  
        const campaignAccounts= await program.account.dropCampaign.fetch(dropCampaignPda);
      
        expect(campaignAccounts.name).to.be.equal(dropName);
        expect(campaignAccounts.creator.toBase58()).to.be.equal(creator.publicKey.toBase58());
        expect(campaignAccounts.goalOrders).to.be.equal(goalOrders);
        expect(campaignAccounts.price.toNumber()).to.be.equal(price.toNumber());
        expect(campaignAccounts.endTimestamp.toNumber()).to.be.equal(campaignAccounts.startTimestamp.add(new anchor.BN(daysUntilEnd*24*60*60)).toNumber());
    })
  
    it("Supporter 1 commits funds the campaign", async()=>{
      const unitsToOrder=4;
      const dropCampaignAccount= await program.account.dropCampaign.fetch(dropCampaignPda);
  
      console.log(`Drop campaign account: ${JSON.stringify(dropCampaignAccount)}`);
      if (unitsToOrder<=dropCampaignAccount.allowedUnitsPerSupporter) {
        await program.methods.preorder(unitsToOrder).accountsPartial({
          supporter: supporter1.publicKey,
          tokenMint,
          marketplaceConfig: marketplacePda,
          dropCampaign: dropCampaignPda,
          supporterAccount: supporter1Pda,
          campaignVault: dropCampaignVault,
          treasury: treasuryPda,
          
          tokenProgram: TOKEN_PROGRAM_ID,
        }).signers([supporter1.payer]).rpc().then(log)
      }else{
        console.log(" Orders exceeded");
        return;
        
      }
      const updatedcampaignAccount= await program.account.dropCampaign.fetch(dropCampaignPda);
  
      expect(dropCampaignAccount.pledgedOrders).to.be.lessThan(unitsToOrder);
      expect(updatedcampaignAccount.pledgedOrders).to.be.greaterThan(dropCampaignAccount.pledgedOrders);
      expect(updatedcampaignAccount.supporterCount.toNumber()).to.be.equal(1);
  
      console.log(`Updated campaign account: ${JSON.stringify(updatedcampaignAccount)}`);
      
  
    })
  
    it("should allow supporter1 adds one more order ", async() => {
      const treasuryBalance= Number((await connection.getTokenAccountBalance(treasuryAta)).value.amount);
     
        const initializeCampaignaAccount= await program.account.dropCampaign.fetch(dropCampaignPda);
        await  program.methods.preorder(1).accountsPartial({
          supporter: supporter1.publicKey,
          tokenMint,
          marketplaceConfig: marketplacePda,
          dropCampaign: dropCampaignPda,
          supporterAccount: supporter1Pda,
          campaignVault: dropCampaignVault,
          treasury: treasuryPda,
          treasuryAta,
          tokenProgram: TOKEN_PROGRAM_ID,
        }).signers([supporter1.payer]).rpc().then(log);

        const updatedcampaignAccount= await program.account.dropCampaign.fetch(dropCampaignPda);

        const updatedTreasuryBalance=  Number((await connection.getTokenAccountBalance(treasuryAta)).value.amount)
  
        expect(updatedcampaignAccount.pledgedOrders).to.be.equal(initializeCampaignaAccount.pledgedOrders+1);
        expect(updatedcampaignAccount.supporterCount.toNumber()).to.be.equal(1);
        expect(updatedTreasuryBalance).to.be.greaterThan(treasuryBalance);
        
     
    })
  
    it("should fail when supporter 1 tries to add any more than specified by the campaign pda", async()=>{
      try {
        
        await program.methods.preorder(3).accountsPartial({
          supporter: supporter1.publicKey,
          tokenMint,
          marketplaceConfig: marketplacePda,
          dropCampaign: dropCampaignPda,
          supporterAccount: supporter1Pda,
          campaignVault: dropCampaignVault,
          treasury: treasuryPda,    
          tokenProgram: TOKEN_PROGRAM_ID,
        }).signers([supporter1.payer]).rpc().then(log)
        assert.fail("Expected preorder to throw, but it succeeded");
      } catch (err) {
        // Assert that the error is the one you expect
      const errMsg = (err as any).error.errorMessage;
      expect(errMsg).to.contain("Unit ordered exceeds allowed units per supporter or is zero")
      console.error(errMsg);
      
      }
    })
    it("should fail if suporter 2 tries to order 0 units", async()=>{
      try {
        const orderUnits=0
        await program.methods.preorder(orderUnits).accountsPartial({
          supporter: supporter2.publicKey,
          tokenMint,
          marketplaceConfig: marketplacePda,
          dropCampaign: dropCampaignPda,
          supporterAccount: supporter2Pda,
          campaignVault: dropCampaignVault,
          treasury: treasuryPda,    
          tokenProgram: TOKEN_PROGRAM_ID,
        }).signers([supporter2]).rpc().then(log)
        assert.fail("Expected preorder to throw, but it succeeded");
      } catch (err) {
      // Anchor errors often show up in logs:
  if (err.logs) {
    console.log("Error logs:", err.logs);
  }

  // Newer Anchor has a helper to parse custom program errors:
  if (err.error) {
    console.log("Anchor error:", err.error);
    console.log("Anchor error message:", err.error.errorMessage);
  }

  // Safer assertion:
  const logs = err.logs?.join(" ") || "";
  expect(logs).to.contain("Unit ordered exceeds allowed units per supporter or is zero");
      
      }
    })

    it("should fail if suporter 2 tries to order more than 5 units", async()=>{
      try {
        const orderUnits=10
        await program.methods.preorder(orderUnits).accountsPartial({
          supporter: supporter2.publicKey,
          tokenMint,
          marketplaceConfig: marketplacePda,
          dropCampaign: dropCampaignPda,
          supporterAccount: supporter2Pda,
          campaignVault: dropCampaignVault,
          treasury: treasuryPda,    
          tokenProgram: TOKEN_PROGRAM_ID,
        }).signers([supporter2]).rpc().then(log)
        assert.fail("Expected preorder to throw, but it succeeded");
      } catch (err) {
       // Anchor errors often show up in logs:
 
    // Safer assertion:
    const logs = err.logs?.join(" ") || "";
    expect(logs).to.contain("Unit ordered exceeds allowed units per supporter or is zero");;
      
      }
    })

    it("should allow supporter2 to commit since unitsToOrder<=5", async()=>{
      const unitsToOrder=5;
      const dropCampaignAccount= await program.account.dropCampaign.fetch(dropCampaignPda);
  
      console.log(`Drop campaign account: ${JSON.stringify(dropCampaignAccount)}`);
      if (unitsToOrder<=dropCampaignAccount.allowedUnitsPerSupporter) {
        await program.methods.preorder(unitsToOrder).accountsPartial({
          supporter: supporter2.publicKey,
          tokenMint,
          marketplaceConfig: marketplacePda,
          dropCampaign: dropCampaignPda,
          supporterAccount: supporter2Pda,
          campaignVault: dropCampaignVault,
          treasury: treasuryPda,
          
          tokenProgram: TOKEN_PROGRAM_ID,
        }).signers([supporter2]).rpc().then(log)
      }else{
        console.log(" Orders exceeded");
        return;
        
      }
      const updatedcampaignAccount= await program.account.dropCampaign.fetch(dropCampaignPda);
  
      expect(updatedcampaignAccount.pledgedOrders).to.be.greaterThan(dropCampaignAccount.pledgedOrders);
      expect(updatedcampaignAccount.supporterCount.toNumber()).to.be.greaterThan(dropCampaignAccount.supporterCount.toNumber());
      expect(updatedcampaignAccount.supporterCount.toNumber()).to.be.equal(2);
  
      console.log(`Updated campaign account: ${JSON.stringify(updatedcampaignAccount)}`);
      
  
    })

    it("should fail if the creator attempts to withdraw from the vault since the required number of orders is not fulfilled", async()=>{
      
       try {
        
        await program.methods.withdraw().accountsPartial({
          creator: creator.publicKey,
          dropCampaign: dropCampaignPda,
          treasury: treasuryPda,
          treasuryAta,
          campaignVault: dropCampaignVault,
          tokenMint,   
          marketplaceConfig: marketplacePda,
          tokenProgram: TOKEN_PROGRAM_ID,
        }).signers([creator]).rpc().then(log)


        assert.fail("Expected the campaign to be finalized and successful");
       } catch (err) {
        // Assert that the error is the one you expect
      const errMsg = (err as any).error.errorMessage;
      expect(errMsg).to.contain("Campaign not successful")
      console.error(errMsg);
       }
      
    })
    it("should allow supporter 3 to commit funds", async()=>{
      const unitsToOrder=5;
      const dropCampaignAccount= await program.account.dropCampaign.fetch(dropCampaignPda);
  
      console.log(`Drop campaign account: ${JSON.stringify(dropCampaignAccount)}`);
      if (unitsToOrder<=dropCampaignAccount.allowedUnitsPerSupporter) {
        await program.methods.preorder(unitsToOrder).accountsPartial({
          supporter: supporter3.publicKey,
          tokenMint,
          marketplaceConfig: marketplacePda,
          dropCampaign: dropCampaignPda,
          supporterAccount: suporter3Pda,
          campaignVault: dropCampaignVault,
          treasury: treasuryPda,
          
          tokenProgram: TOKEN_PROGRAM_ID,
        }).signers([supporter3]).rpc().then(log)
      }else{
        console.log(" Orders exceeded");
        return;
        
      }
      const updatedcampaignAccount= await program.account.dropCampaign.fetch(dropCampaignPda);
  
      expect(updatedcampaignAccount.pledgedOrders).to.be.greaterThan(dropCampaignAccount.pledgedOrders);
      expect(updatedcampaignAccount.supporterCount.toNumber()).to.be.greaterThan(dropCampaignAccount.supporterCount.toNumber());
      expect(updatedcampaignAccount.supporterCount.toNumber()).to.be.equal(3);
  
      console.log(`Updated campaign account: ${JSON.stringify(updatedcampaignAccount)}`);
      
  
    })

    it("refund should for supporter 1 fail since campaign is successful", async()=>{
      try {
        await program.methods.claimRefund().accountsPartial({
          supporter: supporter1.publicKey,
          dropCampaign: dropCampaignPda,
          supporterAccount: supporter1Pda,
          campaignVault: dropCampaignVault,
          tokenMint,
          marketplaceConfig: marketplacePda,
          tokenProgram: TOKEN_PROGRAM_ID,
        }).signers([supporter1.payer]).rpc().then(log)
        // assert.fail("Expected the campaign to be finalized and not successful");
      } catch (err) {
        const errMsg = (err as any).error.errorMessage;
        expect(errMsg).to.contain("Cannot refund from an already successful campaign")
      console.error(errMsg)
      }
    })

    it("should withdraw from campaign vault to creator when target met within time",async()=>{
      const dropCampaignAccount= await program.account.dropCampaign.fetch(dropCampaignPda);
      const treasuryBalance=  Number((await connection.getTokenAccountBalance(treasuryAta)).value.amount)
      const initialCreatorTokenAccoutBalanceForTest=0;
      await program.methods.withdraw().accountsPartial({
        creator: creator.publicKey,
        marketplaceConfig: marketplacePda,
        dropCampaign: dropCampaignPda,
        campaignVault: dropCampaignVault,
        treasury: treasuryPda,
        treasuryAta,
        tokenMint,
        creatorTokenAccount: getAssociatedTokenAddressSync(tokenMint, creator.publicKey),
        tokenProgram: TOKEN_PROGRAM_ID,
      }).signers([creator]).rpc().then(log)
      const updatedCampaignAccount= await program.account.dropCampaign.fetch(dropCampaignPda);
      const updatedTreasuryBalance=  Number((await connection.getTokenAccountBalance(treasuryAta,)).value.amount)
      const updatedCreatorTokenAccount= Number((await connection.getTokenAccountBalance(getAssociatedTokenAddressSync(tokenMint, creator.publicKey))).value.amount);

      expect(updatedCampaignAccount.isFinalized).to.be.equal(true);
      expect(updatedCampaignAccount.isSuccessful).to.be.equal(true);
      expect(updatedCampaignAccount.pledgedOrders).to.be.equal(dropCampaignAccount.goalOrders);
      expect(updatedTreasuryBalance).to.be.greaterThan(treasuryBalance);
      expect(updatedCreatorTokenAccount).to.be.greaterThan(initialCreatorTokenAccoutBalanceForTest);

      console.log(`Creator account balance: ${updatedCreatorTokenAccount}`);   

    })

   
      
       
      
  

  })



  
})

