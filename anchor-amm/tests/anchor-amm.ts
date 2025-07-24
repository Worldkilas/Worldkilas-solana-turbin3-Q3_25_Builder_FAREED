import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorAmm } from "../target/types/anchor_amm";
import { 
  createMint,
  getOrCreateAssociatedTokenAccount,
  mintTo,
  getAssociatedTokenAddress
 } from "@solana/spl-token";
import { expect } from "chai";
import fs from 'fs';
import { it } from "mocha";




describe("anchor-amm", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.anchorAmm as Program<AnchorAmm>;
  const connection=anchor.getProvider().connection;
  const admin= anchor.getProvider().wallet||anchor.AnchorProvider.local().wallet;

  let mintX: anchor.web3.PublicKey;
  let mintY: anchor.web3.PublicKey;
  let mintLp: anchor.web3.PublicKey;

  let configPda: anchor.web3.PublicKey;
  let vaultX: anchor.web3.PublicKey;
  let vaultY: anchor.web3.PublicKey;
  
  // const userAccount= new Uint8Array(fs.readFileSync('/home/void/.config/solana/test-wallet.json'))
  let userAtaX:anchor.web3.PublicKey;
  let userAtaY: anchor.web3.PublicKey;
  let userAtaLp: anchor.web3.PublicKey;

  const seed= new anchor.BN(50);
  const fee=30;
  before(async() =>{
    mintX= await createMint(connection,admin.payer,admin.publicKey,null,6);
    mintY= await createMint(connection,admin.payer,admin.publicKey,null,6);

    [configPda]= anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("config"),seed.toArrayLike(Buffer,"le",8)],
      program.programId,
    );

    [mintLp]= anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("lp"), configPda.toBuffer()],
      program.programId
    );

    vaultX= await getAssociatedTokenAddress(mintX,configPda,true);
    vaultY=await getAssociatedTokenAddress(mintY,configPda,true )

  });
  it("Initialize the AMM pool", async () => {
     const tx= await program.methods.initialize({
        authority:null,
        seed,
        fee
      }).accountsPartial({
        admin: admin.publicKey,
        tokenXMint: mintX,
        tokenYMint: mintY, 
        lpTokenMint: mintLp,
        poolTokenXVault: vaultX,
        poolTokenYVault: vaultY,
        config: configPda
        
      }).rpc()
      console.log(`https://explorer.solana.com/tx/${tx}?cluster=devnet`);
      
      const config= await program.account.config.fetch(configPda);

      expect(config).to.exist;
      expect(config.seed.toNumber()).to.equal(seed.toNumber());
      expect(config.tokenXMint.toBase58()).to.equal(mintX.toBase58());
      expect(config.tokenYMint.toBase58()).to.equal(mintY.toBase58());
      expect(config.fee).to.equal(fee);
      expect(config.locked).to.equal(false);

      const ataX= await getOrCreateAssociatedTokenAccount(connection,admin.payer,mintX,admin.publicKey)
      const ataY= await getOrCreateAssociatedTokenAccount(connection,admin.payer,mintY,admin.publicKey);
      

      await mintTo(connection,admin.payer,mintX, ataX.address,admin.payer,1_000_000);
      await mintTo(connection,admin.payer,mintY,ataY.address,admin.payer,1_000_000);

      userAtaX=ataX.address; 
      userAtaY=ataY.address;
     
  
    
  });

  it("Deposit liquidity into the pool ", async () => {

    const lpToBeMinted= new anchor.BN(1_000_000)
    const maxX= new anchor.BN(500_000)
    const maxY= new anchor.BN(500_000);
    userAtaLp= (await getOrCreateAssociatedTokenAccount(connection,admin.payer, mintLp, admin.publicKey)).address;

    const tx= await program.methods.deposit(lpToBeMinted, maxX,maxY).accountsPartial({
      depositor: admin.publicKey,
      depositorTokenXAccount: userAtaX,
      depositorTokenYAccount: userAtaY,
      depositorLpTokenAccount: userAtaLp,
      config: configPda,
      poolTokenXVault: vaultX,
      poolTokenYVault: vaultY,
      lpTokenMint: mintLp,
      tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID
    }) .rpc()
    console.log(`https://explorer.solana.com/tx/${tx}?cluster=devnet`);
  
    console.log("Depositted liquidity");
  });

 
  
  
});
