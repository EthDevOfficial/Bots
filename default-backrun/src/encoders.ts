import { web3 } from './main'
import { SimpleBackrun, TriBackrun } from './constants'
import { ArbType, Route, RouteBundle } from './types'

export function encode(routeBundle: RouteBundle) {
	return routeBundle.type === ArbType.Simple ? simpleBackrunEncoder(routeBundle.routes) : triBackrunEncoder(routeBundle.routes)
}

function simpleBackrunEncoder(simpleRoutes: Route[]) {
	return simpleRoutes.map(({ poolRoute }) => {
		const paramsToEncode = {
			outerToken: poolRoute[0].token1.address,
			innerToken: poolRoute[1].token1.address,
			exchange1: poolRoute[0].exchange.router,
			exchange2: poolRoute[1].exchange.router,
			swapFee1: poolRoute[0].exchange.swapFee,
			swapFee2: poolRoute[1].exchange.swapFee,
		}
		return web3.eth.abi.encodeParameter(SimpleBackrun, paramsToEncode)
	})
}

function triBackrunEncoder(triRoutes: Route[]) {
	return triRoutes.map(({ poolRoute }) => {
		const paramsToEncode = {
			token1: poolRoute[0].token1.address,
			token2: poolRoute[1].token1.address,
			token3: poolRoute[2].token1.address,
			exchange1: poolRoute[0].exchange.router,
			exchange2: poolRoute[1].exchange.router,
			exchange3: poolRoute[2].exchange.router,
			swapFee1: poolRoute[0].exchange.swapFee,
			swapFee2: poolRoute[1].exchange.swapFee,
			swapFee3: poolRoute[2].exchange.swapFee,
		}
		return web3.eth.abi.encodeParameter(TriBackrun, paramsToEncode)
	})
}
