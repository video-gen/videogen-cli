import { getStringFlag } from "../args";
import { resolveRequest } from "../body";
import { printJson } from "../output";
import { requireStringFlag } from "./flags";
import type { ResourceCommands } from "./types";

const requireFields = (body: Record<string, unknown>, allowed: readonly string[]): void => {
  if (Object.keys(body).some((key) => !allowed.includes(key))) {
    throw new TypeError("Unsupported billing request field. Payment credentials must use VIDEOGEN_PAYMENT_TOKEN.");
  }
};

const rejectPaymentTokenFlags = (flags: Record<string, string | boolean>): void => {
  if ("paymentToken" in flags || "stripeSharedPaymentToken" in flags) {
    throw new TypeError("Payment credentials must use VIDEOGEN_PAYMENT_TOKEN, never command-line flags.");
  }
};

const requirePaymentToken = (): string => {
  const token = process.env["VIDEOGEN_PAYMENT_TOKEN"];
  if (token == null || token.length === 0 || token.length > 300 || token.trim() !== token) {
    throw new TypeError("Set VIDEOGEN_PAYMENT_TOKEN to the scoped payment credential using your secure environment.");
  }
  return token;
};

export const billingCommands: ResourceCommands = {
  // @sdk-operation getBillingAccount
  "get-billing-account": async ({ client }) => {
    printJson(await client.billing.getBillingAccount());
  },

  // @sdk-operation createBillingOrder
  "create-billing-order": async ({ client, args }) => {
    rejectPaymentTokenFlags(args.flags);
    const orderIdFlag = getStringFlag(args.flags, "orderId");
    const body = await resolveRequest({
      flags: args.flags,
      extra: orderIdFlag == null ? {} : { orderId: orderIdFlag },
      requireBody: true,
    });
    requireFields(body, ["orderId", "credits"]);
    const { orderId, credits } = body;
    if (typeof orderId !== "string" || typeof credits !== "number") {
      throw new TypeError("Billing quote requires an orderId string and credits number.");
    }
    printJson(await client.billing.createBillingOrder({ orderId, credits }));
  },

  // @sdk-operation getBillingOrder
  "get-billing-order": async ({ client, args }) => {
    const orderId = requireStringFlag(args.flags, "orderId");
    printJson(await client.billing.getBillingOrder({ orderId }));
  },

  // @sdk-operation completeBillingOrder
  "complete-billing-order": async ({ client, args }) => {
    rejectPaymentTokenFlags(args.flags);
    const orderId = requireStringFlag(args.flags, "orderId");
    const body = await resolveRequest({ flags: args.flags, requireBody: true }).catch(() => {
      throw new TypeError("Invalid billing completion body. Provide revision and idempotencyKey only.");
    });
    requireFields(body, ["revision", "idempotencyKey"]);
    const { revision, idempotencyKey } = body;
    if (typeof revision !== "number" || typeof idempotencyKey !== "string") {
      throw new TypeError("Billing completion requires a revision number and idempotencyKey string.");
    }
    const paymentToken = requirePaymentToken();
    printJson(
      await client.billing.completeBillingOrder(
        { orderId, revision, idempotencyKey },
        { paymentToken },
      ),
    );
  },

  // @sdk-operation cancelBillingOrder
  "cancel-billing-order": async ({ client, args }) => {
    const orderId = requireStringFlag(args.flags, "orderId");
    printJson(await client.billing.cancelBillingOrder({ orderId }));
  },

  // @sdk-operation requestBillingRefund
  "request-billing-refund": async ({ client, args }) => {
    rejectPaymentTokenFlags(args.flags);
    const orderId = requireStringFlag(args.flags, "orderId");
    const body = await resolveRequest({ flags: args.flags, requireBody: true });
    requireFields(body, ["reason"]);
    const { reason } = body;
    if (typeof reason !== "string") {
      throw new TypeError("Billing refund request requires a reason string.");
    }
    printJson(await client.billing.requestBillingRefund({ orderId, reason }));
  },
};
