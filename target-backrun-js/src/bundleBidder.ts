import BigNumber from 'bignumber.js'
import { BUNDLE_SIZE, LOG_ARBS, TRANSACTION_BOUNDER, SIMPLE_GAS_ESTIMATE, TRI_GAS_ESTIMATE, REDUNDANCY, MIN_GAS_LIMIT, CHI_GAS_CUT_OFF } from './constants'
import { encode } from './encoders'
import { exists, timestampWithMS } from './general'
import { activeFunctionCount, optimizerExec, optimizerExecAddress, web3, abiDecoder, wallets, chain, bidTri, bidSimple } from './main'
import { ArbType, Chain, Route, RouteBundle, Wallet } from './types'
import { Mutex } from 'async-mutex'

const mutex = new Mutex()

export function bundleRoutesAndBid(simpleRoutes: Route[], triRoutes: Route[], gasPrice: BigNumber) {
	while (simpleRoutes.length > 0) {
		const routesToSplice = Math.min(BUNDLE_SIZE, simpleRoutes.length)
		const routesInBundle: Route[] = simpleRoutes.splice(0, routesToSplice)
		const routeBundle = {
			type: ArbType.Simple,
			routes: routesInBundle,
			encodedRoutes: encode(routesInBundle, ArbType.Simple),
			gasPrice,
		}
		bidOnBundleSimple(routeBundle)
	}

	while (triRoutes.length > 0) {
		const routesToSplice = Math.min(BUNDLE_SIZE, triRoutes.length)
		const routesInBundle: Route[] = triRoutes.splice(0, routesToSplice)
		const routeBundle = {
			type: ArbType.Tri,
			routes: routesInBundle,
			encodedRoutes: encode(routesInBundle, ArbType.Tri),
			gasPrice,
		}
		bidOnBundleTri(routeBundle)
	}
}

async function bidOnBundleSimple(routeBundle: RouteBundle) {
	for (let i = 0; i < REDUNDANCY; i++) {
		let wallet = wallets.shift()
		wallets.push(wallet!)
		if (!exists(wallet)) console.log('Wallets are out of gas')
		bidSimple(routeBundle, wallet!)
	}
}

async function bidOnBundleTri(routeBundle: RouteBundle) {
	for (let i = 0; i < REDUNDANCY; i++) {
		let wallet = wallets.shift()
		wallets.push(wallet!)
		if (!exists(wallet)) console.log('Wallets are out of gas')
		bidTri(routeBundle, wallet!)
	}
}

export function bidSimpleChi(routeBundle: RouteBundle, wallet: Wallet) {
	activeFunctionCount.count++

	const timeSent = timestampWithMS()

	let tx
	if (routeBundle.gasPrice.gte(CHI_GAS_CUT_OFF)) {
		tx = optimizerExec.methods.simpleMulticallChi(routeBundle.encodedRoutes)
	} else {
		tx = optimizerExec.methods.simpleMulticall(routeBundle.encodedRoutes)
	}

	const gasEstimate = calculateGasEstimate(routeBundle)
	const txData = {
		from: wallet.account,
		to: optimizerExecAddress,
		gas: gasEstimate,
		gasPrice: routeBundle.gasPrice.toString(),
		data: tx.encodeABI(),
		chainId: web3.eth.defaultCommon.customChain.chainId,
		common: web3.eth.defaultCommon,
		nonce: wallet.nonce,
	}

	web3.eth
		.sendTransaction(txData)
		.then((receipt) => {
			// update the wallet nonce
			wallet.nonce++

			logArb(routeBundle, receipt, timeSent, gasEstimate)
		})
		.catch((err) => handleError(err.message, wallet))
}

export function bidTriChi(routeBundle: RouteBundle, wallet: Wallet) {
	activeFunctionCount.count++

	const timeSent = timestampWithMS()
	let tx
	if (routeBundle.gasPrice.gte(CHI_GAS_CUT_OFF)) {
		tx = optimizerExec.methods.triMulticallChi(routeBundle.encodedRoutes)
	} else {
		tx = optimizerExec.methods.triMulticall(routeBundle.encodedRoutes)
	}

	const gasEstimate = calculateGasEstimate(routeBundle)
	const txData = {
		from: wallet.account,
		to: optimizerExecAddress,
		gas: gasEstimate,
		gasPrice: routeBundle.gasPrice.toString(),
		data: tx.encodeABI(),
		chainId: web3.eth.defaultCommon.customChain.chainId,
		common: web3.eth.defaultCommon,
		nonce: wallet.nonce,
	}

	web3.eth
		.sendTransaction(txData)
		.then((receipt) => {
			// update the wallet nonce
			wallet.nonce++

			logArb(routeBundle, receipt, timeSent, gasEstimate)
		})
		.catch((err) => handleError(err.message, wallet))
}

export function bidSimpleNoChi(routeBundle: RouteBundle, wallet: Wallet) {
	activeFunctionCount.count++

	const timeSent = timestampWithMS()

	const tx = optimizerExec.methods.simpleMulticall(routeBundle.encodedRoutes)

	const gasEstimate = calculateGasEstimate(routeBundle)
	const txData = {
		from: wallet.account,
		to: optimizerExecAddress,
		gas: gasEstimate,
		gasPrice: routeBundle.gasPrice.toString(),
		data: tx.encodeABI(),
		chainId: web3.eth.defaultCommon.customChain.chainId,
		common: web3.eth.defaultCommon,
		nonce: wallet.nonce,
	}

	web3.eth
		.sendTransaction(txData)
		.then((receipt) => {
			// update the wallet nonce
			wallet.nonce++

			logArb(routeBundle, receipt, timeSent, gasEstimate)
		})
		.catch((err) => handleError(err.message, wallet))
}

export function bidTriNoChi(routeBundle: RouteBundle, wallet: Wallet) {
	activeFunctionCount.count++

	const timeSent = timestampWithMS()
	const tx = optimizerExec.methods.triMulticall(routeBundle.encodedRoutes)

	const gasEstimate = calculateGasEstimate(routeBundle)
	const txData = {
		from: wallet.account,
		to: optimizerExecAddress,
		gas: gasEstimate,
		gasPrice: routeBundle.gasPrice.toString(),
		data: tx.encodeABI(),
		chainId: web3.eth.defaultCommon.customChain.chainId,
		common: web3.eth.defaultCommon,
		nonce: wallet.nonce,
	}

	web3.eth
		.sendTransaction(txData)
		.then((receipt) => {
			// update the wallet nonce
			wallet.nonce++

			logArb(routeBundle, receipt, timeSent, gasEstimate)
		})
		.catch((err) => handleError(err.message, wallet))
}

function logArb(routeBundle: RouteBundle, receipt: any, timeSent: string, gasEstimate: string) {
	if (LOG_ARBS) {
		let logs = abiDecoder.decodeLogs(receipt.logs)
		logs.forEach((element) => {
			if (element.events.length > 0) {
				element.events.forEach((event) => {
					if (event.name === 'diff') {
						console.log(TRANSACTION_BOUNDER)
						console.log(`$$$ Gas limit of ${gasEstimate}  @ ${routeBundle.gasPrice.shiftedBy(-9).toString()} GWEI`)
						console.log(`$$$ Sent @    ${timeSent}`)
						console.log(`$$$ Taken @  ${timestampWithMS()}`)
						console.log(TRANSACTION_BOUNDER)
					}
				})
			}
		})
	}
	activeFunctionCount.count--
}

async function handleError(message: string, wallet: Wallet) {
	if (!message.includes('50 block')) {
		if (message.includes('insufficient')) {
			wallets.forEach((_wallet, i) => {
				if (JSON.stringify(_wallet) === JSON.stringify(wallet)) {
					wallets.splice(i, 1)
				}
			})
		} else if (message.includes('too low')) {
			// update the wallet nonce
			wallet.nonce++
		}
		console.log(message)
	} else {
		wallet.nonce = await web3.eth.getTransactionCount(wallet.account.address)
	}
	activeFunctionCount.count--
}

function calculateGasEstimate(routeBundle: RouteBundle) {
	const oneRouteGE = routeBundle.type === ArbType.Simple ? SIMPLE_GAS_ESTIMATE : TRI_GAS_ESTIMATE
	return oneRouteGE.multipliedBy(routeBundle.routes.length).plus(MIN_GAS_LIMIT).toString()
}
