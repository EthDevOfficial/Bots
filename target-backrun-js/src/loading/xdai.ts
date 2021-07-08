import Web3 from 'web3'
import { OPP_EXEC_CONTRACT } from '../constants'
import Token from '../token'
import { Address, ForkType } from '../types'
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
		swapper: { address: '0xE43e60736b1cb4a75ad25240E2f9a62Bff65c0C0', ForkType: ForkType.Uniswap },
	}

	const tokens = {
		wxdai: new Token('wxdai', '0xe91D153E0b41518A2Ce8Dd3D7944Fa863463a97d', 18, '10000000000000000000'),
		weth: new Token('weth', '0x6A023CCd1ff6F2045C3309768eAd9E68F978f6e1', 18, '5000000000000000'),
		hny: new Token('hny', '0x71850b7E9Ee3f13Ab46d67167341E4bDc905Eef9', 18),
		agve: new Token('agve', '0x3a97704a1b25F08aa230ae53B352e2e72ef52843', 18),
		usdc: new Token('usdc', '0xDDAfbb505ad214D7b80b1f830fcCc89B60fb7A83', 6),
		stake: new Token('stake', '0xb7D311E2Eb55F2f68a9440da38e7989210b9A05e', 18),
	}

	const exchanges = {
		baoswap: {
			id: 'baoswap',
			router: routers.baoswap,
			swapFee: 30,
		},
		honeyswap: {
			id: 'honeyswap',
			router: routers.honeyswap,
			swapFee: 30,
		},
		swapper: {
			id: 'swapper',
			router: routers.swapper,
			swapFee: 25,
		},
	}

	const preferedTokens: Token[] = [tokens.weth, tokens.wxdai, tokens.hny]
	const outerTokens: Token[] = [tokens.weth, tokens.wxdai]
	const innerTokens: Token[] = [tokens.hny, tokens.agve, tokens.stake, tokens.weth]
	const ignoreTokens: Address[] = ['0x4609e9b9c2912dd5b954cbf3a5d7d89ab6c8979d', '0x43bf77e8c21b0A57774fedD90Ca8791B58C457D1', '0xec07b6E321014B3093101C8296944a7C56354B3f']

	const optimizerExecAddress = OPP_EXEC_CONTRACT
	const optimizerExec = new web3.eth.Contract(abis.optimizerExec.abi, optimizerExecAddress)

	web3.eth.defaultCommon = { customChain: { name: 'XDAI', chainId: 100, networkId: 100 }, baseChain: 'mainnet', hardfork: 'istanbul' }

	return {
		exchangeObj: exchanges,
		optimizerExec,
		optimizerExecAddress,
		stdToken: tokens.wxdai,
		stdExchange: exchanges.honeyswap,
		preferedTokenObjs: preferedTokens,
		outerTokens,
		innerTokens,
		tokenObj: tokens,
		ignoreTokenCased: ignoreTokens,
		routerObj: routers,
	}
}

export default XDaiLoader
