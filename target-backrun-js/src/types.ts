import BigNumber from 'bignumber.js'

export enum Chain {
	Mainnet = 0,
	XDai = 1,
	Polygon = 2,
	Fantom = 3,
}

export enum ForkType {
	Default = 0,
	Uniswap = 1,
	Paraswap = 2,
}

export enum ArbType {
	Simple = 0,
	Tri = 1,
}

export type Address = string
export type ID = string

export type Router = {
	address: Address
	forkType: ForkType
}

export type Exchange = {
	id: ID
	router: Router
	swapFee: number
}

export type RouteBundle = {
	type: ArbType
	routes: Route[]
	gasPrice: BigNumber
	encodedRoutes: any
}

export type Wallet = {
	account: any
	id: number
	nonce: number
}

export type Route = {
	type: ArbType
	pools: Pool[]
}

export type Pool = {
	token1: Address
	t1Key: string
	token2: Address
	t2Key: string
	exchange: Exchange
	gasPrice?: BigNumber
}
