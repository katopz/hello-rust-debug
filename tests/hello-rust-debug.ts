import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { HelloRustDebug } from '../target/types/hello_rust_debug';

describe('hello-rust-debug', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.HelloRustDebug as Program<HelloRustDebug>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
