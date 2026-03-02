import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { ScoutChain } from "../target/types/scout_chain";
import { PublicKey, Keypair, SystemProgram, LAMPORTS_PER_SOL, Transaction } from "@solana/web3.js";
import { expect } from "chai";

describe("scout_chain_barca_tests", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.ScoutChain as Program<ScoutChain>;

  const playerAuth = Keypair.generate();
  const scoutAuth = Keypair.generate();

  let playerPda: PublicKey;
  let scoutPda: PublicKey;

  const logTx = (signature: string) => {
    console.log(`   Transaction: https://explorer.solana.com/tx/${signature}?cluster=devnet`);
  };

  before(async () => {
    const transferTx = new Transaction().add(
      SystemProgram.transfer({
        fromPubkey: provider.wallet.publicKey,
        toPubkey: playerAuth.publicKey,
        lamports: 0.05 * LAMPORTS_PER_SOL,
      }),
      SystemProgram.transfer({
        fromPubkey: provider.wallet.publicKey,
        toPubkey: scoutAuth.publicKey,
        lamports: 0.05 * LAMPORTS_PER_SOL,
      })
    );

    const sig = await provider.sendAndConfirm(transferTx);
    console.log(`Setup: Wallets funded.`);
    logTx(sig);

    [playerPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("player"), playerAuth.publicKey.toBuffer()],
      program.programId
    );
    [scoutPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("scout"), scoutAuth.publicKey.toBuffer()],
      program.programId
    );
  });

  it("Registers Pedri as a player", async () => {
    const sig = await program.methods
      .createPlayer("Pedri González", "Midfielder", 21, "92% Pass", "https://fcb.com/pedri")
      .accounts({
        player: playerPda,
        authority: playerAuth.publicKey,
        systemProgram: SystemProgram.programId,
      } as any)
      .signers([playerAuth])
      .rpc();

    console.log(`Player Created: ${playerPda.toBase58()}`);
    logTx(sig);
  });

  it("Registers Xavi as a scout", async () => {
    const sig = await program.methods
      .createScout("Xavi Hernández", "FC Barcelona", 25)
      .accounts({
        scout: scoutPda,
        authority: scoutAuth.publicKey,
        systemProgram: SystemProgram.programId,
      } as any)
      .signers([scoutAuth])
      .rpc();

    console.log(`Scout Created: ${scoutPda.toBase58()}`);
    logTx(sig);
  });

  it("Scout reviews Pedri", async () => {
    const playerAccount = await program.account.player.fetch(playerPda);
    const countBuffer = Buffer.alloc(4);
    countBuffer.writeUInt32LE(playerAccount.reviewCount);

    const [reviewPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("review"), playerPda.toBuffer(), scoutPda.toBuffer(), countBuffer],
      program.programId
    );

    const sig = await program.methods
      .createReview(5, "Genius.")
      .accounts({
        player: playerPda,
        scout: scoutPda,
        review: reviewPda,
        authority: scoutAuth.publicKey,
        systemProgram: SystemProgram.programId,
      } as any)
      .signers([scoutAuth])
      .rpc();

    console.log(`Review Created: ${reviewPda.toBase58()}`);
    logTx(sig);
  });

  it("Updates Pedri's stats", async () => {
    const sig = await program.methods
      .updatePlayer("Pedri González", "Captain", 22, "95% Pass", "https://fcb.com/pedri")
      .accounts({
        player: playerPda,
        authority: playerAuth.publicKey,
      } as any)
      .signers([playerAuth])
      .rpc();

    console.log(`Player Updated: Pedri is now Captain`);
    logTx(sig);
  });

  it("Deactivates scout", async () => {
    const sig = await program.methods
      .deactivateScout()
      .accounts({
        scout: scoutPda,
        authority: scoutAuth.publicKey,
      } as any)
      .signers([scoutAuth])
      .rpc();

    console.log(`Scout Deactivated: Xavi`);
    logTx(sig);
  });

  it("Removes the review record", async () => {
    const countBuffer = Buffer.alloc(4);
    countBuffer.writeUInt32LE(0); // Assuming it's the first review

    const [reviewPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("review"), playerPda.toBuffer(), scoutPda.toBuffer(), countBuffer],
      program.programId
    );

    const sig = await program.methods
      .deleteReview()
      .accounts({
        review: reviewPda,
        authority: scoutAuth.publicKey,
      } as any)
      .signers([scoutAuth])
      .rpc();

    console.log(`Review Deleted: Account closed and SOL recovered.`);
    logTx(sig);
  });
});