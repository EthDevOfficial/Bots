import BigNumber from 'bignumber.js'
import { exists, makeAddressKey } from './general'
import { Address } from './types'

class Token {
	id: string
	address: Address
	key: string
	decimals: number
	tradeThreshold?: BigNumber

	constructor(id: string, address: Address, decimals: number, tradeThreshold: string | undefined = undefined) {
		this.id = id
		this.address = address
		this.key = makeAddressKey(address)
		this.decimals = decimals

		if (exists(tradeThreshold)) {
			this.tradeThreshold = new BigNumber(tradeThreshold!)
		}
	}

	static compare(token1: Token, token2: Token) {
		return token1.key === token2.key
	}
}
export default Token
