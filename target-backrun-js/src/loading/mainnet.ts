// import Web3 from 'web3'
// import { makeTokenKey } from '../helpers/makeKeys'
// import { Address, Exchange, Exchanges, ForkType, Token } from '../types'
// const abis = require('../abis')

// function MainnetLoader(web3: Web3) {
// 	const stdToken = {
// 		id: 'wxdai',
// 		address: 'address here',
// 		key: '',
// 	}
// 	const preferredOuterToken: Token = stdToken

// 	const liquidTokens = {
// 		weth: {
// 			id: 'weth',
// 			address: 'address here',
// 		},
// 		hny: {
// 			id: 'hny',
// 			address: 'address here',
// 		},
// 		wxdai: {
// 			id: 'wxdai',
// 			address: 'address here',
// 		},
// 	}

// 	const exchanges: Exchanges = {
// 		uniswap: {
// 			id: 'uniswap',
// 			factory: 'address here',
// 			forkType: ForkType.Uniswap,
// 			router: 'address here',
// 		},
// 		sushiswap: {
// 			id: 'sushiswap',
// 			factory: 'address here',
// 			forkType: ForkType.Uniswap,
// 			router: 'address here',
// 		},
// 	}
// 	const stdExchange: Exchange = {
// 		id: 'uniswap',
// 		factory: 'address here',
// 		forkType: ForkType.Uniswap,
// 		router: 'address here',
// 	}

// 	const outerTokens = {
// 		token: 'as',
// 	}
// 	const innerLiquidTokens = {
// 		token: 'address',
// 	}
// 	const ignorePoolWithTokens: Address[] = []

// 	return {
// 		stdToken,
// 		liquidTokens,
// 		exchanges,
// 		innerLiquidTokens,
// 		ignorePoolWithTokens,
// 		stdExchange,
// 		optimizerExec: new web3.eth.Contract(abis.optimizerExec.abi, ''),
// 		optimizerExecAddress: '',
// 		outerTokens,
// 		preferredOuterToken,
// 		subHotAddress: '',
// 		subHotPrivateKey: '',
// 	}
// }

// export default MainnetLoader
