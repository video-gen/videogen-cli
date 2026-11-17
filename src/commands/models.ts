import { resolveRequest } from "../body";
import { printJson } from "../output";
import { requireStringFlag } from "./flags";
import type { ResourceCommands } from "./types";

export const modelsCommands: ResourceCommands = {
  // @sdk-operation startModel1Generation
  "start-model1-generation": async ({ client, args }) => {
    const body = await resolveRequest({ flags: args.flags, requireBody: true });
    const { prompt, seconds, imageFileIds } = body;
    if (
      typeof prompt !== "string" ||
      (seconds != null && typeof seconds !== "number")
    )
      throw new TypeError(
        "Provide a prompt string and optional seconds number.",
      );
    if (
      Object.keys(body).some(
        (key) => !["prompt", "seconds", "imageFileIds"].includes(key),
      )
    )
      throw new TypeError(
        "Only prompt, seconds, and imageFileIds are accepted in the body.",
      );
    const images: string[] = [];
    if (imageFileIds != null) {
      if (!Array.isArray(imageFileIds))
        throw new TypeError("imageFileIds must be an array of file ids.");
      for (const id of imageFileIds) {
        if (typeof id !== "string")
          throw new TypeError("Every imageFileIds entry must be a string.");
        images.push(id);
      }
    }
    printJson(
      await client.models.startModel1Generation(
        {
          prompt,
          ...(seconds == null ? {} : { seconds }),
          imageFileIds: images,
        },
        { idempotencyKey: requireStringFlag(args.flags, "idempotencyKey") },
      ),
    );
  },
  // @sdk-operation getModel1Generation
  "get-model1-generation": async ({ client, args }) => {
    printJson(
      await client.models.getModel1Generation({
        generationId: requireStringFlag(args.flags, "generationId"),
      }),
    );
  },
  // @sdk-operation getModel1Quota
  "get-model1-quota": async ({ client }) => {
    printJson(await client.models.getModel1Quota());
  },
};
