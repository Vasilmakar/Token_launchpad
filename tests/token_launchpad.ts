import * as anchor from "@coral-xyz/anchor";
import { Program, web3, BN  } from "@coral-xyz/anchor";
import { TokenLaunchpad } from "../target/types/token_launchpad";
import { getAssociatedTokenAddress, createAssociatedTokenAccountInstruction } from "@solana/spl-token";
import { PublicKey } from "@solana/web3.js";
describe("token_launchpad", () => {
  // Configure the client to use the local cluster.
  let provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.tokenLaunchpad as Program<TokenLaunchpad>;

  it("Is initialized!", async () => {
    const TOKEN_METADATA_PROGRAM_ID = new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");

    const payer = provider.wallet.publicKey;

// PDA mint-authority (той самий сид, що в програмі)
    const [mintAuthPda] = PublicKey.findProgramAddressSync(
        [Buffer.from("mint-auth"), payer.toBuffer()],
        program.programId
    );

// Створимо Keypair для mint у клієнта або довіримо init через Anchor (вище ми робимо init в інструкції — тож mint буде створений on-chain).
    const mintKp = web3.Keypair.generate();

// Recipient
    const recipient = payer; // або інший Pubkey
    const recipientAta = await getAssociatedTokenAddress(mintKp.publicKey, recipient);

// Metadata PDA
    const [metadataPda] = PublicKey.findProgramAddressSync(
        [Buffer.from("metadata"), TOKEN_METADATA_PROGRAM_ID.toBuffer(), mintKp.publicKey.toBuffer()],
        TOKEN_METADATA_PROGRAM_ID
    );

    await program.methods
        .createCustomMint({
          name: "My Token",
          symbol: "MYT",
          uri: "https://arweave.net/your_metadata_json", // обов'язково валідний JSON за стандартом Metaplex
          decimals: 9,
          initialSupply: new BN(1_000_000_000_000), // приклад для 1_000 токенів з 9 decimals
          freezeAuthority: true,
          recipientIsPayer: true,
        })
        .accounts({
          payer,
          mintAuthorityPda: mintAuthPda,
          mint: mintKp.publicKey,
          recipient,
          recipientAta,
          metadata: metadataPda,
          tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
          associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
          systemProgram: web3.SystemProgram.programId,
          tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
          rent: web3.SYSVAR_RENT_PUBKEY,
        })
        .signers([mintKp]) // бо ми init'имо mint акаунт
        .rpc();
  });
})
