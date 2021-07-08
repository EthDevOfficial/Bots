import Web3 from 'web3'
import { OPP_EXEC_CONTRACT } from '../constants'
import Token from '../token'
import { Address, ForkType } from '../types'
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
		wftm: new Token('wftm', '0x21be370D5312f44cB42ce377BC9b8a0cEF1A4C83', 18, '100000000000000000000'),
		weth: new Token('weth', '0x74b23882a30290451A17c44f4F05243b6b58C76d', 18, '50000000000000000'),
		usdc: new Token('usdc', '0x04068DA6C83AFCFA0e13ba15A6696662335D5B75', 6, '100000000'),
		fusd: new Token('fusd', '0xAd84341756Bf337f5a0164515b1f6F993D194E1f', 18),
		fbtc: new Token('fbtc', '0xe1146b9AC456fCbB60644c36Fd3F868A9072fc6E', 18),
		aave: new Token('aave', '0x6a07A792ab2965C72a5B8088d3a069A7aC3a993B', 18),
		dai: new Token('dai', '0x8D11eC38a3EB5E956B052f67Da8Bdc9bef8Abf3E', 18),
		curve: new Token('curve', '0x1E4F97b9f9F913c46F1632781732927B9019C68b', 18),
	}

	const exchanges = {
		spirit: {
			id: 'spirit',
			router: routers.spirit,
			swapFee: 30,
		},
		spooky: {
			id: 'spooky',
			router: routers.spooky,
			swapFee: 20,
		},
		sushi: {
			id: 'sushi',
			router: routers.sushi,
			swapFee: 30,
		},
	}

	const preferedTokens: Token[] = [tokens.weth, tokens.wftm, tokens.usdc]
	const outerTokens = [tokens.weth, tokens.wftm]
	const innerTokens = [tokens.fusd, tokens.fbtc, tokens.aave, tokens.curve]
	const ignoreTokens: Address[] = ['0xbAc5d43A56696e5D0CB631609E85798f564b513b', '0x34e54e32dbe8835990789c9d4a426c2a0cbd70fb', '0x9Ba3e4F84a34DF4e08C112e1a0FF148b81655615', '0x6038C2B554A9e5E65E4cDC4625D840352E21B1C0', '0x9Ba3e4F84a34DF4e08C112e1a0FF148b81655615', '0x181F3F22C9a751E2ce673498A03E1FDFC0ebBFB6', '0x84466a3df84b5008ef4252952e3319f2cae2dbac', '0x2241fabdf45c54ca330c34c783b114e6bcaae7e7'].map((address) => address.toLowerCase())

	const optimizerExecAddress = OPP_EXEC_CONTRACT
	const optimizerExec = new web3.eth.Contract(abis.optimizerExec.abi, optimizerExecAddress)

	web3.eth.defaultCommon = { customChain: { name: 'FANTOM', chainId: 250, networkId: 250 }, baseChain: 'mainnet', hardfork: 'istanbul' }

	return {
		exchangeObj: exchanges,
		optimizerExec,
		optimizerExecAddress,
		stdToken: tokens.wftm,
		stdExchange: exchanges.spooky,
		preferedTokenObjs: preferedTokens,
		outerTokens,
		innerTokens,
		tokenObj: tokens,
		ignoreTokenCased: ignoreTokens,
		routerObj: routers,
	}
}

export default FantomLoader
