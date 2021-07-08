import load from './loader'
import { getChainName, makeRow, timestampWithMS } from './general'
import getChangedPools from './changedPools'
import { Chain, Wallet } from './types'
import WalletLoader from './walletLoader'
import RouteManager from './routeManager'
import { LOG_BOUNDER } from './constants'

export const { chain, web3, web3Provider, stdToken, stdExchange, hotAddress, hotPrivateKey, optimizerExec, optimizerExecAddress, exchanges, abiDecoder, routers, routerExchangeMap, routerForkTypeMap, tokenMap, outerTokens, ignoreTokens, preferedTokens } = load()
export const block: { number: number; startTime: string } = { number: 0, startTime: '' }
export const wallets: Wallet[] = []

const main = async () => {
	const walletLoader = new WalletLoader(web3, hotAddress, hotPrivateKey)
	const loadedWallets = await walletLoader.loadWallets()
	loadedWallets.forEach((loadedWallet) => wallets.push(loadedWallet))

	console.log(LOG_BOUNDER)
	console.log(`==== Loaded ${wallets.length} Wallets`)
	console.log(`==== ${makeRow('Target Backrunner Started on', `${getChainName()}`)} ====`)
	console.log(LOG_BOUNDER)

	/* PENDING TX SUBSCRIPTION */
	web3.eth.subscribe('pendingTransactions').on('data', async (transaction) => {
		const txData = await web3.eth.getTransaction(transaction).catch(() => undefined)
		const changedPools = await getChangedPools(txData)
		if (changedPools.length > 0) {
			if (chain === Chain.Fantom) {
				changedPools.forEach((changedPool) => {
					const routeManager = new RouteManager([changedPool], block.number)
					routeManager.bidOnAllRoutes()
				})
			} else {
				const routeManager = new RouteManager(changedPools, block.number)
				routeManager.bidOnAllRoutes()
			}
		}
	})

	/* NEW BLOCK SUBSCRIPTION */
	web3.eth.subscribe('newBlockHeaders').on('data', async (newBlock) => {
		block.startTime = timestampWithMS()
		block.number = newBlock.number
	})
}
main()
