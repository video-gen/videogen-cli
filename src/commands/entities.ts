import { getStringFlag } from "../args";
import { resolveRequest } from "../body";
import { printJson } from "../output";
import { paginationFlags, requireStringFlag } from "./flags";
import type { ResourceCommands } from "./types";

export const entitiesCommands: ResourceCommands = {
  // @sdk-operation listEntities
  "list-entities": async ({ client, args }) => {
    const entityTypeFlag = getStringFlag(args.flags, "entityType");
    const entityType =
      entityTypeFlag === "ACTOR" ||
      entityTypeFlag === "PRODUCT" ||
      entityTypeFlag === "VISUAL_STYLE" ||
      entityTypeFlag === "SLIDESHOW_THEME"
        ? entityTypeFlag
        : undefined;
    if (entityTypeFlag != null && entityType == null) {
      throw new Error(
        `Invalid --entity-type "${entityTypeFlag}". Expected ACTOR, PRODUCT, VISUAL_STYLE, or SLIDESHOW_THEME.`,
      );
    }
    printJson(
      await client.entities.listEntities({
        ...paginationFlags(args.flags),
        ...(entityType != null ? { entityType } : {}),
      }),
    );
  },

  // @sdk-operation createEntity
  "create-entity": async ({ client, args }) => {
    // We intentionally use an unsafe `as` assertion here because the API server validates the body and returns a type error if invalid.
    const request = (await resolveRequest({
      flags: args.flags,
      requireBody: true,
    })) as Parameters<typeof client.entities.createEntity>[0];
    printJson(await client.entities.createEntity(request));
  },

  // @sdk-operation getEntity
  "get-entity": async ({ client, args }) => {
    const entityId = requireStringFlag(args.flags, "entityId");
    printJson(await client.entities.getEntity({ entityId }));
  },

  // @sdk-operation updateEntity
  "update-entity": async ({ client, args }) => {
    const entityId = requireStringFlag(args.flags, "entityId");
    // We intentionally use an unsafe `as` assertion here because the API server validates the body and returns a type error if invalid.
    const request = (await resolveRequest({
      flags: args.flags,
      extra: { entityId },
      requireBody: true,
    })) as Parameters<typeof client.entities.updateEntity>[0];
    printJson(await client.entities.updateEntity(request));
  },

  // @sdk-operation archiveEntity
  "archive-entity": async ({ client, args }) => {
    const entityId = requireStringFlag(args.flags, "entityId");
    printJson(await client.entities.archiveEntity({ entityId }));
  },

  // @sdk-operation addEntityReference
  "add-entity-reference": async ({ client, args }) => {
    const entityId = requireStringFlag(args.flags, "entityId");
    // We intentionally use an unsafe `as` assertion here because the API server validates the body and returns a type error if invalid.
    const request = (await resolveRequest({
      flags: args.flags,
      extra: { entityId },
      requireBody: true,
    })) as Parameters<typeof client.entities.addEntityReference>[0];
    printJson(await client.entities.addEntityReference(request));
  },

  // @sdk-operation removeEntityReference
  "remove-entity-reference": async ({ client, args }) => {
    const entityId = requireStringFlag(args.flags, "entityId");
    // We intentionally use an unsafe `as` assertion here because the API server validates the body and returns a type error if invalid.
    const request = (await resolveRequest({
      flags: args.flags,
      extra: { entityId },
      requireBody: true,
    })) as Parameters<typeof client.entities.removeEntityReference>[0];
    printJson(await client.entities.removeEntityReference(request));
  },
};
