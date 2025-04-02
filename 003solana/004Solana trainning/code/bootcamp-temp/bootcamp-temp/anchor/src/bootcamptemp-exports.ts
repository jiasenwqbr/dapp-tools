// Here we export some useful types and functions for interacting with the Anchor program.
import { AnchorProvider, Program } from '@coral-xyz/anchor'
import { Cluster, PublicKey } from '@solana/web3.js'
import BootcamptempIDL from '../target/idl/bootcamptemp.json'
import type { Bootcamptemp } from '../target/types/bootcamptemp'

// Re-export the generated IDL and type
export { Bootcamptemp, BootcamptempIDL }

// The programId is imported from the program IDL.
export const BOOTCAMPTEMP_PROGRAM_ID = new PublicKey(BootcamptempIDL.address)

// This is a helper function to get the Bootcamptemp Anchor program.
export function getBootcamptempProgram(provider: AnchorProvider, address?: PublicKey) {
  return new Program({ ...BootcamptempIDL, address: address ? address.toBase58() : BootcamptempIDL.address } as Bootcamptemp, provider)
}

// This is a helper function to get the program ID for the Bootcamptemp program depending on the cluster.
export function getBootcamptempProgramId(cluster: Cluster) {
  switch (cluster) {
    case 'devnet':
    case 'testnet':
      // This is the program ID for the Bootcamptemp program on devnet and testnet.
      return new PublicKey('coUnmi3oBUtwtd9fjeAvSsJssXh5A5xyPbhpewyzRVF')
    case 'mainnet-beta':
    default:
      return BOOTCAMPTEMP_PROGRAM_ID
  }
}
