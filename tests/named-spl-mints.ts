import { NamedSplMints } from "../target/types/named_spl_mints";
import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import * as web3 from "@solana/web3.js";
import {
  PublicKey,
  Keypair,
  SystemProgram,
  SYSVAR_RENT_PUBKEY,
} from "@solana/web3.js";
import { Token, TOKEN_PROGRAM_ID, MintLayout } from "@solana/spl-token";
interface Pda {
  address: PublicKey;
  bump: number;
}
describe("named-spl-mints", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);
  const anyAnchor: any = anchor;
  const program = anyAnchor.workspace.NamedSplMints as Program<NamedSplMints>;

  const clientMint = Keypair.generate();
  const programMint = Keypair.generate();
  const creator = provider.wallet;

  let namedMint: Pda;
  let namedMintAttribution: Pda;

  it("program init", async () => {
    let name = "test";
    namedMint = await findMint(name);
    namedMintAttribution = await findAttribution(namedMint.address);

    const tx = await program.rpc.createNewMint(
      namedMint.bump,
      namedMintAttribution.bump,
      name,
      0,
      creator.publicKey,
      creator.publicKey,
      {
        accounts: {
          creator: creator.publicKey,
          mint: namedMint.address,
          attribution: namedMintAttribution.address,
          rent: web3.SYSVAR_RENT_PUBKEY,
          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
        },
      }
    );

    let newMint = await provider.connection.getAccountInfo(namedMint.address);
    //console.log(newMint);

    let newToken = new Token(
      provider.connection,
      namedMint.address,
      TOKEN_PROGRAM_ID,
      Keypair.generate()
    );
    let info = await newToken.getMintInfo();
    console.log(info);

    let newAttr = await program.account.attribution.fetch(
      namedMintAttribution.address
    );
    console.log(newAttr);
  });
});
const anyAnchor: any = anchor;
const NamedSplMints = anyAnchor.workspace
  .NamedSplMints as Program<NamedSplMints>;
const findMint = async (name: String) => {
  return PublicKey.findProgramAddress(
    [anchor.utils.bytes.utf8.encode(name.toLowerCase())],
    NamedSplMints.programId
  ).then(([address, bump]) => {
    return {
      address: address,
      bump: bump,
    };
  });
};
const findAttribution = async (mint: PublicKey) => {
  let [address, bump] = await PublicKey.findProgramAddress(
    [mint.toBuffer()],
    NamedSplMints.programId
  );
  return {
    address: address,
    bump: bump,
  };
};
const getFreezeAuth = async (): Promise<Pda> => {
  let [address, bump] = await PublicKey.findProgramAddress(
    [anchor.utils.bytes.utf8.encode("freeze")],
    NamedSplMints.programId
  );
  return {
    address: address,
    bump: bump,
  };
};
