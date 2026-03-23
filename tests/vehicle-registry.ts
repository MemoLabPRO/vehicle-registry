import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { VehicleRegistry } from "../target/types/vehicle_registry";
import { assert } from "chai";

describe("vehicle-registry", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.VehicleRegistry as Program<VehicleRegistry>;

  // VIN de prueba — 17 bytes exactos
  const vin = Array.from(Buffer.from("1HGBH41JXMN109186"));

  const [vehiclePda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("vehicle"), Buffer.from(vin)],
    program.programId
  );

  it("Registra un vehículo", async () => {
    await program.methods
      .registerVehicle(vin, "Honda", "Civic", 2021)
      .accounts({
        vehicleRecord: vehiclePda,
        owner: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const record = await program.account.vehicleRecord.fetch(vehiclePda);
    assert.equal(record.make, "Honda");
    assert.equal(record.model, "Civic");
    assert.equal(record.year, 2021);
    assert.deepEqual(record.status, { active: {} });
    console.log("  Propietario:", record.owner.toBase58());
  });

  it("Cambia estado a ForSale", async () => {
    await program.methods
      .updateStatus(vin, { forSale: {} })
      .accounts({
        vehicleRecord: vehiclePda,
        owner: provider.wallet.publicKey,
      })
      .rpc();

    const record = await program.account.vehicleRecord.fetch(vehiclePda);
    assert.deepEqual(record.status, { forSale: {} });
    console.log("  Estado:", JSON.stringify(record.status));
  });

  it("Transfiere el título a nuevo propietario", async () => {
    const newOwner = anchor.web3.Keypair.generate();

    await program.methods
      .transferTitle(vin)
      .accounts({
        vehicleRecord: vehiclePda,
        owner: provider.wallet.publicKey,
        newOwner: newOwner.publicKey,
      })
      .rpc();

    const record = await program.account.vehicleRecord.fetch(vehiclePda);
    assert.equal(record.owner.toBase58(), newOwner.publicKey.toBase58());
    assert.deepEqual(record.status, { active: {} });
    console.log("  Nuevo propietario:", record.owner.toBase58());
  });
});