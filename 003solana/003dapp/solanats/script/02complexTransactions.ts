// import custom helper for demos
import { payer, testWallet, connection, STATIC_PUBLICKEY } from "@/lib/vars";
import { explorerURL, printConsoleSeparator } from "@/lib/helpers";
import { SystemProgram, TransactionMessage, VersionedTransaction } from "@solana/web3.js";

(async () => {
  //////////////////////////////////////////////////////////////////////////////
  //////////////////////////////////////////////////////////////////////////////

  console.log("Payer address:", payer.publicKey.toBase58());
  console.log("Test wallet address:", testWallet.publicKey.toBase58());

  //////////////////////////////////////////////////////////////////////////////
  //////////////////////////////////////////////////////////////////////////////
  /**
   * create a simple instruction (using web3.js) to create an account
   */
  const space = 0; // on-chain space to allocated (in number of bytes)
  // request the cost (in lamports) to allocate `space` number of bytes on chain
  const balanceForRentExemption = await connection.getMinimumBalanceForRentExemption(space);

  // create this simple instruction using web3.js helper function
  const createTestAccountIx = SystemProgram.createAccount({
    fromPubkey: payer.publicKey,
    newAccountPubkey: testWallet.publicKey,
    lamports: balanceForRentExemption + 2_000_000,
    space,
    programId: SystemProgram.programId,
  });
  // create an instruction to transfer lamports
  const transferToTestWalletIx = SystemProgram.transfer({
    lamports: balanceForRentExemption + 100_000,
    // `fromPubkey` - from MUST sign the transaction
    fromPubkey: payer.publicKey,
    // `toPubkey` - does NOT have to sign the transaction
    toPubkey: testWallet.publicKey,
    programId: SystemProgram.programId,
  });

  // create another transaction to send to the  blockchain
  const transferToStaticWalletIx = SystemProgram.transfer({
    lamports: 100_000,
    fromPubkey: payer.publicKey,
    toPubkey: STATIC_PUBLICKEY,
    programId: SystemProgram.programId,
  });
  /**
   * build the transaction to send to the blockchain
   */

  // get the latest recent blockhash
  let recentBlockhash = await connection.getLatestBlockhash().then(res => res.blockhash);

  // create a transaction meessage
  const message = new TransactionMessage({
    payerKey: payer.publicKey,
    recentBlockhash,
    instructions: [
      createTestAccountIx,
      transferToStaticWalletIx,
      transferToTestWalletIx,
      transferToStaticWalletIx,
    ],
  }).compileToLegacyMessage();
  /**
   * try changing the order of the instructions inside of the message above...
   * see what happens :)
   */
  // create a versioned transaction using the message
  const tx = new VersionedTransaction(message);
  // console.log("tx before signing:", tx);

  // sign the transaction with our needed Signers (e.g. `payer` and `keypair`)
  tx.sign([payer, testWallet]);

  // actually send the transaction
  /**
   * display some helper text
   */
  const sig = await connection.sendTransaction(tx);
  /**
   * display some helper txt
   *  */
  printConsoleSeparator();
  console.log("Transaction complated.");
  console.log(explorerURL({ txSignature: sig }));
})();
