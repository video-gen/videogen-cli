import assert from "node:assert/strict";
import { test } from "node:test";
import { VideoGen } from "@videogen/sdk";
import { parseArgs } from "../src/args.ts";
import { modelsCommands } from "../src/commands/models.ts";

test("CLI Model 1 submit keeps idempotency in the header and returns the generation id", async (t) => {
  const calls = []; const output = [];
  t.mock.method(globalThis, "fetch", async (url, init) => {
    calls.push({ url, init });
    return new Response(JSON.stringify({ generationId: "vg_mgen_test" }), { status: 202 });
  });
  t.mock.method(process.stdout, "write", (chunk) => { output.push(String(chunk)); return true; });
  const client = new VideoGen({ apiKey: "test-key" });
  const command = modelsCommands["start-model1-generation"];
  assert.ok(command != null);
  await command({ client, args: parseArgs(["--idempotency-key", "retry-key-from-cli", "--body", '{"prompt":"A product","seconds":10}']), wait: false });
  assert.equal(calls.length, 1); assert.equal(new Headers(calls[0].init.headers).get("Idempotency-Key"), "retry-key-from-cli");
  assert.deepEqual(JSON.parse(calls[0].init.body), { prompt: "A product", seconds: 10, imageFileIds: [] });
  assert.match(output.join(""), /vg_mgen_test/);
});
