import { resolveRequest } from "../body";
import { printJson } from "../output";
import { requireStringFlag } from "./flags";
import type { ResourceCommands } from "./types";

export const assistantCommands: ResourceCommands = {
  // @sdk-operation startAssistantChat
  "start-assistant-chat": async ({ client, args }) => {
    // We intentionally use an unsafe `as` assertion here because the API server validates the body and returns a type error if invalid.
    const request = (await resolveRequest({
      flags: args.flags,
      requireBody: true,
    })) as Parameters<typeof client.assistant.startAssistantChat>[0];
    printJson(await client.assistant.startAssistantChat(request));
  },

  // @sdk-operation sendAssistantMessage
  "send-assistant-message": async ({ client, args }) => {
    const projectId = requireStringFlag(args.flags, "projectId");
    // We intentionally use an unsafe `as` assertion here because the API server validates the body and returns a type error if invalid.
    const request = (await resolveRequest({
      flags: args.flags,
      extra: { projectId },
      requireBody: true,
    })) as Parameters<typeof client.assistant.sendAssistantMessage>[0];
    printJson(await client.assistant.sendAssistantMessage(request));
  },

  // @sdk-operation actOnAssistantAction
  "act-on-assistant-action": async ({ client, args }) => {
    const projectId = requireStringFlag(args.flags, "projectId");
    const assistantActionId = requireStringFlag(args.flags, "assistantActionId");
    // We intentionally use an unsafe `as` assertion here because the API server validates the body and returns a type error if invalid.
    const request = (await resolveRequest({
      flags: args.flags,
      extra: { projectId, assistantActionId },
      requireBody: true,
    })) as Parameters<typeof client.assistant.actOnAssistantAction>[0];
    printJson(await client.assistant.actOnAssistantAction(request));
  },
};
