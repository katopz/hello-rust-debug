import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { DebugAnchor } from '../target/types/debug_anchor';

describe('debug_anchor', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.DebugAnchor as Program<DebugAnchor>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
