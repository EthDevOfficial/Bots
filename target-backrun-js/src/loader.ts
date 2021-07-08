import Web3 from 'web3'
import PolygonLoader from './loading/polygon'
import abiDecoder from 'abi-decoder'
import { Chain, Exchange, Router } from './types'
import FantomLoader from './loading/fantom'
import XDaiLoader from './loading/xdai'
import { makeAddressKey } from './general'
import Token from './token'
import { bidSimpleNoChi, bidSimpleChi, bidTriNoChi, bidTriChi } from './bundleBidder'
const path = require('path')
const abis = require('./abis')
require('dotenv').config({ path: path.resolve(__dirname, '../.env') })

const loaders = {
	// [Chain.Mainnet]: MainnetLoader,
	[Chain.XDai]: XDaiLoader,
	[Chain.Polygon]: PolygonLoader,
	[Chain.Fantom]: FantomLoader,
}

function load() {
	const chain: Chain = Number(process.env.CHAIN!)
	const web3 = new Web3(new Web3.providers.WebsocketProvider(process.env.NODE_WS!))
	const web3Provider = web3.eth.currentProvider as any

	// load chain specific things
	const loader = loaders[chain](web3)

	// set more things off chain specific things
	const { exchangeObj, routerObj, preferedTokenObjs, ignoreTokenCased, tokenObj } = loader
	const exchanges: Exchange[] = Object.keys(exchangeObj).map((key) => exchangeObj[key])
	const tokens: Token[] = Object.keys(tokenObj).map((key) => tokenObj[key])
	const routers: Router[] = Object.keys(routerObj).map((key) => routerObj[key])

	const routerExchangeMap = exchanges
		.map((exchange) => {
			return { key: exchange.router.address, val: exchange }
		})
		.reduce((map, obj) => {
			map[obj.key] = obj.val
			return map
		}, {})

	const routerForkTypeMap = routers
		.map((router) => {
			return { key: router.address, val: router.forkType }
		})
		.reduce((map, obj) => {
			map[obj.key] = obj.val
			return map
		}, {})

	const tokenMap = tokens
		.map((token) => {
			return { key: makeAddressKey(token.address), val: token }
		})
		.reduce((map, obj) => {
			map[obj.key] = obj.val
			return map
		}, {})

	abiDecoder.addABI(abis.uniswapRouter)
	abiDecoder.addABI(abis.paraswapRouter)
	abiDecoder.addABI(abis.optimizerExec.abi)

	return {
		chain,
		web3,
		web3Provider,
		abiDecoder,
		routers,
		exchanges,
		routerExchangeMap,
		routerForkTypeMap,
		tokenMap,
		preferedTokens: preferedTokenObjs.map((token) => token.key),
		ignoreTokens: ignoreTokenCased.map((address) => makeAddressKey(address)),
		hotAddress: '0x6ebaA58e9C60A4E6d2ebEcb2d55b119eC1DE8D42',
		hotPrivateKey: '8cf317cf854ba12a9283f41b4c8dd214d60deaac35888121a046b002beb0b0be',
		bidSimple: chain === Chain.XDai ? bidSimpleNoChi : bidSimpleChi,
		bidTri: chain === Chain.XDai ? bidTriNoChi : bidTriChi,
		...loader,
	}
}

export default load
