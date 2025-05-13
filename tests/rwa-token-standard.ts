import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { RwaTokenStandard } from "../target/types/rwa_token_standard";
import { getTokenMetadata } from "@solana/spl-token";

describe("rwa-token-standard", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.AnchorProvider.env();

  const program = anchor.workspace.rwaTokenStandard as Program<RwaTokenStandard>;

  const authority = new anchor.web3.Keypair();

  const mint = new anchor.web3.Keypair();

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });

  it("Asset Initialized", async () => {
    const signature1 = await provider.connection.requestAirdrop(
      authority.publicKey,
      2 * anchor.web3.LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(signature1, "confirmed");

    const tx = await program.methods
      .createAsset({ name: "as", symbol: "asds", uri: "pssd", delegate: "" })
      .accounts({
        authority: authority.publicKey,
        mint: mint.publicKey,
      })
      .signers([mint, authority])
      .rpc();
    const chainMetadata = await getTokenMetadata(provider.connection, mint.publicKey);
    console.log(chainMetadata);
  });
});
