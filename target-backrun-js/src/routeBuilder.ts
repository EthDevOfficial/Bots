import { exchanges, innerTokens, outerTokens, preferedTokens, stdExchange } from './main'
import Token from './token'
import { ArbType, Exchange, Pool, Route } from './types'

/* ============================ SimpleBuilders ========================= */

export function getSimpleRoutes(changedPools: Pool[]): Route[] {
	const simpleRoutes: Route[] = []
	for (const changedPool of changedPools) {
		for (const awayExchange of exchanges) {
			if (awayExchange.id !== changedPool.exchange.id) {
				simpleRoutes.push(buildSimpleRoute(changedPool, awayExchange))
			}
		}
	}
	return simpleRoutes
}

// changedPool -> awayPool
function buildSimpleRoute(changedPool: Pool, awayExchange: Exchange): Route {
	const shouldSwap = shouldSwapPoolOrder(changedPool)
	changedPool = shouldSwap ? swapTokensInPool(changedPool) : changedPool

	const route = {
		type: ArbType.Simple,
		pools: [
			changedPool,
			{
				token1: changedPool.token2,
				t1Key: changedPool.t2Key,
				token2: changedPool.token1,
				t2Key: changedPool.t1Key,
				exchange: awayExchange,
			},
		],
	}

	return shouldSwap ? route : reverseSimple(route)
}

/* ============================ TriBuilders ============================ */

export function getTriRoutes(changedPools: Pool[]): Route[] {
	const triRoutes: Route[] = []
	for (const changedPool of changedPools) {
		outerTokens.forEach((outerToken: Token) => {
			if (outerToken.key !== changedPool.t1Key && outerToken.key !== changedPool.t2Key) {
				triRoutes.push(buildInnerTriRoute(changedPool, outerToken))
			}
		})
		// if (outerTokens.includes(changedPool.t1Key) || outerTokens.includes(changedPool.t2Key)) {
		innerTokens.forEach((innerToken: Token) => {
			if (innerToken.key !== changedPool.t1Key && innerToken.key !== changedPool.t2Key) {
				triRoutes.push(buildOuterTriRoute(changedPool, innerToken))
			}
		})
		// }
	}
	return triRoutes
}

// outerPool -> changedPool -> outerPool
export function buildInnerTriRoute(changedPool: Pool, outerToken: Token): Route {
	const shouldSwap = shouldSwapPoolOrder(changedPool)
	changedPool = shouldSwap ? swapTokensInPool(changedPool) : changedPool

	const route = {
		type: ArbType.Tri,
		pools: [
			{
				token1: outerToken.address,
				t1Key: outerToken.key,
				token2: changedPool.token1,
				t2Key: changedPool.t1Key,
				exchange: changedPool.exchange,
			},
			changedPool,
			{
				token1: changedPool.token2,
				t1Key: changedPool.t2Key,
				token2: outerToken.address,
				t2Key: outerToken.key,
				exchange: changedPool.exchange,
			},
		],
	}

	return shouldSwap ? route : reverseTri(route)
}

// changedPool -> innerPool -> outerPool
export function buildOuterTriRoute(changedPool: Pool, innerToken: Token): Route {
	const shouldSwap = shouldSwapPoolOrder(changedPool)
	changedPool = shouldSwap ? swapTokensInPool(changedPool) : changedPool

	const route = {
		type: ArbType.Tri,
		pools: [
			changedPool,
			{
				token1: changedPool.token2,
				t1Key: changedPool.t2Key,
				token2: innerToken.address,
				t2Key: innerToken.key,
				exchange: changedPool.exchange,
			},
			{
				token1: innerToken.address,
				t1Key: innerToken.key,
				token2: changedPool.token1,
				t2Key: changedPool.t1Key,
				exchange: changedPool.exchange,
			},
		],
	}

	return shouldSwap ? route : reverseTri(route)
}

/* ============================ Helpers ============================ */

function swapTokensInPool(pool: Pool): Pool {
	return {
		...pool,
		token1: pool.token2,
		t1Key: pool.t2Key,
		token2: pool.token1,
		t2Key: pool.t1Key,
	}
}

function shouldSwapPoolOrder(pool: Pool): boolean {
	return preferedTokens.includes(pool.t2Key)
}

function reverseSimple(route: Route): Route {
	return {
		...route,
		pools: [swapTokensInPool(route.pools[1]), swapTokensInPool(route.pools[0])],
	}
}

function reverseTri(route: Route): Route {
	return {
		...route,
		pools: [swapTokensInPool(route.pools[2]), swapTokensInPool(route.pools[1]), swapTokensInPool(route.pools[0])],
	}
}
