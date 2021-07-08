import { web3 } from './main'
import { SimpleBackrun, TriBackrun } from './constants'
import { ArbType, Route } from './types'

export function encode(routesInBundle: Route[], type: ArbType) {
	return type === ArbType.Simple ? simpleBackrunEncoder(routesInBundle) : triBackrunEncoder(routesInBundle)
}

function simpleBackrunEncoder(simpleRoutes: Route[]) {
	return simpleRoutes.map(({ pools }) => {
		const paramsToEncode = {
			outerToken: pools[0].t1Key,
			innerToken: pools[1].t1Key,
			exchange1: pools[0].exchange.router.address,
			exchange2: pools[1].exchange.router.address,
			swapFeeSum: (pools[0].exchange.swapFee + pools[1].exchange.swapFee).toFixed(0),
		}
		return web3.eth.abi.encodeParameter(SimpleBackrun, paramsToEncode)
	})
}

function triBackrunEncoder(triRoutes: Route[]) {
	return triRoutes.map(({ pools }) => {
		const paramsToEncode = {
			token1: pools[0].t1Key,
			token2: pools[1].t1Key,
			token3: pools[2].t1Key,
			exchange1: pools[0].exchange.router.address,
			exchange2: pools[1].exchange.router.address,
			exchange3: pools[2].exchange.router.address,
			swapFeeSum: (pools[0].exchange.swapFee + pools[1].exchange.swapFee + pools[2].exchange.swapFee).toFixed(0),
		}
		return web3.eth.abi.encodeParameter(TriBackrun, paramsToEncode)
	})
}
