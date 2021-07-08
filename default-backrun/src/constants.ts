import BigNumber from 'bignumber.js'
import { chain } from './main'
import { Chain } from './types'

// Routing
export const RUN_TRIS = process.env.RUN_TRIS === 'true' ? true : false

// Wallets
export const GEN_NEW_WALLETS = process.env.GEN_NEW_WALLETS === 'true' ? true : false
export const NUM_WALLETS = process.env.NUM_WALLETS ? Number(process.env.NUM_WALLETS) : 150
export const WALLET_BALANCE = process.env.WALLET_BALANCE ? Number(process.env.WALLET_BALANCE) : 0.19
export const WALLETS_PATH = chain === Chain.Polygon ? process.env.WALLETS_PATH : '/saved/wallets.json'
export const DEFAULT_GP_WL_GWEI = process.env.DEFAULT_GP_WL_GWEI ? Number(process.env.DEFAULT_GP_WL_GWEI) : 1

// BigNumber Constants
export const ONE_GWEI_BN = new BigNumber('1000000000')

// Bidding
export const MS_SPAM_INTERVAL = process.env.MS_SPAM_INTERVAL ? Number(process.env.MS_SPAM_INTERVAL) : 500
export const BACKRUN_GWEIS = (process.env.BACKRUN_GWEIS ? process.env.BACKRUN_GWEIS : '1, 2, 3')!.split(', ').map((gwei) => {
	return ONE_GWEI_BN.multipliedBy(Number(gwei))
})
export const SIMPLE_GAS_ESTIMATE = new BigNumber(process.env.SIMPLE_GAS_ESTIMATE ? process.env.SIMPLE_GAS_ESTIMATE : '100000')
export const TRI_GAS_ESTIMATE = new BigNumber(process.env.TRI_GAS_ESTIMATE ? process.env.TRI_GAS_ESTIMATE : '300000')
export const MIN_GAS_LIMIT = new BigNumber(500000)
export const OPP_EXEC_CONTRACT = process.env.OPP_EXEC_CONTRACT ? process.env.OPP_EXEC_CONTRACT : 'ADD CONTRACT TO ENV'

// Logging
export const LOG_LENGTH = 40
export const LOG_BOUNDER = '='.repeat(LOG_LENGTH - 2)
export const ERROR_BOUNDER = '!'.repeat(LOG_LENGTH * 2)
export const TRANSACTION_BOUNDER = '$'.repeat(LOG_LENGTH - 2)

export const INCOME_PATH = '/saved/income.json'

export const SimpleBackrun = {
	SimpleBackrun: {
		outerToken: 'address',
		innerToken: 'address',
		exchange1: 'address',
		exchange2: 'address',
		swapFee1: 'uint256',
		swapFee2: 'uint256',
	},
}

export const TriBackrun = {
	TriBackrun: {
		token1: 'address',
		token2: 'address',
		token3: 'address',
		exchange1: 'address',
		exchange2: 'address',
		exchange3: 'address',
		swapFee1: 'uint256',
		swapFee2: 'uint256',
		swapFee3: 'uint256',
	},
}
