import Web3 from 'web3'
import { OPP_EXEC_CONTRACT } from '../constants'
import { Token } from '../types'
const abis = require('../abis')

/**
 * Loads all the Fantom specific data
 * @param {Web3} web3
 * @returns {LoadDetails}
 */
function FantomLoader(web3: Web3) {
	const tokens = {
		wftm: { id: 'wftm', address: '0x21be370D5312f44cB42ce377BC9b8a0cEF1A4C83', decimals: 18 },
		weth: { id: 'weth', address: '0x74b23882a30290451A17c44f4F05243b6b58C76d', decimals: 18 },
		usdc: { id: 'usdc', address: '0x04068DA6C83AFCFA0e13ba15A6696662335D5B75', decimals: 6 },
	}

	const exchanges = {
		spirit: {
			id: 'spirit',
			router: '0x16327E3FbDaCA3bcF7E38F5Af2599D2DDc33aE52',
			swapFee: 997,
		},
		spooky: {
			id: 'spooky',
			router: '0xF491e7B69E4244ad4002BC14e878a34207E38c29',
			swapFee: 998,
		},
		// sushi: {
		// 	id: 'sushi',
		// 	router: '0x1b02dA8Cb0d097eB8D57A175b88c7D8b47997506',
		// 	swapFee: 997,
		// },
	}

	const tokenPaths: Token[][] = [
		[tokens.wftm, tokens.usdc],
		[tokens.weth, tokens.usdc],
	]

	const optimizerExecAddress = OPP_EXEC_CONTRACT
	const optimizerExec = new web3.eth.Contract(abis.optimizerExec.abi, optimizerExecAddress)

	return {
		exchanges: Object.keys(exchanges).map((key) => exchanges[key]),
		optimizerExec,
		optimizerExecAddress,
		tokenPaths,
		stdToken: tokens.wftm,
		stdExchange: exchanges.spooky,
	}
}

export default FantomLoader
