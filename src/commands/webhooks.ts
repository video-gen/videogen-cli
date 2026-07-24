import { resolveRequest } from "../body";
import { printJson } from "../output";
import { paginationFlags, requireStringFlag } from "./flags";
import type { ResourceCommands } from "./types";

export const webhooksCommands: ResourceCommands = {
  // @sdk-operation listWebhookEndpoints
  "list-webhook-endpoints": async ({ client, args }) => {
    printJson(await client.webhooks.listWebhookEndpoints(paginationFlags(args.flags)));
  },

  // @sdk-operation createWebhookEndpoint
  "create-webhook-endpoint": async ({ client, args }) => {
    // We intentionally use an unsafe `as` assertion here because the API server validates the body and returns a type error if invalid.
    const request = (await resolveRequest({
      flags: args.flags,
      requireBody: true,
    })) as Parameters<typeof client.webhooks.createWebhookEndpoint>[0];
    printJson(await client.webhooks.createWebhookEndpoint(request));
  },

  // @sdk-operation deleteWebhookEndpoint
  "delete-webhook-endpoint": async ({ client, args }) => {
    const endpointId = requireStringFlag(args.flags, "endpointId");
    printJson(await client.webhooks.deleteWebhookEndpoint({ endpointId }));
  },
};
