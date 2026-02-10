## Fungible and Non-Fungible Tokens

**Fungible Tokens:** These tokens are interchangeable with one another. They are indistinguishable and hold the same value.

**Non-Fungible Tokens (NFTs):** Each NFT is a special digital asset that holds a unique information or value. NFTs can represent ownership of a specific digital or physical item, such as digital art or real estate.

## Token Program

Token Program defines a common implementation for fungible and non-fungible tokens. All tokens on Solana are created using the [SPL Token Program](https://spl.solana.com/token).

!!! tip

    To see the source code, check out [Solana Program Library Token Program](https://github.com/solana-labs/solana-program-library/tree/master/token/program).


!!! info

    Solana's **SPL tokens** are similar to Ethereum's **ERC-20** or **ERC-721** standards. However, Solana does not require you to deploy a new contract for each token you create. Instead, you simply send instructions to the Token Program, which will create and mint tokens on your behalf.

![Blockchain](../../images/token_program.png)

## Creating Tokens

A token can be created by initializing a new [mint account](./mint-account.md). The mint account is used to create and hold info about new tokens, which are then stored in [token accounts](./token-account.md). Once a mint account is initialized, the **mint_authority** can create (mint) new tokens using the **MintTo** instruction.

!!! important

    As long as mint account contains a valid **mint_authority**, it is considered to have a non-fixed supply, and the **mint_authority** can create new tokens with the **MintTo** instruction at any time.

## Transferring Tokens

Balances can be transferred between accounts using the **Transfer** instruction, with the source account owner required as a signer when the accounts differ.

## Burning Tokens

The **Burn** instruction decreases an account's token balance without transferring to another account. The burnt tokens are permanently removed from the circulation, and this action can be verified on chain.

## Freezing Accounts

The mint account may include a **freeze_authority**, allowing it to invoke **FreezeAccount** instruction that will make account unusable. Frozen accounts can be reactivated using the **ThawAccount** instruction and **freeze_authority** can be changed using the **SetAuthority** instruction.

## Wrapping SOL

SOL can be wrapped and used like a Token Program token when interacting with programs that use the Token Program's interface. Accounts that hold wrapped SOL are associated with **Native Mint**. Wrapped SOL token accounts have unique behaviors, which you can learn more about [here](https://spl.solana.com/token#wrapping-sol).
