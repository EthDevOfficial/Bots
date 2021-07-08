import Web3 from 'web3'
import PolygonLoader from './loading/polygon'
import abiDecoder from 'abi-decoder'
import { Chain, Exchange, ForkType, Router, Token } from './types'
import FantomLoader from './loading/fantom'
import XDaiLoader from './loading/xdai'
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
			return { key: exchange.router.address.toLowerCase(), val: exchange }
		})
		.reduce((map, obj) => {
			map[obj.key] = obj.val
			return map
		}, {})

	const routerForkTypeMap = routers
		.map((router) => {
			return { key: router.address.toLowerCase(), val: router.forkType }
		})
		.reduce((map, obj) => {
			map[obj.key] = obj.val
			return map
		}, {})

	const tokenMap = tokens
		.map((token) => {
			return { key: token.address.toLowerCase(), val: token }
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
		preferedTokens: preferedTokenObjs.map((token) => token.address.toLowerCase()),
		ignoreTokens: ignoreTokenCased.map((address) => address.toLowerCase()),
		hotAddress: '0x6ebaA58e9C60A4E6d2ebEcb2d55b119eC1DE8D42',
		hotPrivateKey: '8cf317cf854ba12a9283f41b4c8dd214d60deaac35888121a046b002beb0b0be',
		...loader,
	}
}

export default load
