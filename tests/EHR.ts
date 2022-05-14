import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Ehr } from "../target/types/ehr";
import {expect} from "chai";
import {PublicKey} from "@solana/web3.js";

describe("EHR", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Ehr as Program<Ehr>;
  
  it("Is initialized!", async () => {
    // Add your test here.
    const [patientEHRPDA, patientEHRBump] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode("patient-profile"),
        provider.wallet.publicKey.toBuffer()
      ],
      program.programId
    );
    
    const tx = await program.methods.initPatientProfile("Bhargav","Male","23/11/1994")
                                    .accounts({
                                      patient: provider.wallet.publicKey,
                                      patientEhrAccount: patientEHRPDA,
                                    }).rpc();
    console.log("Your transaction signature", tx);
    expect((await program.account.patientEhr.fetch(patientEHRPDA)).name).to.equal("Bhargav");
    const patientEhrAccounts = await program.account.patientEhr.all();
    console.log("patientEhr");
    console.log(patientEhrAccounts);
    
  });
});
