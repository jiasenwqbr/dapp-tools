import * as anchor from "@coral-xyz/anchor";
import { Keypair } from "@solana/web3.js";
import { AccountDataAnchorProgramExample } from "../target/types/account_data_anchor_program_example";

describe("account-data-anchor-program-example", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const payer = provider.wallet as anchor.Wallet;
  const program = anchor.workspace.AccountDataAnchorProgramExample;
  console.log(program);
  // Generate a new keypair for the addressInfo account
  const addressInfoAccount = new Keypair();
  it("Create the address info account", async () => {
    console.log(`Payer Address      : ${payer.publicKey}`);
    console.log(`Address Info Acct  : ${addressInfoAccount.publicKey}`);

    // Instruction data
    const addressInfo = {
      name: "Joe C",
      houseNumber: 136,
      street: "Mile High Dr.",
      city: "Solana Beach",
    };

    await program.methods
      .anchorProgramExample(
        addressInfo.name,
        addressInfo.houseNumber,
        addressInfo.street,
        addressInfo.city
      )
      .accounts({
        addressInfo: addressInfoAccount.publicKey,
        payer: payer.publicKey,
      })
      .signers([addressInfoAccount])
      .rpc();
  });

  it("Read the new account's data", async () => {
    const addressInfo = await program.account.addressInfo.fetch(
      addressInfoAccount.publicKey
    );
    console.log(`Name     : ${addressInfo.name}`);
    console.log(`House Num: ${addressInfo.houseNumber}`);
    console.log(`Street   : ${addressInfo.street}`);
    console.log(`City     : ${addressInfo.city}`);
  });
});
