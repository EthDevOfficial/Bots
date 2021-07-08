import BigNumber from 'bignumber.js'

export enum Chain {
	Mainnet = 0,
	XDai = 1,
	Polygon = 2,
	Fantom = 3,
}

export enum ArbType {
	Simple = 0,
	Tri = 1,
}

export type Address = string
export type ID = string

export type Exchange = {
	id: ID
	router: Address
	swapFee: number
}

export type Token = {
	id: ID
	address: Address
	decimals: number
}

export type Pool = {
	token1: Token
	token2: Token
	exchange: Exchange
}

export type Route = {
	poolRoute: Pool[]
}

export type RouteBundle = {
	type: ArbType
	routes: Route[]
	encodedRoutes?: any //
}

export type Wallet = {
	account: any
	id: number
}
