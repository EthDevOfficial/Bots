import load from './loader'
import { timestampWithMS, sleep, getChainName, makeRow } from './general'
import { Wallet } from './types'
import WalletLoader from './walletLoader'
import RouteManager from './routeManager'
import { LOG_BOUNDER, MS_SPAM_INTERVAL } from './constants'

export const { chain, web3, web3Provider, tokenPaths, stdToken, stdExchange, hotAddress, hotPrivateKey, optimizerExec, optimizerExecAddress, exchanges, abiDecoder } = load()
export const block: { number: number; startTime: string } = { number: 0, startTime: '' }

export const wallets: Wallet[] = []
const main = async () => {
	const walletLoader = new WalletLoader(web3, hotAddress, hotPrivateKey)
	const loadedWallets = await walletLoader.loadWallets()
	loadedWallets.forEach((loadedWallet) => wallets.push(loadedWallet))

	console.log(LOG_BOUNDER)
	console.log(`==== Loaded ${wallets.length} Wallets`)
	console.log(`==== ${makeRow('Default Backrunner Started on:', `${getChainName()}`)} ====`)
	console.log(LOG_BOUNDER)

	/* NEW BLOCK SUBSCRIPTION */
	web3.eth.subscribe('newBlockHeaders').on('data', async (newBlock) => {
		block.startTime = timestampWithMS()
		block.number = newBlock.number
	})

	const routeManager = new RouteManager(tokenPaths)
	while (true) {
		routeManager.bidOnAllRoutes()
		await sleep(MS_SPAM_INTERVAL)
	}
}
main()
