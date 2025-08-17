# Hue Marketplace Capstone

This project implements the Hue Marketplace Capstone program on Solana using Anchor. It provides smart contract functionality to initialize a marketplace, launch drop campaigns, support preorders from multiple supporters, process refunds, and mint soulbound tokens (SBTs) for supporters.

## Table of Contents

- [Overview](#overview)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Building & Deploying](#building--deploying)
- [Running Tests](#running-tests)
- [Project Structure](#project-structure)
- [Usage](#usage)
- [Contributing](#contributing)
- [License](#license)

## Overview

The Hue Marketplace Capstone project includes the following features:

- **Marketplace Initialization**: Set up marketplace configuration including fee parameters.
- **Campaign Creation**: Launch a drop campaign with a goal, price per unit, allowed units, end time and metadata URI.
- **Preorder**: Allow supporters to commit orders, enforcing limits per supporter.
- **Withdrawal & Refunds**: Enable withdrawal of funds once the campaign is successful and allow refunds if funds need to be returned.
- **Minting SBTs**: Mint soulbound tokens for supporters based on their participation.

## Prerequisites

Before running this project, ensure you have the following installed:

- [Rust](https://www.rust-lang.org/) with the nightly toolchain
- [Anchor CLI](https://book.anchor-lang.com/getting_started/installation.html)
- [Solana CLI](https://docs.solana.com/cli)
- [Node.js](https://nodejs.org/)
- [Yarn](https://classic.yarnpkg.com/en/)

## Installation

1. Clone the repository:
   ```sh
   git clone https://github.com/Worldkilas/Worldkilas-solana-turbin3-Q3_25_Builder_FAREED/tree/main/hue-marketplace-capstone
   cd hue-marketplace-capstone
   ```
