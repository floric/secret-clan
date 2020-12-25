<script lang="typescript">
  import { push } from "svelte-spa-router";
  import Dialog from "../components/layout/Dialog.svelte";
  import DialogHeader from "../components/headers/DialogHeader.svelte";
  import { Game, GameDetails, GameState } from "../types/Game";
  import type { PublicPlayer } from "../types/Player";
  import InternalLink from "../components/buttons/InternalLink.svelte";
  import { getToken } from "../utils/auth";
  import Settings from "./pages/Settings.svelte";
  import ActiveGame from "./pages/ActiveGame.svelte";

  export let params: { token?: string } = {};
  let details: GameDetails | null = null;
  let refreshId: number | null = null;

  const fetchPlayerById = async (id: string): Promise<PublicPlayer | null> => {
    const res = await fetch(`/api/players/${id}`);
    if (!res.ok) {
      return null;
    }

    return (await res.json()) as PublicPlayer;
  };

  const refreshGame = async () => {
    const res = await fetch(`/api/games/${params.token}`, {
      headers: {
        Authorization: `Bearer ${getToken()}`,
      },
    });
    if (!res.ok) {
      details = null;
      if (refreshId) {
        clearInterval(refreshId);
      }
      return;
    }

    const game = (await res.json()) as Game;
    const [admin, ...players] = await Promise.all([
      fetchPlayerById(game.admin_id),
      ...game.player_ids.map(fetchPlayerById),
    ]);

    details = {
      game,
      participants: {
        admin,
        players,
      },
    };
  };

  const leaveGame = async () => {
    if (getToken()) {
      await fetch(`/api/games/${params.token}/leave`, {
        headers: {
          Authorization: `Bearer ${getToken()}`,
        },
        method: "POST",
      });
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
        {#if details.game.state === GameState.Initialized}
          <Settings {leaveGame} {refreshGame} {details} token={params.token} />
        {:else if details.game.state === GameState.Started}
          <ActiveGame {leaveGame} />
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
