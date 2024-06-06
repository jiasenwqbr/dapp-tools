import {
  Keypair,
  PublicKey,
  SystemProgram,
  Transaction,
  TransactionInstruction,
} from "@solana/web3.js";
import { describe, test } from "node:test";
import { start } from "solana-bankrun";
describe("checking account", async () => {
  const PROGRAM_ID = PublicKey.unique();
  const context = await start(
    [{ name: "checking_accounts_program", programId: PROGRAM_ID }],
    []
  );
  const client = context.banksClient;
  const payer = context.payer;
  const rent = await client.getRent();

  // We'll create this ahead of time.
  // Our program will try to modify it.
  const accountToChange = Keypair.generate();
  // Our program will create this
  const accountToCreate = Keypair.generate();

  test("carete an account owned by our program", async () => {
    const blockhash = context.lastBlockhash;
    let ix = SystemProgram.createAccount({
      fromPubkey: payer.publicKey,
      newAccountPubkey: accountToChange.publicKey,
      lamports: Number(rent.minimumBalance(BigInt(0))),
      space: 0,
      programId: PROGRAM_ID, // Our program
    });
    const tx = new Transaction();
    tx.recentBlockhash = blockhash;
    tx.add(ix).sign(payer, accountToChange);

    await client.processTransaction(tx);
  });

  test("Check accounts", async () => {
    const blockhash = context.lastBlockhash;
    let ix = new TransactionInstruction({
      keys: [
        { pubkey: payer.publicKey, isSigner: true, isWritable: true },
        { pubkey: accountToCreate.publicKey, isSigner: true, isWritable: true },
        { pubkey: accountToChange.publicKey, isSigner: true, isWritable: true },
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
      ],
      programId: PROGRAM_ID,
      data: Buffer.alloc(0),
    });

    const tx = new Transaction();
    tx.recentBlockhash = blockhash;
    tx.add(ix).sign(payer, accountToChange, accountToCreate);

    await client.processTransaction(tx);
  });
});
