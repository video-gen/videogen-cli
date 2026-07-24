import { getBooleanFlag } from "../args";
import { resolveRequest } from "../body";
import { printJson } from "../output";
import { paginationFlags, requireStringFlag, selfOnlyFlag } from "./flags";
import type { ResourceCommands } from "./types";

export const projectsCommands: ResourceCommands = {
  // @sdk-operation listProjects
  "list-projects": async ({ client, args }) => {
    const includeUiProjects = getBooleanFlag(args.flags, "includeUiProjects");
    printJson(
      await client.projects.listProjects({
        ...paginationFlags(args.flags),
        ...selfOnlyFlag(args.flags),
        ...(includeUiProjects != null ? { includeUiProjects } : {}),
      }),
    );
  },

  // @sdk-operation getProject
  "get-project": async ({ client, args }) => {
    const projectId = requireStringFlag(args.flags, "projectId");
    printJson(await client.projects.getProject({ projectId }));
  },

  // @sdk-operation exportProject
  "export-project": async ({ client, args, wait }) => {
    const projectId = requireStringFlag(args.flags, "projectId");
    // We intentionally use an unsafe `as` assertion here because the API server validates the body and returns a type error if invalid.
    const request = (await resolveRequest({
      flags: args.flags,
      extra: { projectId },
      requireBody: false,
    })) as Parameters<typeof client.projects.exportProject>[0];
    printJson(
      wait
        ? await client.projects.exportAndWait(request)
        : await client.projects.exportProject(request),
    );
  },

  // @sdk-operation listProjectExports
  "list-project-exports": async ({ client, args }) => {
    const projectId = requireStringFlag(args.flags, "projectId");
    printJson(
      await client.projects.listProjectExports({
        projectId,
        ...paginationFlags(args.flags),
      }),
    );
  },

  // @sdk-operation getProjectExport
  "get-project-export": async ({ client, args }) => {
    const projectId = requireStringFlag(args.flags, "projectId");
    const exportId = requireStringFlag(args.flags, "exportId");
    printJson(await client.projects.getProjectExport({ projectId, exportId }));
  },

  // @sdk-operation createTimelineInterchange
  "create-timeline-interchange": async ({ client, args, wait }) => {
    const projectId = requireStringFlag(args.flags, "projectId");
    // We intentionally use an unsafe `as` assertion here because the API server validates the body and returns a type error if invalid.
    const request = (await resolveRequest({
      flags: args.flags,
      extra: { projectId },
      requireBody: false,
    })) as Parameters<typeof client.projects.createTimelineInterchange>[0];
    printJson(
      wait
        ? await client.projects.createTimelineInterchangeAndWait(request)
        : await client.projects.createTimelineInterchange(request),
    );
  },

  // @sdk-operation getTimelineInterchange
  "get-timeline-interchange": async ({ client, args }) => {
    const interchangeJobId = requireStringFlag(args.flags, "interchangeJobId");
    printJson(await client.projects.getTimelineInterchange({ interchangeJobId }));
  },

  // @sdk-operation remixProject
  "remix-project": async ({ client, args, wait }) => {
    const projectId = requireStringFlag(args.flags, "projectId");
    // We intentionally use an unsafe `as` assertion here because the API server validates the body and returns a type error if invalid.
    const request = (await resolveRequest({
      flags: args.flags,
      extra: { projectId },
      requireBody: true,
    })) as Parameters<typeof client.projects.remixProject>[0];
    printJson(
      wait
        ? await client.projects.remixAndWait(request)
        : await client.projects.remixProject(request),
    );
  },

  // @sdk-operation listProjectRemixActions
  "list-project-remix-actions": async ({ client, args }) => {
    const projectId = requireStringFlag(args.flags, "projectId");
    printJson(
      await client.projects.listProjectRemixActions({
        projectId,
        ...paginationFlags(args.flags),
      }),
    );
  },
};
