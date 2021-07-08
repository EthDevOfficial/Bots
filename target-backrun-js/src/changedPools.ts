import BigNumber from 'bignumber.js'
import { MAX_GAS_PRICE } from './constants'
import { exists } from './general'
import { abiDecoder, routers, ignoreTokens, preferedTokens, tokenMap, routerExchangeMap, routerForkTypeMap, stdExchange, chain } from './main'
import { Address, Chain, Exchange, ForkType, Pool, Token } from './types'

async function getChangedPools(tx) {
	let pools: Pool[] = []

	if (exists(tx) && exists(tx.to) && exists(tx.gasPrice)) {
		for (const router of routers) {
			const txRouter: Address = tx.to.toLowerCase()
			if (txRouter === router.address.toLowerCase()) {
				switch (routerForkTypeMap[txRouter]) {
					case ForkType.Uniswap:
						pools.push(...uniswapRouter(tx, txRouter))
						break
					case ForkType.Paraswap:
						pools.push(...paraswapRouter(tx))
						break
				}
			}
		}
	}
	pools = pools.filter((pool) => !ignoreTokens.includes(pool.token1.address.toLowerCase()) && !ignoreTokens.includes(pool.token2.address.toLowerCase()))
	return pools
}

function paraswapRouter(tx: any) {
	const pools: Pool[] = []
	const input = abiDecoder.decodeMethod(tx.input)
	if (exists(input) && exists(input.name.toLowerCase().includes('swap'))) {
		for (let param of input.params) {
			if (param.name === 'path') {
				const tokenPath: any[] = param.value
				for (let i = 0; i < tokenPath.length - 1; i++) {
					const token1: Address = tokenPath[i]
					const token2: Address = tokenPath[i + 1]
					const token2isOuter = preferedTokens.includes(token2.toLowerCase())
					const _token1 = token2isOuter ? token2 : token1
					const _token2 = token2isOuter ? token1 : token2
					const gasPrice = new BigNumber(tx.gasPrice)
					const changedPool = {
						token1: buildToken(_token1),
						token2: buildToken(_token2),
						exchange: stdExchange,
						swapped: token2isOuter,
						gasPrice: gasPrice.gt(MAX_GAS_PRICE) ? MAX_GAS_PRICE : gasPrice,
						isParaswap: true,
					}

					pools.push(changedPool)
				}
			}
		}
	}
	return pools
}

function uniswapRouter(tx: any, txRouter: Address) {
	const pools: Pool[] = []
	const input = abiDecoder.decodeMethod(tx.input)
	if (exists(input) && exists(input.name.toLowerCase().includes('swap'))) {
		const exchange: Exchange = routerExchangeMap[txRouter]
		for (let param of input.params) {
			if (param.name === 'path') {
				const tokenPath: any[] = param.value
				for (let i = 0; i < tokenPath.length - 1; i++) {
					const token1: Address = tokenPath[i]
					const token2: Address = tokenPath[i + 1]
					const token2isOuter = preferedTokens.includes(token2.toLowerCase())
					const _token1 = token2isOuter ? token2 : token1
					const _token2 = token2isOuter ? token1 : token2
					const gasPrice = new BigNumber(tx.gasPrice)
					const changedPool = {
						token1: buildToken(_token1),
						token2: buildToken(_token2),
						exchange,
						swapped: token2isOuter,
						gasPrice: gasPrice.gt(MAX_GAS_PRICE) ? MAX_GAS_PRICE : gasPrice,
						isParaswap: false,
					}

					pools.push(changedPool)
				}
			}
		}
	}
	return pools
}

function buildToken(tokenAddress: Address): Token {
	return {
		id: 'other',
		address: tokenAddress,
		decimals: 18,
	}
}

export default getChangedPools
