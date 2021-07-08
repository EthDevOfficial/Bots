import Web3 from 'web3'
import { OPP_EXEC_CONTRACT } from '../constants'
import Token from '../token'
import { Address, ForkType } from '../types'
const abis = require('../abis')

/**
 * Loads all the Polygon specific data
 * @param {Web3} web3
 * @returns {LoadDetails}
 */
function PolygonLoader(web3: Web3) {
	const routers = {
		quickswap: { address: '0xa5E0829CaCEd8fFDD4De3c43696c57F7D7A678ff', forkType: ForkType.Uniswap },
		sushiswap: { address: '0x1b02dA8Cb0d097eB8D57A175b88c7D8b47997506', forkType: ForkType.Uniswap },
		dfyn: { address: '0xA102072A4C07F06EC3B4900FDC4C7B80b6c57429', forkType: ForkType.Uniswap },
		apeswap: { address: '0xC0788A3aD43d79aa53B09c2EaCc313A787d1d607', forkType: ForkType.Uniswap },
		paraswap: { address: '0x90249ed4d69D70E709fFCd8beE2c5A566f65dADE', forkType: ForkType.Paraswap },
	}

	const tokens = {
		quick: new Token('quick', '0x831753DD7087CaC61aB5644b308642cc1c33Dc13', 18),
		wmatic: new Token('wmatic', '0x0d500B1d8E8eF31E21C99d1Db9A6444d3ADf1270', 18, '1000000000000000000000'),
		weth: new Token('weth', '0x7ceB23fD6bC0adD59E62ac25578270cFf1b9f619', 18, '500000000000000000'),
		wbtc: new Token('wbtc', '0x1bfd67037b42cf73acf2047067bd4f2c47d9bfd6', 8),
		usdt: new Token('usdt', '0xc2132D05D31c914a87C6611C10748AEb04B58e8F', 6, '1000000000'),
		dai: new Token('dai', '0x8f3Cf7ad23Cd3CaDbD9735AFf958023239c6A063', 18),
		usdc: new Token('usdc', '0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174', 6, '1000000000'),
		pup: new Token('pup', '0xcfe2cf35d2bdde84967e67d00ad74237e234ce59', 18),
		dfyn: new Token('dfyn', '0xC168E40227E4ebD8C1caE80F7a55a4F0e6D66C97', 18),
		bone: new Token('bone', '0x6bb45cEAC714c52342Ef73ec663479da35934bf7', 18),
		route: new Token('route', '0x16ECCfDbb4eE1A85A33f3A9B21175Cd7Ae753dB4', 18),
		titan: new Token('titan', '0xaAa5B9e6c589642f98a1cDA99B9D024B8407285A', 18),
	}

	const exchanges = {
		quickswap: {
			id: 'quickswap',
			router: routers.quickswap,
			swapFee: 30,
		},
		sushiswap: {
			id: 'sushiswap',
			router: routers.sushiswap,
			swapFee: 30,
		},
		dfyn: {
			id: 'dfyn',
			router: routers.dfyn,
			swapFee: 30,
		},
		apeswap: {
			id: 'apeswap',
			router: routers.apeswap,
			swapFee: 20,
		},
	}

	const preferedTokens: Token[] = [tokens.weth, tokens.wmatic, tokens.usdc]
	const outerTokens: Token[] = [tokens.weth, tokens.wmatic]
	const innerTokens: Token[] = [tokens.bone, tokens.titan, tokens.usdc, tokens.pup]
	const ignoreTokens: Address[] = ['0x3a3df212b7aa91aa0402b9035b098891d276572b', '0x8a953cfe442c5e8855cc6c61b1293fa648bae472', '0x84259e4c4207Ec8F2e6DB22Ba30d283180baCdB5', '0xaaa5b9e6c589642f98a1cda99b9d024b8407285a', '0xeFb3009DdAc87E8144803d78E235E7fb4cd36e61', '0x05089C9EBFFa4F0AcA269e32056b1b36B37ED71b', '0x3AEF8512Fb6D4231beB786Ef75086951E3ae6362', '0x8f18dC399594b451EdA8c5da02d0563c0b2d0f16', '0x0e59D50adD2d90f5111aca875baE0a72D95B4762']

	const optimizerExecAddress = OPP_EXEC_CONTRACT
	const optimizerExec = new web3.eth.Contract(abis.optimizerExec.abi, optimizerExecAddress)

	web3.eth.defaultCommon = { customChain: { name: 'MATIC', chainId: 137, networkId: 137 }, baseChain: 'mainnet', hardfork: 'istanbul' }

	return {
		exchangeObj: exchanges,
		optimizerExec,
		optimizerExecAddress,
		stdToken: tokens.wmatic,
		stdExchange: exchanges.quickswap,
		preferedTokenObjs: preferedTokens,
		outerTokens,
		innerTokens,
		tokenObj: tokens,
		ignoreTokenCased: ignoreTokens,
		routerObj: routers,
	}
}

export default PolygonLoader
