'use client'

import { getBootcamptempProgram, getBootcamptempProgramId } from '@project/anchor'
import { useConnection } from '@solana/wallet-adapter-react'
import { Cluster, Keypair, PublicKey } from '@solana/web3.js'
import { useMutation, useQuery } from '@tanstack/react-query'
import { useMemo } from 'react'
import toast from 'react-hot-toast'
import { useCluster } from '../cluster/cluster-data-access'
import { useAnchorProvider } from '../solana/solana-provider'
import { useTransactionToast } from '../ui/ui-layout'

export function useBootcamptempProgram() {
  const { connection } = useConnection()
  const { cluster } = useCluster()
  const transactionToast = useTransactionToast()
  const provider = useAnchorProvider()
  const programId = useMemo(() => getBootcamptempProgramId(cluster.network as Cluster), [cluster])
  const program = useMemo(() => getBootcamptempProgram(provider, programId), [provider, programId])

  const accounts = useQuery({
    queryKey: ['bootcamptemp', 'all', { cluster }],
    queryFn: () => program.account.bootcamptemp.all(),
  })

  const getProgramAccount = useQuery({
    queryKey: ['get-program-account', { cluster }],
    queryFn: () => connection.getParsedAccountInfo(programId),
  })

  const initialize = useMutation({
    mutationKey: ['bootcamptemp', 'initialize', { cluster }],
    mutationFn: (keypair: Keypair) =>
      program.methods.initialize().accounts({ bootcamptemp: keypair.publicKey }).signers([keypair]).rpc(),
    onSuccess: (signature) => {
      transactionToast(signature)
      return accounts.refetch()
    },
    onError: () => toast.error('Failed to initialize account'),
  })

  return {
    program,
    programId,
    accounts,
    getProgramAccount,
    initialize,
  }
}

export function useBootcamptempProgramAccount({ account }: { account: PublicKey }) {
  const { cluster } = useCluster()
  const transactionToast = useTransactionToast()
  const { program, accounts } = useBootcamptempProgram()

  const accountQuery = useQuery({
    queryKey: ['bootcamptemp', 'fetch', { cluster, account }],
    queryFn: () => program.account.bootcamptemp.fetch(account),
  })

  const closeMutation = useMutation({
    mutationKey: ['bootcamptemp', 'close', { cluster, account }],
    mutationFn: () => program.methods.close().accounts({ bootcamptemp: account }).rpc(),
    onSuccess: (tx) => {
      transactionToast(tx)
      return accounts.refetch()
    },
  })

  const decrementMutation = useMutation({
    mutationKey: ['bootcamptemp', 'decrement', { cluster, account }],
    mutationFn: () => program.methods.decrement().accounts({ bootcamptemp: account }).rpc(),
    onSuccess: (tx) => {
      transactionToast(tx)
      return accountQuery.refetch()
    },
  })

  const incrementMutation = useMutation({
    mutationKey: ['bootcamptemp', 'increment', { cluster, account }],
    mutationFn: () => program.methods.increment().accounts({ bootcamptemp: account }).rpc(),
    onSuccess: (tx) => {
      transactionToast(tx)
      return accountQuery.refetch()
    },
  })

  const setMutation = useMutation({
    mutationKey: ['bootcamptemp', 'set', { cluster, account }],
    mutationFn: (value: number) => program.methods.set(value).accounts({ bootcamptemp: account }).rpc(),
    onSuccess: (tx) => {
      transactionToast(tx)
      return accountQuery.refetch()
    },
  })

  return {
    accountQuery,
    closeMutation,
    decrementMutation,
    incrementMutation,
    setMutation,
  }
}
