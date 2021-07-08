import { BUNDLE_SIZE, MIN_GAS_LIMIT, REDUNDANCY, RUN_SIMPLES, RUN_TRIS, SIMPLE_GAS_ESTIMATE, TRANSACTION_BOUNDER, TRI_GAS_ESTIMATE } from './constants'
import { exists, timestampWithMS } from './general'
import { abiDecoder, block, exchanges, optimizerExec, optimizerExecAddress, outerTokens, stdExchange, wallets, web3 } from './main'

import { ArbType, Exchange, Pool, Route, RouteBundle, Token, Wallet } from './types'
import BigNumber from 'bignumber.js'
import { encode } from './encoders'

class RouteManager {
	private changedPools: Pool[]
	private block: number
	private simpleRoutes: Route[] = []
	private triRoutes: Route[] = []
	private routeBundles: RouteBundle[] = []
	private gasPrice: BigNumber

	constructor(changedPools: Pool[], blockNum: number) {
		this.changedPools = changedPools
		this.gasPrice = changedPools[0].gasPrice!
		this.block = blockNum
		if (RUN_SIMPLES) this.getSimpleRoutes()
		if (RUN_TRIS) this.getTriRoutes()
		this.bundleRoutes()
	}

	bidOnAllRoutes() {
		for (let i = 0; i < REDUNDANCY; i++) {
			let _routeBundles = this.routeBundles.map((route) => route)
			while (_routeBundles.length > 0) {
				let wallet = wallets.shift()
				wallets.push(wallet!)
				if (!exists(wallet)) console.log('Wallets are out of gas')
				this.bidOnBundle(_routeBundles.shift()!, wallet!)
			}
		}
	}

	private bidOnBundle(routeBundle: RouteBundle, wallet: Wallet) {
		const timeSent = timestampWithMS()
		const tx = routeBundle.type === ArbType.Simple ? optimizerExec.methods.simpleMulticall(routeBundle.encodedRoutes) : optimizerExec.methods.triMulticall(routeBundle.encodedRoutes)
		const gasEstimate = this.calculateGasEstimate(routeBundle)
		const txData = {
			from: wallet.account,
			to: optimizerExecAddress,
			gas: gasEstimate,
			gasPrice: this.gasPrice.toString(),
			data: tx.encodeABI(),
		}

		web3.eth
			.sendTransaction(txData)
			.then((receipt) => this.logArb(receipt, timeSent, gasEstimate))
			.catch((err) => this.handleError(err.message, wallet))
	}

	private logArb(receipt: any, timeSent: string, gasEstimate: string) {
		let logs = abiDecoder.decodeLogs(receipt.logs)
		logs.forEach((element) => {
			if (element.events.length > 0) {
				element.events.forEach((event) => {
					if (event.name === 'diff') {
						console.log(TRANSACTION_BOUNDER)
						console.log(`$$$ Gas limit of ${gasEstimate}  @ ${this.gasPrice.shiftedBy(-9).toString()} GWEI`)
						console.log(`$$$ Sent @    ${timeSent}   Block ${this.block}`)
						console.log(`$$$ Taken @  ${timestampWithMS()}   Block ${block.number}`)
						console.log(TRANSACTION_BOUNDER)
					}
				})
			}
		})
	}

	private handleError(message: string, wallet: Wallet) {
		if (!message.includes('50 blocks')) {
			if (!message.includes('underpriced') && !message.includes('is too low')) {
				console.log(message)
				if (message.includes('insufficient')) {
					wallets.forEach((_wallet, i) => {
						if (JSON.stringify(_wallet) === JSON.stringify(wallet)) {
							wallets.splice(i, 1)
						}
					})
				}
			}
		}
	}

	private calculateGasEstimate(routeBundle: RouteBundle) {
		const oneRouteGE = routeBundle.type === ArbType.Simple ? SIMPLE_GAS_ESTIMATE : TRI_GAS_ESTIMATE
		return oneRouteGE.multipliedBy(routeBundle.routes.length).plus(MIN_GAS_LIMIT).toString()
	}

	/*==========================================CONSTRUCTION HELPERS===================================================*/

	private bundleRoutes() {
		while (this.simpleRoutes.length > 0) {
			const routesInBundle: Route[] = []
			const routesToSplice = Math.min(BUNDLE_SIZE, this.simpleRoutes.length)
			routesInBundle.push(...this.simpleRoutes.splice(0, routesToSplice))
			this.routeBundles.push({
				type: ArbType.Simple,
				routes: routesInBundle,
				encodedRoutes: encode(routesInBundle, ArbType.Simple),
			})
		}

		while (this.triRoutes.length > 0) {
			const routesInBundle: Route[] = []
			const routesToSplice = Math.min(BUNDLE_SIZE, this.triRoutes.length)
			routesInBundle.push(...this.triRoutes.splice(0, routesToSplice))
			this.routeBundles.push({
				type: ArbType.Tri,
				routes: routesInBundle,
				encodedRoutes: encode(routesInBundle, ArbType.Tri),
			})
		}
	}

	private getSimpleRoutes() {
		this.changedPools.forEach((changedPool) => {
			exchanges.forEach((exchange) => {
				if (JSON.stringify(exchange) !== JSON.stringify(changedPool.exchange)) {
					const simpleRoute = this.buildSimpleRoute(changedPool, exchange)
					if (changedPool.isParaswap!) {
						this.simpleRoutes.push(simpleRoute)
						this.simpleRoutes.push(this.reverseSimpleRoute(simpleRoute))
					} else {
						// Choose the profitable direction
						changedPool.swapped ? this.simpleRoutes.push(simpleRoute) : this.simpleRoutes.push(this.reverseSimpleRoute(simpleRoute))
					}
				}
			})
		})
	}

	private buildSimpleRoute(changedPool: Pool, exchange: Exchange): Route {
		return {
			type: ArbType.Simple,
			poolRoute: [
				changedPool,
				{
					token1: changedPool.token2,
					token2: changedPool.token1,
					exchange,
				},
			],
		}
	}

	private getTriRoutes() {
		this.changedPools.forEach((changedPool) => {
			outerTokens.forEach((token: Token) => {
				if (JSON.stringify(token) !== JSON.stringify(changedPool.token1) && JSON.stringify(token) !== JSON.stringify(changedPool.token2)) {
					const triRoute = this.buildInnerTriRoute(changedPool, token)
					if (changedPool.isParaswap) {
						this.triRoutes.push(triRoute)
						this.triRoutes.push(this.reverseTriRoute(triRoute))
					} else {
						// Choose the profitable direction
						changedPool.swapped ? this.triRoutes.push(triRoute) : this.triRoutes.push(this.reverseTriRoute(triRoute))
					}
				}
			})
		})
	}

	private buildInnerTriRoute(changedPool: Pool, outerToken: Token) {
		return {
			type: ArbType.Tri,
			poolRoute: [
				{
					token1: outerToken,
					token2: changedPool.token1,
					exchange: changedPool.exchange,
				},
				changedPool,
				{
					token1: changedPool.token2,
					token2: outerToken,
					exchange: changedPool.exchange,
				},
			],
		}
	}

	private reverseSimpleRoute(simpleRoute: Route): Route {
		return {
			...simpleRoute,
			poolRoute: [
				{
					...simpleRoute.poolRoute[1],
					token1: simpleRoute.poolRoute[1].token2,
					token2: simpleRoute.poolRoute[1].token1,
				},
				{
					...simpleRoute.poolRoute[0],
					token1: simpleRoute.poolRoute[0].token2,
					token2: simpleRoute.poolRoute[0].token1,
				},
			],
		}
	}

	private reverseTriRoute(triRoute: Route): Route {
		return {
			...triRoute,
			poolRoute: [
				{
					...triRoute.poolRoute[2],
					token1: triRoute.poolRoute[2].token2,
					token2: triRoute.poolRoute[2].token1,
				},
				{
					...triRoute.poolRoute[1],
					token1: triRoute.poolRoute[1].token2,
					token2: triRoute.poolRoute[1].token1,
				},
				{
					...triRoute.poolRoute[0],
					token1: triRoute.poolRoute[0].token2,
					token2: triRoute.poolRoute[0].token1,
				},
			],
		}
	}
}
export default RouteManager
