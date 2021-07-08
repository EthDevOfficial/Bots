import BigNumber from 'bignumber.js'
import { pathExistsSync, ensureFileSync, writeJsonSync, readJsonSync } from 'fs-extra'
import Web3 from 'web3'
import { DEFAULT_GP_WL_GWEI, GEN_NEW_WALLETS, NUM_WALLETS, ONE_GWEI_BN, WALLETS_PATH, WALLET_BALANCE } from './constants'
import { stdToken } from './main'
import { Address, Wallet } from './types'

class WalletLoader {
	private web3: Web3
	private hotAddress: Address
	private hotPrivateKey: string
	private wallets: Wallet[] = []

	constructor(web3: Web3, hotAddress: Address, hotPrivateKey: string) {
		this.web3 = web3
		this.hotAddress = hotAddress
		this.hotPrivateKey = hotPrivateKey
	}

	async loadWallets() {
		// See if we need to generate new wallets
		if (GEN_NEW_WALLETS) {
			try {
				// If there's an existing path to a wallet file proceed with pulling
				if (pathExistsSync(WALLETS_PATH!)) {
					// Read the wallets from the file and instantiate the accounts
					const previousWallets = readJsonSync(WALLETS_PATH!).map((wallet) => {
						return {
							...wallet,
							account: this.web3.eth.accounts.wallet.add(wallet.account.privateKey),
						}
					})
					await this.pullFromWallets(previousWallets)
				} else {
					console.log('No Wallets To Pull From')
				}
			} catch {}

			ensureFileSync(WALLETS_PATH!)

			// Create the wallets and send them money
			// Write them back to the file upon loading each
			await this.sendToWallets()
		}

		// Read all the wallets from the file and return them
		return readJsonSync(WALLETS_PATH!).map((wallet) => {
			return {
				...wallet,
				account: this.web3.eth.accounts.wallet.add(wallet.account.privateKey),
			}
		})
	}

	private async pullFromWallets(prevWallets: Wallet[]) {
		let amountInWeiPulled: BigNumber = new BigNumber(0)
		let numWalletsPulledFrom = 0
		await Promise.all(
			prevWallets.map(async (wallet) => {
				const balanceInWei = new BigNumber(await this.web3.eth.getBalance(wallet.account.address))
				const gasPrice = ONE_GWEI_BN.multipliedBy(DEFAULT_GP_WL_GWEI)
				const minBalance = gasPrice.multipliedBy(60000)
				if (balanceInWei.gt(minBalance)) {
					const details = {
						from: wallet.account.address,
						to: this.hotAddress,
						value: balanceInWei.minus(minBalance).toString(),
						gas: 60000,
						gasPrice: gasPrice.toString(),
					}
					try {
						const createTx = await this.web3.eth.accounts.signTransaction(details, wallet.account.privateKey)
						await this.web3.eth.sendSignedTransaction(createTx.rawTransaction!)
						amountInWeiPulled = amountInWeiPulled.plus(balanceInWei.minus(minBalance))
						numWalletsPulledFrom++
					} catch (err) {
						console.log(err)
					}
				}
			})
		)
		console.log(`Pulled ${amountInWeiPulled.shiftedBy(-stdToken.decimals)} From ${numWalletsPulledFrom} Wallets`)
	}

	private async sendToWallets() {
		// const batchSize = 30
		// const numBatches = Math.floor(NUM_WALLETS / 30)
		// const rangeOfWallets = [...Array(batchSize).keys()]
		// let walletId = 0
		// const nonce = await this.web3.eth.getTransactionCount(this.hotAddress)

		for (let i = 0; i < NUM_WALLETS; i++) {
			const wallet = this.web3.eth.accounts.create()

			const details = {
				from: this.hotAddress,
				to: wallet.address,
				value: new BigNumber(WALLET_BALANCE).shiftedBy(stdToken.decimals).toString(),
				gas: 60000,
				gasPrice: ONE_GWEI_BN.multipliedBy(DEFAULT_GP_WL_GWEI).toString(),
			}

			try {
				const createTx = await this.web3.eth.accounts.signTransaction(details, this.hotPrivateKey)
				await this.web3.eth.sendSignedTransaction(createTx.rawTransaction!)

				this.wallets.push({
					account: this.web3.eth.accounts.wallet.add(wallet.privateKey),
					id: i,
				})

				console.log(`[${i}] ${wallet.address} LOADED with BALANCE ${WALLET_BALANCE}`)
				writeJsonSync(WALLETS_PATH!, this.wallets)
			} catch (err) {
				console.log(`WALLET LOAD FAILED FOR WALLET ${i}`)
				console.log('Trying again @ higher gwei')
				details.gasPrice = new BigNumber(details.gasPrice).multipliedBy(1.2).toString()
				try {
					const createTx = await this.web3.eth.accounts.signTransaction(details, this.hotPrivateKey)
					await this.web3.eth.sendSignedTransaction(createTx.rawTransaction!)

					this.wallets.push({
						account: this.web3.eth.accounts.wallet.add(wallet.privateKey),
						id: i,
					})

					console.log(`[${i}] ${wallet.address} LOADED with BALANCE ${WALLET_BALANCE}`)
					writeJsonSync(WALLETS_PATH!, this.wallets)
				} catch {
					console.log(`WALLET LOAD FAILED FOR WALLET ${i} AGAIN`)
				}
			}
		}
	}
}

export default WalletLoader
