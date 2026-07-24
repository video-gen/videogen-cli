import { getBooleanFlag, getStringFlag } from "../args";
import { printJson } from "../output";
import { paginationFlags } from "./flags";
import type { ResourceCommands } from "./types";

export const resourcesCommands: ResourceCommands = {
  // @sdk-operation listAvatarPresenters
  "list-avatar-presenters": async ({ client, args }) => {
    const voiceId = getStringFlag(args.flags, "voiceId");
    printJson(
      await client.resources.listAvatarPresenters({
        ...paginationFlags(args.flags),
        ...(voiceId != null ? { voiceId } : {}),
      }),
    );
  },

  // @sdk-operation listTtsVoices
  "list-tts-voices": async ({ client, args }) => {
    const includeDeprecatedVoices = getBooleanFlag(args.flags, "includeDeprecatedVoices");
    printJson(
      await client.resources.listTtsVoices({
        ...paginationFlags(args.flags),
        ...(includeDeprecatedVoices != null ? { includeDeprecatedVoices } : {}),
      }),
    );
  },

  // @sdk-operation listLanguages
  "list-languages": async ({ client }) => {
    printJson(await client.resources.listLanguages());
  },
};
