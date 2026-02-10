---
hide:
  - toc
---


## Smart Contract

A smart contract is a piece of code deployed on a blockchain with a cryptographically signed transaction. Users can then interact with it by sending transactions that invoke a specific function defined in the smart contract and the business logic is executed as stated in the deployed code.

Data relevant to the smart contract state are also stored on the blockchain. Hence we can look at smart contracts as programs on a decentralized computer that access files in its file system and modify them according to the predefined rules. If such a contract is made immutable, we can trust that the smart contract will not do anything else than what it is supposed to do.

!!! note

    Apart from storing the blockchain itself, each node creates a state as a result of transaction execution. The final state is the result of all processed transactions and can always be deterministically recreated from the blockchain history.

## ISA and Virtual Machines

Code is compiled for a predefined ISA ([Instruction Set Architecture](https://en.wikipedia.org/wiki/Instruction_set_architecture)) and executed in a VM (Virtual Machine) which understands it. The mentioned VM is a special runtime environment similar to well-known VMs such as JVM (Java Virtual Machine) or CLR (Common Language Runtime).

!!! info

    The Solana blockchain's execution environment is called SVM (Solana Virtual Machine). It is written in Rust and enables the blockchain to handle thousands of transactions per second.

!!! note

    Only transactions involving smart contract execution need to be processed by the VM.
