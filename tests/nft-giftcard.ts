import * as anchor from "@project-serum/anchor"
import * as spl from "@solana/spl-token"
import { Program } from "@project-serum/anchor"
import { NftGiftcard } from "../target/types/nft_giftcard"
const fs = require("fs")

describe("nft-giftcard", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env())
  const connection = anchor.getProvider().connection
  const wallet = anchor.workspace.NftGiftcard.provider.wallet
  const program = anchor.workspace.NftGiftcard as Program<NftGiftcard>

  let mint: anchor.web3.PublicKey
  let paymentDestination: anchor.web3.PublicKey

  before(async () => {
    let rawdata = fs.readFileSync(
      "tests/test-key/test-WaoKNLQVDyBx388CfjaVeyNbs3MT2mPgAhoCfXyUvg8.json"
    )
    let json = JSON.parse(rawdata)
    let keypair = anchor.web3.Keypair.fromSecretKey(new Uint8Array(json))
    console.log(keypair)

    mint = await spl.createMint(
      connection,
      wallet.payer,
      wallet.publicKey,
      null,
      0,
      keypair
    )

    paymentDestination = await spl.createAccount(
      connection,
      wallet.payer,
      mint,
      wallet.publicKey
    )
  })

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
      .initialize()
      .accounts({ paymentDestination: paymentDestination })
      .rpc()
    console.log("Your transaction signature", tx)
  })
})
