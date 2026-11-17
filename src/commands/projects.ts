import type { StartVideoEditsResponse } from "@videogen/sdk";
import { getNumberFlag, getStringFlag } from "../args";
import { resolveRequest } from "../body";
import { printJson } from "../output";
import { paginationFlags, requireStringFlag, selfOnlyFlag } from "./flags";
import type { ResourceCommands } from "./types";

export const projectsCommands: ResourceCommands = {
  // @sdk-operation getProjectArtifact
  "get-project-artifact": async ({ client, args }) => {
    printJson(
      await client.projects.getProjectArtifact({
        projectId: requireStringFlag(args.flags, "projectId"),
        artifactId: requireStringFlag(args.flags, "artifactId"),
        textOffset: getNumberFlag(args.flags, "textOffset"),
        textLimit: getNumberFlag(args.flags, "textLimit"),
        itemOffset: getNumberFlag(args.flags, "itemOffset"),
        itemLimit: getNumberFlag(args.flags, "itemLimit"),
      }),
    );
  },
  // @sdk-operation applyProjectEdits
  "apply-project-edits": async ({ client, args }) => {
    const projectId = requireStringFlag(args.flags, "projectId");
    const body = await resolveRequest({ flags: args.flags, requireBody: true });
    printJson(
      await client.request({
        method: "POST",
        path: `/v1/projects/${encodeURIComponent(projectId)}/mutations`,
        body,
      }),
    );
  },
  // @sdk-operation listProjectArtifacts
  "list-project-artifacts": async ({ client, args }) => {
    printJson(
      await client.projects.listProjectArtifacts({
        projectId: requireStringFlag(args.flags, "projectId"),
        ...paginationFlags(args.flags),
      }),
    );
  },
  // @sdk-operation getProjectEditingCapabilities
  "get-project-editing-capabilities": async ({ client, args }) => {
    printJson(
      await client.projects.getProjectEditingCapabilities({
        projectId: requireStringFlag(args.flags, "projectId"),
      }),
    );
  },
  // @sdk-operation inspectEditableVideo
  "inspect-editable-video": async ({ client, args }) => {
    printJson(
      await client.projects.inspectEditableVideo({
        projectId: requireStringFlag(args.flags, "projectId"),
        editableVideoId: requireStringFlag(args.flags, "editableVideoId"),
        offset: getNumberFlag(args.flags, "offset"),
        limit: getNumberFlag(args.flags, "limit"),
        sectionId: getStringFlag(args.flags, "sectionId"),
        layerId: getStringFlag(args.flags, "layerId"),
        assetId: getStringFlag(args.flags, "assetId"),
        expectedRevision: getNumberFlag(args.flags, "expectedRevision"),
        textOffset: getNumberFlag(args.flags, "textOffset"),
      }),
    );
  },
  // @sdk-operation editVideo
  "edit-video": async ({ client, args }) => {
    const projectId = requireStringFlag(args.flags, "projectId");
    const editableVideoId = requireStringFlag(args.flags, "editableVideoId");
    const body = await resolveRequest({ flags: args.flags, requireBody: true });
    // Preserve the complete typed body for server validation, rather than
    // pretending untrusted CLI JSON already satisfies the SDK command union.
    printJson(
      await client.request({
        method: "POST",
        path: `/v1/projects/${encodeURIComponent(projectId)}/videos/${encodeURIComponent(editableVideoId)}/edits`,
        body,
      }),
    );
  },
  // @sdk-operation startVideoEdits
  "start-video-edits": async ({ client, args, wait }) => {
    const projectId = requireStringFlag(args.flags, "projectId");
    const editableVideoId = getStringFlag(args.flags, "editableVideoId");
    const body = await resolveRequest({
      flags: args.flags,
      extra: editableVideoId == null ? {} : { editableVideoId },
      requireBody: true,
    });
    // Preserve the complete JSON body so the server validates the edit union.
    const started = await client.request<StartVideoEditsResponse>({
      method: "POST",
      path: `/v1/projects/${encodeURIComponent(projectId)}/video-edits`,
      body,
    });
    printJson(
      wait
        ? await client.pollVideoEdits({
            projectId: started.projectId,
            videoEditIds: started.videoEditIds,
          })
        : started,
    );
  },

  // @sdk-operation listVideoEdits
  "list-video-edits": async ({ client, args }) => {
    const editableVideoId = getStringFlag(args.flags, "editableVideoId");
    printJson(
      await client.projects.listVideoEdits({
        projectId: requireStringFlag(args.flags, "projectId"),
        ...paginationFlags(args.flags),
        ...(editableVideoId == null ? {} : { editableVideoId }),
      }),
    );
  },

  // @sdk-operation getProjectEdit
  "get-project-edit": async ({ client, args }) => {
    printJson(
      await client.projects.getProjectEdit({
        projectId: requireStringFlag(args.flags, "projectId"),
        mutationId: requireStringFlag(args.flags, "mutationId"),
      }),
    );
  },
  // @sdk-operation undoProjectEdit
  "undo-project-edit": async ({ client, args }) => {
    printJson(
      await client.projects.undoProjectEdit({
        projectId: requireStringFlag(args.flags, "projectId"),
        mutationId: requireStringFlag(args.flags, "mutationId"),
        newMutationId: requireStringFlag(args.flags, "newMutationId"),
      }),
    );
  },
  // @sdk-operation listProjects
  "list-projects": async ({ client, args }) => {
    printJson(
      await client.projects.listProjects({
        ...paginationFlags(args.flags),
        ...selfOnlyFlag(args.flags),
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
    printJson(
      await client.projects.getTimelineInterchange({ interchangeJobId }),
    );
  },
};

/** Existing commands stay executable without appearing in command discovery. */
export const legacyProjectsCommands: ResourceCommands = {
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
