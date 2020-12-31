<script lang="typescript">
  import { push } from "svelte-spa-router";
  import Dialog from "../components/layout/Dialog.svelte";
  import DialogHeader from "../components/headers/DialogHeader.svelte";
  import type { GameDetails } from "../types/Game";
  import InternalLink from "../components/buttons/InternalLink.svelte";
  import { getToken } from "../utils/auth";
  import Settings from "./tasks/Settings.svelte";
  import WaitForTask from "./tasks/WaitForTask.svelte";
  import DiscloseRole from "./tasks/DiscloseRole.svelte";
  import Discuss from "./tasks/Discuss.svelte";
  import { Tasks, TaskType } from "../types/Tasks";
  import { sendRequest } from "../utils/requests";

  export let params: { token?: string } = {};
  let details: GameDetails | null = null;
  let currentTask: Tasks | null = null;
  let refreshId: number | null = null;

  const refreshGame = async () => {
    const res = await sendRequest<GameDetails>(
      `/api/games/${params.token}/details`,
      "GET"
    );
    if (!res) {
      details = null;
      if (refreshId) {
        clearInterval(refreshId);
      }
      return;
    }

    details = res;
    currentTask = details.openTasks.length > 0 ? details.openTasks[0] : null;
  };

  const leaveGame = async () => {
    if (getToken()) {
      await sendRequest(`/api/games/${params.token}/leave`, "POST");
    }
    if (refreshId) {
      clearInterval(refreshId);
    }
    await push("/games");
  };

  const fetchGamePeriodically = async () => {
    try {
      await refreshGame();
    } catch (err) {
      console.error(err);
      await push("/errors/unexpected");
    }
    refreshId = setInterval(() => {
      refreshGame();
    }, 3000);
  };
</script>

<Dialog>
  {#await fetchGamePeriodically()}
    <DialogHeader>Lobby</DialogHeader>
    <p>Loading game</p>
  {:then _}
    {#if details !== null}
      {#if !currentTask}
        <WaitForTask {leaveGame} />
      {:else if currentTask[TaskType.Settings]}
        <Settings {leaveGame} {refreshGame} {details} />
      {:else if currentTask[TaskType.DiscloseRole]}
        <DiscloseRole
          {leaveGame}
          {refreshGame}
          role={currentTask[TaskType.DiscloseRole].role} />
      {:else if currentTask[TaskType.Discuss]}
        <Discuss {leaveGame} />
      {:else}
        <p>Unsupported Game State.</p>
      {/if}
    {:else}
      <DialogHeader>Lobby</DialogHeader>
      <p>Game doesn't exist.</p>
      <div class="flex items-center">
        <div class="flex ml-auto">
          <InternalLink href="/games">Back to Games</InternalLink>
        </div>
      </div>
    {/if}
  {/await}
</Dialog>
