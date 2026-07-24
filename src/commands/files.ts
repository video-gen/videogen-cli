import { readFileSync } from "node:fs";
import { basename } from "node:path";
import { uploadFile } from "@videogen/sdk";
import { getBooleanFlag, getStringFlag } from "../args";
import { resolveRequest } from "../body";
import { printJson } from "../output";
import { paginationFlags, requireStringFlag } from "./flags";
import type { ResourceCommands } from "./types";

const UPLOAD_FILE_TYPES = ["IMAGE", "VIDEO", "AUDIO", "PDF", "SLIDESHOW"] as const;
type UploadFileType = (typeof UPLOAD_FILE_TYPES)[number];

const parseUploadFileType = (value: string | undefined): UploadFileType | undefined => {
  if (value == null) {
    return undefined;
  }

  for (const fileType of UPLOAD_FILE_TYPES) {
    if (fileType === value) {
      return fileType;
    }
  }

  throw new Error(`Invalid --type. Expected one of: ${UPLOAD_FILE_TYPES.join(", ")}.`);
};

export const filesCommands: ResourceCommands = {
  // @sdk-operation getFiles
  "get-files": async ({ client, args }) => {
    const includeExportFiles = getBooleanFlag(args.flags, "includeExportFiles");
    const includeProjectFiles = getBooleanFlag(args.flags, "includeProjectFiles");
    printJson(
      await client.files.getFiles({
        ...paginationFlags(args.flags),
        ...(includeExportFiles != null ? { includeExportFiles } : {}),
        ...(includeProjectFiles != null ? { includeProjectFiles } : {}),
      }),
    );
  },

  // @sdk-operation searchFiles
  "search-files": async ({ client, args }) => {
    // We intentionally use an unsafe `as` assertion here because the API server validates the body and returns a type error if invalid.
    const request = (await resolveRequest({
      flags: args.flags,
      requireBody: true,
    })) as Parameters<typeof client.files.searchFiles>[0];
    printJson(await client.files.searchFiles(request));
  },

  // @sdk-operation getFile
  "get-file": async ({ client, args }) => {
    const fileId = requireStringFlag(args.flags, "fileId");
    printJson(await client.files.getFile({ fileId }));
  },

  // @sdk-operation createFileUpload
  "create-file-upload": async ({ client, args }) => {
    // We intentionally use an unsafe `as` assertion here because the API server validates the body and returns a type error if invalid.
    const request = (await resolveRequest({
      flags: args.flags,
      requireBody: true,
    })) as Parameters<typeof client.files.createFileUpload>[0];
    printJson(await client.files.createFileUpload(request));
  },

  upload: async ({ client, args }) => {
    const path = args.positionals[0];
    if (path == null || path.length === 0) {
      throw new Error(
        "Usage: videogen files upload <path> [--type ...] [--display-name ...] [--temporary]",
      );
    }

    const data = readFileSync(path);
    const type = parseUploadFileType(getStringFlag(args.flags, "type"));
    const displayName = getStringFlag(args.flags, "displayName") ?? basename(path);
    const temporary = getBooleanFlag(args.flags, "temporary");

    printJson(
      await uploadFile({
        client,
        data,
        displayName,
        ...(type != null ? { type } : {}),
        ...(temporary != null ? { temporary } : {}),
      }),
    );
  },

  // @sdk-operation hydrateFile
  "hydrate-file": async ({ client, args }) => {
    const fileId = requireStringFlag(args.flags, "fileId");
    printJson(await client.files.hydrateFile({ fileId }));
  },

  // @sdk-operation archiveFile
  "archive-file": async ({ client, args }) => {
    const fileId = requireStringFlag(args.flags, "fileId");
    printJson(await client.files.archiveFile({ fileId }));
  },

  // @sdk-operation enablePublicPreview
  "enable-public-preview": async ({ client, args }) => {
    const fileId = requireStringFlag(args.flags, "fileId");
    printJson(await client.files.enablePublicPreview({ fileId }));
  },

  // @sdk-operation disablePublicPreview
  "disable-public-preview": async ({ client, args }) => {
    const fileId = requireStringFlag(args.flags, "fileId");
    printJson(await client.files.disablePublicPreview({ fileId }));
  },
};
