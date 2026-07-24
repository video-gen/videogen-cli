import { printJson } from "../output";
import type { ResourceCommands } from "./types";

export const accountCommands: ResourceCommands = {
  // @sdk-operation getMe
  "get-me": async ({ client }) => {
    printJson(await client.account.getMe());
  },
};
