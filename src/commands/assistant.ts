import { resolveRequest } from "../body";
import { printJson } from "../output";
import { requireStringFlag } from "./flags";
import type { ResourceCommands } from "./types";

export const assistantCommands: ResourceCommands = {
  // @sdk-operation startAssistantChat
  "start-assistant-chat": async ({ client, args, wait }) => {
    // We intentionally use an unsafe `as` assertion here because the API server validates the body and returns a type error if invalid.
    const request = (await resolveRequest({
      flags: args.flags,
      requireBody: true,
    })) as Parameters<typeof client.assistant.startAssistantChat>[0];
    printJson(
      wait
        ? await client.assistant.startAssistantChatAndWait(request)
        : await client.assistant.startAssistantChat(request),
    );
  },

  // @sdk-operation getAssistant
  "get-assistant": async ({ client, args }) => {
    const assistantId = requireStringFlag(args.flags, "assistantId");
    printJson(await client.assistant.getAssistant({ assistantId }));
  },

  // @sdk-operation sendAssistantMessage
  "send-assistant-message": async ({ client, args, wait }) => {
    const assistantId = requireStringFlag(args.flags, "assistantId");
    // We intentionally use an unsafe `as` assertion here because the API server validates the body and returns a type error if invalid.
    const request = (await resolveRequest({
      flags: args.flags,
      extra: { assistantId },
      requireBody: true,
    })) as Parameters<typeof client.assistant.sendAssistantMessage>[0];
    printJson(
      wait
        ? await client.assistant.sendAssistantMessageAndWait(request)
        : await client.assistant.sendAssistantMessage(request),
    );
  },

  // @sdk-operation actOnAssistantAction
  "act-on-assistant-action": async ({ client, args, wait }) => {
    const assistantId = requireStringFlag(args.flags, "assistantId");
    const actionId = requireStringFlag(args.flags, "actionId");
    // We intentionally use an unsafe `as` assertion here because the API server validates the body and returns a type error if invalid.
    const request = (await resolveRequest({
      flags: args.flags,
      extra: { assistantId, actionId },
      requireBody: false,
    })) as Parameters<typeof client.assistant.actOnAssistantAction>[0];
    printJson(
      wait
        ? await client.assistant.actOnAssistantActionAndWait(request)
        : await client.assistant.actOnAssistantAction(request),
    );
  },

  // @sdk-operation getAssistantMessage
  "get-assistant-message": async ({ client, args }) => {
    const messageId = requireStringFlag(args.flags, "messageId");
    printJson(await client.assistant.getAssistantMessage({ messageId }));
  },
};
