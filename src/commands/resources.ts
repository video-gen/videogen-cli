import { getBooleanFlag, getStringFlag } from "../args";
import { printJson } from "../output";
import { paginationFlags } from "./flags";
import type { ResourceCommands } from "./types";

export const resourcesCommands: ResourceCommands = {
  // @sdk-operation listTtsVoices
  "list-tts-voices": async ({ client, args }) => {
    const includeDeprecatedVoices = getBooleanFlag(args.flags, "includeDeprecatedVoices");
    const query = getStringFlag(args.flags, "query");
    printJson(
      await client.resources.listTtsVoices({
        ...paginationFlags(args.flags),
        ...(includeDeprecatedVoices != null ? { includeDeprecatedVoices } : {}),
        ...(query != null ? { query } : {}),
      }),
    );
  },

  // @sdk-operation listLanguages
  "list-languages": async ({ client, args }) => {
    const query = getStringFlag(args.flags, "query");
    printJson(
      await client.resources.listLanguages({
        ...(query != null ? { query } : {}),
      }),
    );
  },
};
