import { address } from 'gill'
import { SolanaClusterId } from '@wallet-ui/react'
import { TICKET_REGISTRY_PROGRAM_ADDRESS } from './client/js'
import TicketRegistryIDL from '../target/idl/ticket_registry.json'

export { TicketRegistryIDL }

export function getTicketRegistryProgramId(cluster: SolanaClusterId) {
  switch (cluster) {
    case 'solana:devnet':
    case 'solana:testnet':
      return address('7occibxbsT7JbhBAC3BwXhzXhuR31GiKpuHNoqv7hm3X')
    case 'solana:mainnet':
    default:
      return TICKET_REGISTRY_PROGRAM_ADDRESS
  }
}

export * from './client/js'
