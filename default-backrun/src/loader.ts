import Web3 from 'web3'
// import MainnetLoader from './loading/mainnet'
import PolygonLoader from './loading/polygon'
// import XDaiLoader from './loading/xdai'
import FantomLoader from './loading/fantom'
import abiDecoder from 'abi-decoder'
import { Chain } from './types'
const path = require('path')
const abis = require('./abis')
require('dotenv').config({ path: path.resolve(__dirname, '../.env') })

const loaders = {
	// [Chain.Mainnet]: MainnetLoader,
	// [Chain.XDai]: XDaiLoader,
	[Chain.Polygon]: PolygonLoader,
	[Chain.Fantom]: FantomLoader,
}

function load() {
	const chain: Chain = Number(process.env.CHAIN!)
	const web3 = new Web3(new Web3.providers.WebsocketProvider(process.env.NODE_WS!))
	const web3Provider = web3.eth.currentProvider as any

	abiDecoder.addABI(abis.optimizerExec.abi)

	return {
		chain,
		web3,
		web3Provider,
		abiDecoder,
		hotAddress: '0x6ebaA58e9C60A4E6d2ebEcb2d55b119eC1DE8D42',
		hotPrivateKey: '8cf317cf854ba12a9283f41b4c8dd214d60deaac35888121a046b002beb0b0be',
		...loaders[chain](web3),
	}
}

export default load
