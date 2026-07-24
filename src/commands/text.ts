import { resolveRequest } from "../body";
import { printJson } from "../output";
import type { ResourceCommands } from "./types";

export const textCommands: ResourceCommands = {
  // @sdk-operation generateText
  "generate-text": async ({ client, args }) => {
    // We intentionally use an unsafe `as` assertion here because the API server validates the body and returns a type error if invalid.
    const request = (await resolveRequest({
      flags: args.flags,
      requireBody: true,
    })) as Parameters<typeof client.text.generateText>[0];
    printJson(await client.text.generateText(request));
  },
};
