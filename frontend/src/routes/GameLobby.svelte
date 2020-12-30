<script lang="typescript">
  import { push } from "svelte-spa-router";
  import Dialog from "../components/layout/Dialog.svelte";
  import DialogHeader from "../components/headers/DialogHeader.svelte";
  import { GameDetails } from "../types/Game";
  import InternalLink from "../components/buttons/InternalLink.svelte";
  import { getToken } from "../utils/auth";
  import Settings from "./tasks/Settings.svelte";
  import WaitForTask from "./tasks/WaitForTask.svelte";
  import DiscloseRole from "./tasks/DiscloseRole.svelte";
  import { TaskType } from "../types/Tasks";
  import { sendRequest } from "../utils/requests";

  export let params: { token?: string } = {};
  let details: GameDetails | null = null;
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
    await refreshGame();
    refreshId = setInterval(() => {
      refreshGame();
    }, 3000);
  };
</script>

<Dialog>
  <DialogHeader>Lobby</DialogHeader>
  <div>
    {#await fetchGamePeriodically()}
      <p>Loading game</p>
    {:then _}
      {#if details !== null}
        {#if details.openTasks.length === 0}
          <WaitForTask {leaveGame} />
        {:else if details.openTasks[0][TaskType.Settings]}
          <Settings {leaveGame} {refreshGame} {details} />
        {:else if details.openTasks[0][TaskType.DiscloseRole]}
          <DiscloseRole
            {leaveGame}
            {refreshGame}
            role={details.openTasks[0][TaskType.DiscloseRole].role} />
        {:else}
          <p>Unsupported Game State.</p>
        {/if}
      {:else}
        <p>Game doesn't exist.</p>
        <div class="flex items-center">
          <div class="flex ml-auto">
            <InternalLink href="/games">Back to Games</InternalLink>
          </div>
        </div>
      {/if}
    {:catch _}
      <p style="color: red">Loading game has failed</p>
      <div class="flex items-center">
        <div class="flex ml-auto">
          <InternalLink href="/games">Back to Games</InternalLink>
        </div>
      </div>
    {/await}
  </div>
</Dialog>
