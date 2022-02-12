import {
    Keypair,
    Connection,
    PublicKey,
    LAMPORTS_PER_SOL,
    SystemProgram,
    TransactionInstruction,
    Transaction,
    sendAndConfirmTransaction,
} from '@solana/web3.js';
import * as fs from 'fs';

(async () => {
    const connection = new Connection('https://api.devnet.solana.com', 'confirmed');
    const receiverPublicKey = new PublicKey("9EYyX1rckbjhqhphngFHpnV1fUGfuCaCUobyZibWE1tX");
    const programKeypair = await createKeypairFromFile("./dist/program/transfersolana-keypair.json")
    const programId = programKeypair.publicKey;
    const lamportsAmount = 100;

    const payer = Keypair.generate();
    await connection.requestAirdrop(
        payer.publicKey,
        1 * LAMPORTS_PER_SOL,
    );
    const instruction = new TransactionInstruction({
        keys: [
            { pubkey: payer.publicKey, isSigner: true, isWritable: true },
            { pubkey: receiverPublicKey, isSigner: false, isWritable: true },
            { pubkey: SystemProgram.programId, isSigner: false, isWritable: false }
        ],
        programId,
        data: Buffer.from([0, lamportsAmount]), // All instructions are hellos
    });
    const txId = await sendAndConfirmTransaction(
        connection,
        new Transaction().add(instruction),
        [payer],
    );
    console.log("Transaction ID: ", txId);
})();

async function createKeypairFromFile(
    filePath: string,
): Promise<Keypair> {
    const secretKeyString = fs.readFileSync(filePath, 'utf-8',);
    const secretKey = Uint8Array.from(JSON.parse(secretKeyString));
    return Keypair.fromSecretKey(secretKey);
}
