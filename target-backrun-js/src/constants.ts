import BigNumber from 'bignumber.js'
import { chain } from './main'
import { Chain } from './types'

// Routing
export const RUN_SIMPLES = process.env.RUN_SIMPLES === 'true' ? true : false
export const RUN_TRIS = process.env.RUN_TRIS === 'true' ? true : false
export const BUNDLE_SIZE = process.env.BUNDLE_SIZE ? Number(process.env.BUNDLE_SIZE) : 2

// Wallets
export const GEN_NEW_WALLETS = process.env.GEN_NEW_WALLETS === 'true' ? true : false
export const NUM_WALLETS = process.env.NUM_WALLETS ? Number(process.env.NUM_WALLETS) : 150
export const WALLET_BALANCE = process.env.WALLET_BALANCE ? Number(process.env.WALLET_BALANCE) : 0.19
export const WALLETS_PATH = chain === Chain.Polygon ? process.env.WALLETS_PATH : '/saved/wallets.json'
export const DEFAULT_GP_WL_GWEI = process.env.DEFAULT_GP_WL_GWEI ? Number(process.env.DEFAULT_GP_WL_GWEI) : 10

// BigNumber Constants
export const ONE_GWEI_BN = new BigNumber('1000000000')

// Bidding
export const REDUNDANCY = process.env.REDUNDANCY ? Number(process.env.REDUNDANCY) : 1
export const MAX_GAS_PRICE = new BigNumber(process.env.MAX_GAS_PRICE ? process.env.MAX_GAS_PRICE : '100').multipliedBy(ONE_GWEI_BN)
export const SIMPLE_GAS_ESTIMATE = new BigNumber(process.env.SIMPLE_GAS_ESTIMATE ? process.env.SIMPLE_GAS_ESTIMATE : '150000')
export const TRI_GAS_ESTIMATE = new BigNumber(process.env.TRI_GAS_ESTIMATE ? process.env.TRI_GAS_ESTIMATE : '200000')
export const MIN_GAS_LIMIT = new BigNumber(320000)
export const OPP_EXEC_CONTRACT = process.env.OPP_EXEC_CONTRACT ? process.env.OPP_EXEC_CONTRACT : 'ADD CONTRACT TO ENV'
export const CHI_GAS_CUT_OFF = new BigNumber(5000000000)

// Logging
export const LOG_ARBS = false
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
		swapFeeSum: 'uint256',
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
		swapFeeSum: 'uint256',
	},
}
