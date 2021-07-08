// import Web3 from 'web3'
// import { makeTokenKey } from '../helpers/makeKeys'
// import { Address, Exchange, ForkType, Token } from '../types'
// const abis = require('../abis')

// /**
//  * Loads all the XDai specific data
//  * @param {Web3} web3
//  * @returns {LoadDetails}
//  */
// function XDaiLoader(web3: Web3) {
// 	const stdToken: Token = {
// 		id: 'wxdai',
// 		address: '0xe91D153E0b41518A2Ce8Dd3D7944Fa863463a97d',
// 		key: makeTokenKey('0xe91D153E0b41518A2Ce8Dd3D7944Fa863463a97d'),
// 	}

// 	const exchanges = {
// 		baoswap: {
// 			id: 'baoswap',
// 			factory: '0x45DE240fbE2077dd3e711299538A09854FAE9c9b',
// 			forkType: ForkType.Uniswap,
// 			router: '0x6093AeBAC87d62b1A5a4cEec91204e35020E38bE',
// 		},
// 		honeyswap: {
// 			id: 'honeyswap',
// 			factory: '0xA818b4F111Ccac7AA31D0BCc0806d64F2E0737D7',
// 			forkType: ForkType.Uniswap,
// 			router: '0x1C232F01118CB8B424793ae03F870aa7D0ac7f77',
// 		},
// 		// DXSWAP: {
// 		// 	id: 'DXSWAP',
// 		// 	factory: '0x5D48C95AdfFD4B40c1AAADc4e08fc44117E02179',
// 		// 	forkType: ForkType.Uniswap,
// 		// 	router: '0xE43e60736b1cb4a75ad25240E2f9a62Bff65c0C0',
// 		// },
// 	}
// 	const stdExchange: Exchange = {
// 		id: 'honeyswap',
// 		factory: '0xA818b4F111Ccac7AA31D0BCc0806d64F2E0737D7',
// 		forkType: ForkType.Uniswap,
// 		router: '0x1C232F01118CB8B424793ae03F870aa7D0ac7f77',
// 	}

// 	const preferredOuterToken: Token = stdToken
// 	const outerTokens = {
// 		wxdai: '0xe91D153E0b41518A2Ce8Dd3D7944Fa863463a97d',
// 	}

// 	// Ordered by num transactions on network
// 	const innerLiquidTokens = {
// 		hny: '0x71850b7E9Ee3f13Ab46d67167341E4bDc905Eef9',
// 		agve: '0x3a97704a1b25F08aa230ae53B352e2e72ef52843',
// 		// haus: '0xb0C5f3100A4d9d9532a4CfD68c55F1AE8da987Eb',
// 		stake: '0xb7D311E2Eb55F2f68a9440da38e7989210b9A05e',
// 		// bao: '0x82dFe19164729949fD66Da1a37BC70dD6c4746ce',
// 		usdc: '0xDDAfbb505ad214D7b80b1f830fcCc89B60fb7A83',
// 		weth: '0x6A023CCd1ff6F2045C3309768eAd9E68F978f6e1',
// 		// xmoon: '0x1e16aa4Df73d29C029d94CeDa3e3114EC191E25A',
// 		link: '0xE2e73A1c69ecF83F464EFCE6A5be353a37cA09b2',
// 		levin: '0x1698cD22278ef6E7c0DF45a8dEA72EDbeA9E42aa',
// 		// prtcl: '0xB5d592f85ab2D955c25720EbE6FF8D4d1E1Be300',
// 		wbtc: '0x8e5bBbb09Ed1ebdE8674Cda39A0c169401db4252',
// 		cold: '0xdbcadE285846131a5e7384685EADDBDFD9625557',
// 		miva: '0x63e62989D9EB2d37dfDB1F93A22f063635b07d51',
// 		dai: '0x44fA8E6f47987339850636F88629646662444217',
// 		// xbrick: '0x2f9ceBf5De3bc25E0643D0E66134E5bf5c48e191',
// 		// donut: '0x524B969793a64a602342d89BC2789D43a016B13A',
// 		aave: '0xDF613aF6B44a31299E48131e9347F034347E2F00',
// 		// hex: '0xd9Fa47e33d4Ff7a1ACA489DE1865ac36c042B07a',
// 		// uncx: '0x0116e28B43A358162B96f70B4De14C98A4465f25',
// 		// '1inch': '0x7f7440C5098462f833E123B44B8A03E1d9785BAb',
// 		// graph: '0xFAdc59D012Ba3c110B08A15B7755A5cb7Cbe77D7',
// 		// matic: '0x7122d7661c4564b7C6Cd4878B06766489a6028A2',
// 		// alvin: '0x50DBde932A94b0c23D27cdd30Fbc6B987610c831',
// 		// comb: '0x906Df587eF029dF1727Ece063a877aEd000A878a',
// 		// comp: '0xDf6FF92bfDC1e8bE45177DC1f4845d391D3ad8fD',
// 		// dpi: '0xD3D47d5578e55C880505dC40648F7F9307C3e7A8',
// 		// bal: '0x7eF541E2a22058048904fE5744f9c7E4C57AF717',
// 		// ewtb: '0x6A8cb6714B1EE5b471a7D2eC4302cb4f5Ff25eC2',
// 		// rsr: '0x5A87eaC5642BfEd4e354Ee8738DACd298E07D1Af',
// 		// mew: '0xDA623B1bC7eA3edbBC69AE229334e6D46a8aa307',
// 		// trini: '0x4ACCF43c858ac40555fE1ddf5806BcbC1612a4C3',
// 		// dot: '0xe51Cf68301B30b06c72d38f5314FA85c94C8e5f1',
// 	}

// 	const ignorePoolWithTokens: Address[] = ['0x4609e9b9c2912dd5b954cbf3a5d7d89ab6c8979d', '0x43bf77e8c21b0A57774fedD90Ca8791B58C457D1', '0xec07b6E321014B3093101C8296944a7C56354B3f'].map((address) => address.toLowerCase())

// 	const optimizerExecAddress = '0xd5D016E8a7B56801F1Faa52C5bE9eE238339a10B'
// 	const optimizerExec = new web3.eth.Contract(abis.optimizerExec.abi, optimizerExecAddress)

// 	return {
// 		stdToken,
// 		exchanges,
// 		innerLiquidTokens,
// 		ignorePoolWithTokens,
// 		stdExchange,
// 		optimizerExec,
// 		optimizerExecAddress,
// 		outerTokens,
// 		preferredOuterToken,
// 		subHotAddress: '0x6ebaA58e9C60A4E6d2ebEcb2d55b119eC1DE8D42',
// 		subHotPrivateKey: '8cf317cf854ba12a9283f41b4c8dd214d60deaac35888121a046b002beb0b0be',
// 	}
// }

// export default XDaiLoader
