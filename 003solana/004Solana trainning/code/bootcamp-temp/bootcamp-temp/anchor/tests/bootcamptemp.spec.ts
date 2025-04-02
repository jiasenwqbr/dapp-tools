import * as anchor from '@coral-xyz/anchor'
import { Program } from '@coral-xyz/anchor'
import { Keypair } from '@solana/web3.js'
import { Bootcamptemp } from '../target/types/bootcamptemp'

describe('bootcamptemp', () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)
  const payer = provider.wallet as anchor.Wallet

  const program = anchor.workspace.Bootcamptemp as Program<Bootcamptemp>

  const bootcamptempKeypair = Keypair.generate()

  it('Initialize Bootcamptemp', async () => {
    await program.methods
      .initialize()
      .accounts({
        bootcamptemp: bootcamptempKeypair.publicKey,
        payer: payer.publicKey,
      })
      .signers([bootcamptempKeypair])
      .rpc()

    const currentCount = await program.account.bootcamptemp.fetch(bootcamptempKeypair.publicKey)

    expect(currentCount.count).toEqual(0)
  })

  it('Increment Bootcamptemp', async () => {
    await program.methods.increment().accounts({ bootcamptemp: bootcamptempKeypair.publicKey }).rpc()

    const currentCount = await program.account.bootcamptemp.fetch(bootcamptempKeypair.publicKey)

    expect(currentCount.count).toEqual(1)
  })

  it('Increment Bootcamptemp Again', async () => {
    await program.methods.increment().accounts({ bootcamptemp: bootcamptempKeypair.publicKey }).rpc()

    const currentCount = await program.account.bootcamptemp.fetch(bootcamptempKeypair.publicKey)

    expect(currentCount.count).toEqual(2)
  })

  it('Decrement Bootcamptemp', async () => {
    await program.methods.decrement().accounts({ bootcamptemp: bootcamptempKeypair.publicKey }).rpc()

    const currentCount = await program.account.bootcamptemp.fetch(bootcamptempKeypair.publicKey)

    expect(currentCount.count).toEqual(1)
  })

  it('Set bootcamptemp value', async () => {
    await program.methods.set(42).accounts({ bootcamptemp: bootcamptempKeypair.publicKey }).rpc()

    const currentCount = await program.account.bootcamptemp.fetch(bootcamptempKeypair.publicKey)

    expect(currentCount.count).toEqual(42)
  })

  it('Set close the bootcamptemp account', async () => {
    await program.methods
      .close()
      .accounts({
        payer: payer.publicKey,
        bootcamptemp: bootcamptempKeypair.publicKey,
      })
      .rpc()

    // The account should no longer exist, returning null.
    const userAccount = await program.account.bootcamptemp.fetchNullable(bootcamptempKeypair.publicKey)
    expect(userAccount).toBeNull()
  })
})
