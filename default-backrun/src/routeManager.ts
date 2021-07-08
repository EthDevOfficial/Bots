import { BACKRUN_GWEIS, MIN_GAS_LIMIT, RUN_TRIS, SIMPLE_GAS_ESTIMATE, TRANSACTION_BOUNDER, TRI_GAS_ESTIMATE } from './constants'
import { exists, timestampWithMS } from './general'
import { abiDecoder, exchanges, optimizerExec, optimizerExecAddress, stdExchange, wallets, web3 } from './main'
import { encode } from './encoders'
import { ArbType, Exchange, Pool, Route, RouteBundle, Token, Wallet } from './types'
import BigNumber from 'bignumber.js'

class RouteManager {
	routeBundles: RouteBundle[] = []

	constructor(tokenPaths: Token[][]) {
		tokenPaths.forEach((tokenPath) => {
			if (tokenPath.length === 2) {
				exchanges.forEach((exchange) => {
					if (exchange.id !== stdExchange.id) {
						const pool1 = this.buildPool(tokenPath[0], tokenPath[1], stdExchange)
						const pool2 = this.buildPool(tokenPath[1], tokenPath[0], exchange)
						const route = { poolRoute: [pool1, pool2] }
						const bundle: RouteBundle = { type: ArbType.Simple, routes: [route, this.reverseSimpleRoute(route)] }
						this.routeBundles.push(bundle)
					}
				})
			} else if (RUN_TRIS) {
				const pool1 = this.buildPool(tokenPath[0], tokenPath[1], stdExchange)
				const pool2 = this.buildPool(tokenPath[1], tokenPath[2], stdExchange)
				const pool3 = this.buildPool(tokenPath[2], tokenPath[1], stdExchange)
				const route = { poolRoute: [pool1, pool2, pool3] }
				const triBundle: RouteBundle = { type: ArbType.Tri, routes: [route, this.reverseTriRoute(route)] }
				this.routeBundles.push(triBundle)
			}
		})
		this.routeBundles = this.routeBundles.map((route) => {
			return { ...route, encodedRoutes: encode(route) }
		})
	}

	bidOnAllRoutes() {
		for (const gasPrice of BACKRUN_GWEIS) {
			let _routeBundles = this.routeBundles.map((route) => route)
			while (_routeBundles.length > 0) {
				let wallet = wallets.shift()
				if (!exists(wallet)) console.log('Wallets are out of gas')
				this.bidOnBundle(_routeBundles.shift()!, wallet!, gasPrice)
				wallets.push(wallet!)
			}
		}
	}

	private bidOnBundle(routeBundle: RouteBundle, wallet: Wallet, gasPrice: BigNumber) {
		const timeSent = timestampWithMS()
		const tx = routeBundle.type === ArbType.Simple ? optimizerExec.methods.simpleMulticall(routeBundle.encodedRoutes) : optimizerExec.methods.triMulticall(routeBundle.encodedRoutes)
		const txData = {
			from: wallet.account,
			to: optimizerExecAddress,
			gas: this.calculateGasEstimate(routeBundle),
			gasPrice: gasPrice.toString(),
			data: tx.encodeABI(),
		}

		web3.eth
			.sendTransaction(txData)
			.then((receipt) => this.logArb(routeBundle, receipt, timeSent))
			.catch((err) => this.handleError(err.message, wallet))
	}

	private logArb(routeBundle: RouteBundle, receipt: any, timeSent: string) {
		let logs = abiDecoder.decodeLogs(receipt.logs)
		logs.forEach((element) => {
			if (element.events.length > 0) {
				element.events.forEach((event) => {
					if (event.name === 'diff') {
						const diff = new BigNumber(event.value)
						const token1 = routeBundle.routes[0].poolRoute[0].token1
						const token2 = routeBundle.routes[0].poolRoute[1].token1

						console.log(TRANSACTION_BOUNDER)
						console.log(`REVENUE:  ${diff.shiftedBy(-token1.decimals).toString()}`)
						console.log(`SENT AT:    ${timeSent}`)
						console.log(`TAKEN AT:  ${timestampWithMS}`)

						if (routeBundle.type === ArbType.Simple) {
							console.log(`${token1.id} --> ${token2.id}`)
						} else {
							const token3 = routeBundle.routes[0].poolRoute[2].token1
							console.log(`${token1.id} --> ${token2.id} --> ${token3.id}`)
						}
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
		return oneRouteGE.multipliedBy(routeBundle.routes.length).lt(MIN_GAS_LIMIT) ? MIN_GAS_LIMIT.toString() : oneRouteGE.multipliedBy(routeBundle.routes.length).toString()
	}

	/*==========================================CONSTRUCTION HELPERS===================================================*/

	private buildPool(token1: Token, token2: Token, exchange: Exchange): Pool {
		return {
			token1: token1,
			token2: token2,
			exchange,
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
