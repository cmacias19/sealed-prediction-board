# Sealed Prediction Board

**A tamper-proof "I called it" machine. Lock in a secret prediction on Solana now, and prove later that you made it first.**

You write a prediction, and only a scrambled fingerprint of it (a SHA-256 hash) goes on the blockchain with a timestamp, so nobody can read it yet. Later you reveal the text, and the program checks it matches the fingerprint exactly. That proves you made the prediction before the event, and nobody can edit or backdate it.

A screenshot or a tweet can be faked or deleted. The blockchain gives a public timestamp nobody controls.

Built on Solana devnet with Anchor in [Solana Playground](https://beta.solpg.io).

## How it works

1. **Seal (`commit`)**: the client hashes `sha256(salt || prediction)`, where the salt is `sha256(secret phrase)`, and stores only that hash with the current time in an account owned by the program.
2. **Reveal (`reveal`)**: the author sends the prediction text and the salt. The program recomputes the hash on-chain and only accepts the reveal if it matches, then stores the text and the reveal time.
3. **Board**: a web page reads every prediction from devnet, shows sealed ones as envelopes and flips them over live when they're revealed.

The secret phrase stops people from guessing short predictions ("Team X wins") by trying candidates until one matches the fingerprint.

## Files

| File | What it is |
|---|---|
| `src/lib.rs` | The Anchor program: `commit` and `reveal` instructions and the `Prediction` account |
| `client/client.ts` | Solana Playground script to seal, reveal or show a prediction |
| `board.html` | The live prediction board; open it in any browser |

## Run it

1. Open [beta.solpg.io](https://beta.solpg.io), create an **Anchor (Rust)** project and paste `src/lib.rs` into `src/lib.rs`.
2. Connect a wallet, get devnet SOL from [faucet.solana.com](https://faucet.solana.com), then **Build** and **Deploy**.
3. Paste `client/client.ts` into `client/client.ts`, set `PREDICTION` and `SECRET`, and type `run` in the terminal to seal it.
4. Open `board.html`, paste the program ID from `declare_id!(...)` and click **Show board**.
5. Later, set `MODE = "reveal"` in the client and run it again. The envelope flips on the board.

Keep `PREDICTION` and `SECRET` identical between sealing and revealing, and don't use a real password as the secret: it becomes public when you reveal.
