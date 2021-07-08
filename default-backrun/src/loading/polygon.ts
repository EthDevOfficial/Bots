import Web3 from 'web3'
import { OPP_EXEC_CONTRACT } from '../constants'
import { Token } from '../types'
const abis = require('../abis')

/**
 * Loads all the Polygon specific data
 * @param {Web3} web3
 * @returns {LoadDetails}
 */
function PolygonLoader(web3: Web3) {
	const exchanges = {
		quickswap: {
			id: 'quickswap',
			router: '0xa5E0829CaCEd8fFDD4De3c43696c57F7D7A678ff',
			swapFee: 997,
		},
		sushiswap: {
			id: 'sushiswap',
			router: '0x1b02dA8Cb0d097eB8D57A175b88c7D8b47997506',
			swapFee: 997,
		},
	}

	const tokens = {
		quick: { id: 'quick', address: '0x831753DD7087CaC61aB5644b308642cc1c33Dc13', decimals: 18 },
		wmatic: { id: 'wmatic', address: '0x0d500B1d8E8eF31E21C99d1Db9A6444d3ADf1270', decimals: 18 },
		weth: { id: 'weth', address: '0x7ceB23fD6bC0adD59E62ac25578270cFf1b9f619', decimals: 18 },
		usdt: { id: 'usdt', address: '0xc2132D05D31c914a87C6611C10748AEb04B58e8F', decimals: 6 },
		dai: { id: 'dai', address: '0x8f3Cf7ad23Cd3CaDbD9735AFf958023239c6A063', decimals: 18 },
		usdc: { id: 'usdc', address: '0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174', decimals: 6 },
	}

	const tokenPaths: Token[][] = [
		[tokens.usdc, tokens.wmatic],
		[tokens.usdc, tokens.weth],
	]

	const optimizerExecAddress = OPP_EXEC_CONTRACT
	const optimizerExec = new web3.eth.Contract(abis.optimizerExec.abi, optimizerExecAddress)

	return {
		exchanges: Object.keys(exchanges).map((key) => exchanges[key]),
		optimizerExec,
		optimizerExecAddress,
		tokenPaths,
		stdToken: tokens.wmatic,
		stdExchange: exchanges.quickswap,
	}
}

export default PolygonLoader
