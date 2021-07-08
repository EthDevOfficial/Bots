import Web3 from 'web3'
import { OPP_EXEC_CONTRACT } from '../constants'
import { Address, ForkType, Token } from '../types'
const abis = require('../abis')

/**
 * Loads all the XDai specific data
 * @param {Web3} web3
 * @returns {LoadDetails}
 */
function XDaiLoader(web3: Web3) {
	const routers = {
		baoswap: { address: '0x6093AeBAC87d62b1A5a4cEec91204e35020E38bE', forkType: ForkType.Uniswap },
		honeyswap: { address: '0x1C232F01118CB8B424793ae03F870aa7D0ac7f77', forkType: ForkType.Uniswap },
	}

	const tokens = {
		wxdai: { id: 'wxdai', address: '0xe91D153E0b41518A2Ce8Dd3D7944Fa863463a97d', decimals: 18 },
		weth: { id: 'weth', address: '0x6A023CCd1ff6F2045C3309768eAd9E68F978f6e1', decimals: 18 },
	}

	const exchanges = {
		baoswap: {
			id: 'baoswap',
			router: routers.baoswap,
			swapFee: 997,
		},
		honeyswap: {
			id: 'honeyswap',
			router: routers.honeyswap,
			swapFee: 997,
		},
		// DXSWAP: {
		// 	id: 'DXSWAP',
		// 	factory: '0x5D48C95AdfFD4B40c1AAADc4e08fc44117E02179',
		// 	forkType: ForkType.Uniswap,
		// 	router: '0xE43e60736b1cb4a75ad25240E2f9a62Bff65c0C0',
		// },
	}

	const preferedTokens: Token[] = [tokens.weth, tokens.wxdai]
	const outerTokens: Token[] = [tokens.weth, tokens.wxdai]
	const ignoreTokens: Address[] = ['0x4609e9b9c2912dd5b954cbf3a5d7d89ab6c8979d', '0x43bf77e8c21b0A57774fedD90Ca8791B58C457D1', '0xec07b6E321014B3093101C8296944a7C56354B3f']

	const optimizerExecAddress = OPP_EXEC_CONTRACT
	const optimizerExec = new web3.eth.Contract(abis.optimizerExec.abi, optimizerExecAddress)

	return {
		exchangeObj: exchanges,
		optimizerExec,
		optimizerExecAddress,
		stdToken: tokens.wxdai,
		stdExchange: exchanges.honeyswap,
		preferedTokenObjs: preferedTokens,
		outerTokens,
		tokenObj: tokens,
		ignoreTokenCased: ignoreTokens,
		routerObj: routers,
	}
}

export default XDaiLoader
