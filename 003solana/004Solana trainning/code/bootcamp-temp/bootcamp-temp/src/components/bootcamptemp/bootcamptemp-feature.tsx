'use client'

import { useWallet } from '@solana/wallet-adapter-react'
import { WalletButton } from '../solana/solana-provider'
import { AppHero, ellipsify } from '../ui/ui-layout'
import { ExplorerLink } from '../cluster/cluster-ui'
import { useBootcamptempProgram } from './bootcamptemp-data-access'
import { BootcamptempCreate, BootcamptempList } from './bootcamptemp-ui'

export default function BootcamptempFeature() {
  const { publicKey } = useWallet()
  const { programId } = useBootcamptempProgram()

  return publicKey ? (
    <div>
      <AppHero
        title="Bootcamptemp"
        subtitle={
          'Create a new account by clicking the "Create" button. The state of a account is stored on-chain and can be manipulated by calling the program\'s methods (increment, decrement, set, and close).'
        }
      >
        <p className="mb-6">
          <ExplorerLink path={`account/${programId}`} label={ellipsify(programId.toString())} />
        </p>
        <BootcamptempCreate />
      </AppHero>
      <BootcamptempList />
    </div>
  ) : (
    <div className="max-w-4xl mx-auto">
      <div className="hero py-[64px]">
        <div className="hero-content text-center">
          <WalletButton />
        </div>
      </div>
    </div>
  )
}
