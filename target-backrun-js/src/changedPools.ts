import BigNumber from 'bignumber.js'
import { MAX_GAS_PRICE } from './constants'
import { exists, makeAddressKey } from './general'
import { abiDecoder, routers, routerExchangeMap, preferedTokens, tokenMap, ignoreTokens } from './main'
import Token from './token'
import { Address, Exchange, ForkType, Pool } from './types'

async function getChangedPools(tx) {
	let pools: Pool[] = []

	if (exists(tx.to)) {
		const gasPrice = new BigNumber(tx.gasPrice)
		if (gasPrice.lt(MAX_GAS_PRICE)) {
			const exchange: Exchange = routerExchangeMap[tx.to]
			if (exists(exchange)) {
				pools.push(...uniswapRouter(tx, exchange, gasPrice))
			}
		}
	}
	return pools
}

function uniswapRouter(tx: any, exchange: Exchange, gasPrice: BigNumber) {
	const pools: Pool[] = []
	const input = abiDecoder.decodeMethod(tx.input)
	if (exists(input) && exists(input.name.includes('swap'))) {
		let tokenPath = []
		let aboveThreshold = false
		if (input.name === 'swapExactTokensForTokens' || input.name === 'swapExactTokensForETH') {
			tokenPath = input.params[2].value
			aboveThreshold = isTradeAboveThreshold(tokenPath[0], input.params[0].value) || isTradeAboveThreshold(tokenPath[tokenPath.length - 1], input.params[1].value)
		} else if (input.name === 'swapExactETHForTokens' || input.name === 'swapETHForExactTokens') {
			tokenPath = input.params[1].value
			aboveThreshold = isTradeAboveThreshold(tokenPath[tokenPath.length - 1], input.params[0].value) || isTradeAboveThreshold(tokenPath[0], tx.value)
		} else if (input.name === 'swapTokensForExactTokens' || input.name === 'swapTokensForExactETH') {
			tokenPath = input.params[2].value
			aboveThreshold = isTradeAboveThreshold(tokenPath[tokenPath.length - 1], input.params[0].value) || isTradeAboveThreshold(tokenPath[0], input.params[1].value)
		} else {
			// console.log(`missed all: ${input.name}`)
		}
		if (aboveThreshold) {
			for (let i = 0; i < tokenPath.length - 1; i++) {
				const token1: Address = tokenPath[i]
				const token2: Address = tokenPath[i + 1]
				if (useChangedPool(token1, token2)) {
					const pool: Pool = {
						token1: token1,
						t1Key: makeAddressKey(token1),
						token2: token2,
						t2Key: makeAddressKey(token2),
						exchange,
						gasPrice: gasPrice,
					}
					pools.push(pool)
				}
			}
		}
	}
	return pools
}

function isTradeAboveThreshold(token: Address, amount: string) {
	const tokenObj: Token | undefined = tokenMap[makeAddressKey(token)]
	if (exists(tokenObj) && preferedTokens.includes(tokenObj!.key)) {
		const amountBN = new BigNumber(amount)
		return amountBN.gt(tokenObj!.tradeThreshold!)
	}
	return false
}

function useChangedPool(token1: Address, token2: Address) {
	const token1Key = makeAddressKey(token1)
	const token2Key = makeAddressKey(token2)
	// const hasIgnoreToken = ignoreTokens.includes(token1Key) || ignoreTokens.includes(token2Key)
	return preferedTokens.includes(token1Key) || preferedTokens.includes(token2Key)
}

export default getChangedPools
