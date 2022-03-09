import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { TestDapps } from '../target/types/test_dapps';
import * as assert from "assert";
import {Keypair, PublicKey, SystemProgram} from '@solana/web3.js'

describe('test-dapps', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());
  let _baseAccount;
  const program = anchor.workspace.TestDapps as Program<TestDapps>;
  const user = Keypair.generate();

  it('Airdrop', async () => {
    await airdrop(user.publicKey, 'user');
  });

  it("Creates a counter)", async () => {
    /* Call the create function via RPC */
    const baseAccount = anchor.web3.Keypair.generate();
    await program.rpc.create({
      accounts: {
        baseAccount: baseAccount.publicKey,
        user: user.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [baseAccount, user],
    });

    /* Fetch the account and check the value of count */
    const account = await program.account.baseAccount.fetch(baseAccount.publicKey);
    console.log('Count 0: ', account.count.toString())
    assert.ok(account.count.toString() == 0);
    _baseAccount = baseAccount;

  });

  it("Increments the counter", async () => {
    const baseAccount = _baseAccount;

    await program.rpc.increment({
      accounts: {
        baseAccount: baseAccount.publicKey,
      },
    });

    const account = await program.account.baseAccount.fetch(baseAccount.publicKey);
    console.log('Count 1: ', account.count.toString())
    assert.ok(account.count.toString() == 1);
  });
});


export async function airdrop(pubkey: PublicKey, logTag: string): Promise<void> {
  const conn = anchor.getProvider().connection;
  const sig = await conn.requestAirdrop(pubkey, 5 * anchor.web3.LAMPORTS_PER_SOL);
  await conn.confirmTransaction(sig);

  console.log(
      'Airdrop successful! TAG: ',
      logTag,
      ', Pubkey: ',
      pubkey.toString(),
      ', Balance: ',
      await conn.getBalance(pubkey),
      ' lamports.',
  );
}
