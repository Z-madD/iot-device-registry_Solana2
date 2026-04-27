import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { IotDeviceRegistry } from "../target/types/iot_device_registry";
import { assert } from "chai";

describe("iot-device-registry", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.IotDeviceRegistry as Program<IotDeviceRegistry>;

  const owner = provider.wallet.publicKey;
  const deviceId = "sensor-001";
  let devicePda: anchor.web3.PublicKey;

  before(async () => {
    // Derivamos el PDA que se usará para todas las operaciones
    [devicePda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("device"), owner.toBuffer(), Buffer.from(deviceId)],
      program.programId
    );
  });

  it("Crea un dispositivo IoT", async () => {
    await program.methods
      .createDevice(
        deviceId,
        "Sensor de temperatura",
        "Temperatura",
        "Oficina central",
        "activo"
      )
      .accounts({
        device: devicePda,
        owner: owner,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    // Leemos la cuenta del dispositivo para verificar
    const deviceAccount = await program.account.device.fetch(devicePda);
    assert.equal(deviceAccount.owner.toString(), owner.toString());
    assert.equal(deviceAccount.deviceId, deviceId);
    assert.equal(deviceAccount.deviceName, "Sensor de temperatura");
    assert.equal(deviceAccount.deviceType, "Temperatura");
    assert.equal(deviceAccount.location, "Oficina central");
    assert.equal(deviceAccount.status, "activo");
    assert.isNumber(deviceAccount.createdAt.toNumber()); // timestamp
  });

  it("Lee un dispositivo IoT (via fetch)", async () => {
    const deviceAccount = await program.account.device.fetch(devicePda);
    assert.equal(deviceAccount.deviceId, deviceId);
    // También podemos llamar a readDevice si queremos probar la instrucción
    await program.methods
      .readDevice(deviceId)
      .accounts({
        device: devicePda,
        owner: owner,
      })
      .rpc();
  });

  it("Actualiza un dispositivo IoT", async () => {
    const newName = "Sensor de humedad";
    const newType = "Humedad";
    const newLocation = "Almacén";
    const newStatus = "inactivo";

    await program.methods
      .updateDevice(deviceId, newName, newType, newLocation, newStatus)
      .accounts({
        device: devicePda,
        owner: owner,
      })
      .rpc();

    const deviceAccount = await program.account.device.fetch(devicePda);
    assert.equal(deviceAccount.deviceName, newName);
    assert.equal(deviceAccount.deviceType, newType);
    assert.equal(deviceAccount.location, newLocation);
    assert.equal(deviceAccount.status, newStatus);
  });

  it("Elimina un dispositivo IoT", async () => {
    // Verificamos el saldo antes de eliminar
    const balanceBefore = await provider.connection.getBalance(owner);

    await program.methods
      .deleteDevice(deviceId)
      .accounts({
        device: devicePda,
        owner: owner,
      })
      .rpc();

    // La cuenta ya no debe existir
    const accountInfo = await provider.connection.getAccountInfo(devicePda);
    assert.isNull(accountInfo);

    // El propietario recibe los lamports (el balance debe ser mayor)
    const balanceAfter = await provider.connection.getBalance(owner);
    assert.isTrue(balanceAfter > balanceBefore);
  });
});
