import Web3 from 'web3'
import { OPP_EXEC_CONTRACT } from '../constants'
import { Address, ForkType, Token } from '../types'
const abis = require('../abis')

/**
 * Loads all the Fantom specific data
 * @param {Web3} web3
 * @returns {LoadDetails}
 */
function FantomLoader(web3: Web3) {
	const routers = {
		spirit: { address: '0x16327E3FbDaCA3bcF7E38F5Af2599D2DDc33aE52', forkType: ForkType.Uniswap },
		spooky: { address: '0xF491e7B69E4244ad4002BC14e878a34207E38c29', forkType: ForkType.Uniswap },
		sushi: { address: '0x1b02dA8Cb0d097eB8D57A175b88c7D8b47997506', forkType: ForkType.Uniswap },
	}

	const tokens = {
		wftm: { id: 'wftm', address: '0x21be370D5312f44cB42ce377BC9b8a0cEF1A4C83', decimals: 18 },
		weth: { id: 'weth', address: '0x74b23882a30290451A17c44f4F05243b6b58C76d', decimals: 18 },
		usdc: { id: 'usdc', address: '0x04068DA6C83AFCFA0e13ba15A6696662335D5B75', decimals: 6 },
	}

	const exchanges = {
		spirit: {
			id: 'spirit',
			router: routers.spirit,
			swapFee: 997,
		},
		spooky: {
			id: 'spooky',
			router: routers.spooky,
			swapFee: 998,
		},
		sushi: {
			id: 'sushi',
			router: routers.sushi,
			swapFee: 997,
		},
	}

	const preferedTokens: Token[] = [tokens.weth, tokens.wftm, tokens.usdc]
	const outerTokens = [tokens.weth, tokens.wftm]
	const ignoreTokens: Address[] = ['0xbAc5d43A56696e5D0CB631609E85798f564b513b', '0x34e54e32dbe8835990789c9d4a426c2a0cbd70fb', '0x9Ba3e4F84a34DF4e08C112e1a0FF148b81655615', '0x6038C2B554A9e5E65E4cDC4625D840352E21B1C0', '0x9Ba3e4F84a34DF4e08C112e1a0FF148b81655615', '0x181F3F22C9a751E2ce673498A03E1FDFC0ebBFB6', '0x84466a3df84b5008ef4252952e3319f2cae2dbac', '0x2241fabdf45c54ca330c34c783b114e6bcaae7e7'].map((address) => address.toLowerCase())

	const optimizerExecAddress = OPP_EXEC_CONTRACT
	const optimizerExec = new web3.eth.Contract(abis.optimizerExec.abi, optimizerExecAddress)

	return {
		exchangeObj: exchanges,
		optimizerExec,
		optimizerExecAddress,
		stdToken: tokens.wftm,
		stdExchange: exchanges.spooky,
		preferedTokenObjs: preferedTokens,
		outerTokens,
		tokenObj: tokens,
		ignoreTokenCased: ignoreTokens,
		routerObj: routers,
	}
}

export default FantomLoader
