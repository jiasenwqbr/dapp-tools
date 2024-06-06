import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CheckingAccount } from "../target/types/checking_account";
import {
  Keypair,
  SystemProgram,
  Transaction,
  sendAndConfirmTransaction,
} from "@solana/web3.js";

describe("checking-account", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.CheckingAccount;
  const wallet = provider.wallet as anchor.Wallet;
  // We'll create this ahead of time.
  // Our program will try to modify it.
  const accountToChange = new Keypair();
  // Our program will create this.
  const accountToCreate = new Keypair();
  it("Create an account owned by our program", async () => {
    let instruction = SystemProgram.createAccount({
      fromPubkey: provider.wallet.publicKey,
      newAccountPubkey: accountToChange.publicKey,
      lamports: await provider.connection.getMinimumBalanceForRentExemption(0),
      space: 0,
      programId: program.programId,
    });
    const transaction = new Transaction().add(instruction);
    await sendAndConfirmTransaction(provider.connection, transaction, [
      wallet.payer,
      accountToChange,
    ]);
  });
  it("Checking account", async () => {
    await program.methods
      .checkAccounts()
      .accounts({
        payer: wallet.publicKey,
        accountToCreate: accountToCreate.publicKey,
        accountToChange: accountToChange.publicKey,
      })
      .rpc();
  });
});
