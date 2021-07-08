import load from './loader'
import { exists, getChainName, makeRow, timestampWithMS } from './general'
import getChangedPools from './changedPools'
import { Chain, Wallet } from './types'
import WalletLoader from './walletLoader'
import { LOG_BOUNDER, RUN_SIMPLES, RUN_TRIS } from './constants'
import { getSimpleRoutes, getTriRoutes } from './routeBuilder'
import { bundleRoutesAndBid } from './bundleBidder'

export const { chain, web3, web3Provider, stdToken, stdExchange, hotAddress, hotPrivateKey, optimizerExec, optimizerExecAddress, exchanges, abiDecoder, routers, routerExchangeMap, routerForkTypeMap, tokenMap, outerTokens, ignoreTokens, preferedTokens, innerTokens, bidSimple, bidTri } = load()
export const wallets: Wallet[] = []
export const activeFunctionCount = { count: 0 }

const main = async () => {
	const walletLoader = new WalletLoader(web3, hotAddress, hotPrivateKey)
	const loadedWallets = await walletLoader.loadWallets()
	loadedWallets.forEach((loadedWallet) => wallets.push(loadedWallet))

	console.log(LOG_BOUNDER)
	console.log(`==== Loaded ${wallets.length} Wallets`)
	console.log(`==== Target Backrunner Started on ${getChainName()}`)
	console.log(LOG_BOUNDER)

	/* PENDING TX SUBSCRIPTION */
	web3.eth.subscribe('pendingTransactions').on('data', async (transaction) => {
		const txData = await web3.eth.getTransaction(transaction).catch(() => undefined)
		if (exists(txData)) {
			const changedPools = await getChangedPools(txData)
			if (changedPools.length > 0) {
				const simpleRoutes = RUN_SIMPLES ? getSimpleRoutes(changedPools) : []
				const triRoutes = RUN_TRIS ? getTriRoutes(changedPools) : []
				bundleRoutesAndBid(simpleRoutes, triRoutes, changedPools[0].gasPrice!)

				console.log(activeFunctionCount.count)
			}
		}
	})
}
main()
