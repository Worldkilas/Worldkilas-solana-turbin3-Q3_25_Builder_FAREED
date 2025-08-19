# Hue Marketplace (Capstone Project)

Hue Marketplace is a **Solana-based preorder + merch platform** where creators can launch limited drop campaigns, supporters can preorder/back items, and **soulbound tokens (SBTs)** are issued as **proof of support**.

The project was built as part of the **Turbin3 Builders Cohort**.

---

## 🔗 Research & References

- [Market Research & Competitor Analysis](https://docs.google.com/document/d/1WHBGJspg_s0pfCZjSIUH51QPB7tXtQKVmNlzBy7pXZw/edit?tab=t.0)  
- [User Stories & On-Chain Mapping](https://docs.google.com/document/d/1EUEbBaeQkNF0ny-WKEfX6cGznjVhc6kbAABJjNN1DDc/edit?tab=t.0)  
- [Architecture Design](https://docs.google.com/document/d/1Q-VYphQnh3SC2zGk50UMczptmdC_ZAtqbEnUkTWmk_Q/edit?tab=t.0#heading=h.o94rdc71fwiu)  

---

## 🏗️ Architecture

![Architecture Diagram](./Architectural%20diagram.png)

- **Marketplace Config PDA** → stores global settings (fees, authority, treasury bump).  
- **Drop Campaign PDA** → represents a merch drop (goal, price, duration, state).  
- **Supporter PDA** → tracks each supporter’s contributions, refund status, and SBT minting.  
- **Treasury PDA** → stores collected fees.  
- **Campaign Vault (ATA)** → holds funds committed by supporters until campaign finalization.  

---

## 📦 Program Instructions

### 1. `InitializeMarketplace`
- Creates marketplace config.  
- Initializes treasury account + treasury ATA.  
- Stores fees in basis points (`commit_fees_bps`, `withdraw_fees_bps`).  

### 2. `InitializeCampaign`
- Creator launches a merch drop.  
- Allocates a **campaign vault**.  
- Integrates with **Metaplex Core** to create a collection (with Oracle plugin for validation).  

### 3. `Preorder`
- Supporter preorders items from a drop.  
- Splits payment into:  
  - **Marketplace fees** → sent to treasury.  
  - **Campaign commitment** → sent to campaign vault.  
- Increases `supporter_count` and `pledged_orders`.  
- Finalizes campaign if `pledged_orders == goal_orders`.  

### 4. `Withdraw`
- Allows creator to withdraw funds after a **successful, finalized campaign**.  
- Deducts marketplace fees.  
- Closes campaign vault after withdrawal.  

### 5. `ClaimRefund`
- Allows supporters to refund if:  
  - Campaign is finalized **and unsuccessful**.  
- Transfers funds back from campaign vault to supporter.  
- Marks supporter as refunded.  

---

## ⚖️ Error Handling

The program uses a centralized error enum (`MarketplaceError`) with clear messages:  

- `CampaignFinalized` → Prevents changes after finalization.  
- `CampaignNotActive` → Ensures preorders only happen within timeframe.  
- `InvalidUnitsOrdered` → Guards against zero or exceeding allowed orders.  
- `AlreadyRefunded`, `AlreadyWithdrawn`, `AlreadyFinalized`, etc.  

---

## 🧪 Testing

- Uses **Anchor’s Mocha + Chai test framework**.  
- Covers flows: marketplace init, campaign init, preorder, refunds, and withdrawals.  
- To run tests:  

```bash
anchor test
```

---

## 🚀 Deployment

1. Generate a new program ID:  

```bash
solana-keygen new --outfile target/deploy/hue_marketplace-keypair.json
```

2. Update `Anchor.toml`:  

```toml
[programs.devnet]
hue_marketplace = "<YOUR_NEW_PROGRAM_ID>"
```

3. Build & deploy:  

```bash
anchor build
anchor deploy
```

---

## 📌 Future Work

- Soulbound Token (SBT) minting for successful campaigns.  
- Enhanced marketplace fee distribution.  
- More flexible campaign states (cancelation, extensions).  
- Frontend integration for campaign management + supporter UX.  
