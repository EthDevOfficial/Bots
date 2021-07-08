import { Address, Chain } from './types'
import { LOG_LENGTH } from './constants'

import { chain, hotAddress } from './main'

export function exists(obj: any) {
	return obj ? true : false
}

export function timestamp() {
	const date = new Date()
	const _hour = date.getHours()
	const hour = _hour >= 5 ? _hour - 5 : 19 + _hour
	const minutes = date.getMinutes().toString()
	const seconds = date.getSeconds().toString()
	return `${hour}:${minutes.length > 1 ? minutes : '0' + minutes}:${seconds.length > 1 ? seconds : '0' + seconds}`
}

export function timestampWithMS() {
	const date = new Date()
	const _hour = date.getHours()
	const hour = _hour >= 5 ? _hour - 5 : 19 + _hour
	const minutes = date.getMinutes().toString()
	const seconds = date.getSeconds().toString()
	return `${hour}:${minutes.length > 1 ? minutes : '0' + minutes}:${seconds.length > 1 ? seconds : '0' + seconds} ${date.getMilliseconds()} ms`
}

export function getChainName() {
	switch (chain) {
		case Chain.Mainnet:
			return 'Mainnet'
		case Chain.XDai:
			return 'XDai'
		case Chain.Polygon:
			return 'Polygon'
		case Chain.Fantom:
			return 'Fantom'
	}
}

export function logNewBlock(blockNumber: number) {
	console.log(`====== New Block: ${blockNumber} | Time: ${timestamp()} ======`)
}

export function dateAndTime() {
	// (month - day - year)
	const date = new Date()
	const _hour = date.getHours()
	const hour = _hour >= 5 ? _hour - 5 : 19 + _hour
	const minutes = date.getMinutes().toString()
	const seconds = date.getSeconds().toString()
	return {
		date: `${date.getMonth() + 1}/${date.getDate()}/${date.getFullYear().toString().slice(2, 4)}`,
		time: `${hour}:${minutes.length > 1 ? minutes : '0' + minutes}:${seconds.length > 1 ? seconds : '0' + seconds}`,
	}
}

export function removeTrailingZeros(num: string): string {
	const lastIndex = num.length - 1
	const last = num[lastIndex]
	if (last === '0') {
		return removeTrailingZeros(num.slice(0, lastIndex))
	} else if (last === '.') {
		return num.slice(0, lastIndex)
	} else {
		return num
	}
}

const SHORTER_LENGTH = LOG_LENGTH - 10
export function makeRow(left: string, right: string, padString = '=') {
	return left + ' ' + (' ' + right).padStart(SHORTER_LENGTH - left.length, padString)
}

export function makeGFRow(left: string, right: string, padString = '=') {
	return left + right.padStart(SHORTER_LENGTH - left.length, padString)
}

export function makeAddressKey(address: Address) {
	return address.toLowerCase()
}
