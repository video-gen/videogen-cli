import assert from "node:assert/strict";
import { test } from "node:test";
import { VideoGen } from "@videogen/sdk";
import { parseArgs } from "../src/args.ts";
import { billingCommands } from "../src/commands/billing.ts";

const orderId = "3ad99513-4692-48fd-9429-578d36e601be";
const paymentToken = "spt_fake_cli_test_only";
const idempotencyKey = "approved-purchase-0001";

const invoke = (name, flags = []) => billingCommands[name]({
  client: new VideoGen({ apiKey: "test-api-key", baseUrl: "https://example.test", clientId: "cli" }),
  args: parseArgs(["billing", name, ...flags]),
  wait: false,
});

const capture = (t, response = { orderId, state: "requires_approval" }, status = 200) => {
  const requests = [];
  const output = [];
  t.mock.method(globalThis, "fetch", async (url, init) => {
    requests.push({ url, init });
    return new Response(JSON.stringify(response), {
      status,
      headers: { "Content-Type": "application/json" },
    });
  });
  t.mock.method(process.stdout, "write", (chunk) => {
    output.push(String(chunk));
    return true;
  });
  return { requests, output };
};

const setToken = (t, value) => {
  const previous = process.env.VIDEOGEN_PAYMENT_TOKEN;
  if (value == null) {
    delete process.env.VIDEOGEN_PAYMENT_TOKEN;
  } else {
    process.env.VIDEOGEN_PAYMENT_TOKEN = value;
  }
  t.after(() => {
    if (previous == null) {
      delete process.env.VIDEOGEN_PAYMENT_TOKEN;
    } else {
      process.env.VIDEOGEN_PAYMENT_TOKEN = previous;
    }
  });
};

for (const { name, flags, method, path, body } of [
  { name: "get-billing-account", flags: [], method: "GET", path: "/account" },
  {
    name: "create-billing-order",
    flags: ["--body", JSON.stringify({ orderId, credits: 2000 })],
    method: "POST", path: "/orders", body: { orderId, credits: 2000 },
  },
  {
    name: "create-billing-order",
    flags: ["--order-id", orderId, "--body", '{"credits":2000}'],
    method: "POST", path: "/orders", body: { orderId, credits: 2000 },
  },
  {
    name: "get-billing-order", flags: ["--order-id", orderId],
    method: "GET", path: `/orders/${orderId}`,
  },
  {
    name: "cancel-billing-order", flags: ["--order-id", orderId],
    method: "POST", path: `/orders/${orderId}/cancel`, body: {},
  },
  {
    name: "request-billing-refund",
    flags: ["--order-id", orderId, "--body", '{"reason":"Wrong quantity."}'],
    method: "POST", path: `/orders/${orderId}/refund`, body: { reason: "Wrong quantity." },
  },
]) {
  test(`${name} maps to its REST operation (${JSON.stringify(flags)})`, async (t) => {
    const { requests, output } = capture(t);
    await invoke(name, flags);
    assert.equal(requests.length, 1);
    assert.equal(requests[0].url, `https://example.test/v1/billing${path}`);
    assert.equal(requests[0].init.method, method);
    assert.deepEqual(requests[0].init.body == null ? undefined : JSON.parse(requests[0].init.body), body);
    assert.equal(new Headers(requests[0].init.headers).get("Stripe-Shared-Payment-Token"), null);
    assert.deepEqual(JSON.parse(output.join("")), { orderId, state: "requires_approval" });
  });
}

test("completion sends the environment credential only in the header and prints the order state", async (t) => {
  setToken(t, paymentToken);
  const { requests, output } = capture(t, { orderId, state: "paid_pending_fulfillment" });
  await invoke("complete-billing-order", [
    "--order-id", orderId,
    "--body", JSON.stringify({ revision: 2, idempotencyKey }),
    "--wait",
  ]);
  assert.equal(requests.length, 1);
  const request = requests[0];
  assert.equal(request.url, `https://example.test/v1/billing/orders/${orderId}/complete`);
  assert.equal(request.init.method, "POST");
  assert.equal(new Headers(request.init.headers).get("Stripe-Shared-Payment-Token"), paymentToken);
  assert.deepEqual(JSON.parse(request.init.body), { revision: 2, idempotencyKey });
  assert.equal(request.init.body.includes(paymentToken), false);
  assert.equal(request.url.includes(paymentToken), false);
  assert.equal(output.join("").includes(paymentToken), false);
  assert.deepEqual(JSON.parse(output.join("")), { orderId, state: "paid_pending_fulfillment" });
});

test("completion never retries an uncertain payment response", async (t) => {
  setToken(t, paymentToken);
  const { requests, output } = capture(t, { message: "Check the original order." }, 503);
  await assert.rejects(invoke("complete-billing-order", [
    "--order-id", orderId,
    "--body", JSON.stringify({ revision: 2, idempotencyKey }),
  ]));
  assert.equal(requests.length, 1);
  assert.equal(output.length, 0);
});

for (const credential of [undefined, "", " token-with-spaces "]) {
  test(`completion refuses a missing or invalid environment credential (${JSON.stringify(credential)})`, async (t) => {
    setToken(t, credential);
    const { requests, output } = capture(t);
    await assert.rejects(invoke("complete-billing-order", [
      "--order-id", orderId,
      "--body", JSON.stringify({ revision: 2, idempotencyKey }),
    ]), /VIDEOGEN_PAYMENT_TOKEN/);
    assert.equal(requests.length, 0);
    assert.equal(output.length, 0);
  });
}

for (const flags of [
  ["--payment-token", paymentToken, "--body", JSON.stringify({ revision: 2, idempotencyKey })],
  ["--stripe-shared-payment-token", paymentToken, "--body", JSON.stringify({ revision: 2, idempotencyKey })],
  ["--body", JSON.stringify({ revision: 2, idempotencyKey, paymentToken })],
  ["--body", `{"paymentToken":"${paymentToken}"`],
]) {
  test("completion rejects credentials in command arguments and JSON without disclosing them", async (t) => {
    setToken(t, paymentToken);
    const { requests, output } = capture(t);
    await assert.rejects(invoke("complete-billing-order", ["--order-id", orderId, ...flags]), (error) => {
      assert.equal(error.message.includes(paymentToken), false);
      return true;
    });
    assert.equal(requests.length, 0);
    assert.equal(output.length, 0);
  });
}

for (const { name, flags } of [
  { name: "create-billing-order", flags: ["--body", JSON.stringify({ orderId, credits: "2000" })] },
  { name: "complete-billing-order", flags: ["--order-id", orderId, "--body", JSON.stringify({ revision: "2", idempotencyKey })] },
  { name: "request-billing-refund", flags: ["--order-id", orderId, "--body", '{"reason":7}'] },
]) {
  test(`${name} rejects malformed input before sending a request`, async (t) => {
    const { requests } = capture(t);
    await assert.rejects(invoke(name, flags), TypeError);
    assert.equal(requests.length, 0);
  });
}
