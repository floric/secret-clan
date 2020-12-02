<script lang="typescript">
  import Dialog from "../components/layout/Dialog.svelte";
  import DialogHeader from "../components/headers/DialogHeader.svelte";
  import PrimaryButton from "../components/buttons/Primary.svelte";
  import type { Game, GameDetails } from "../types/Game";
  import type { PublicPlayer } from "../types/Player";
  import InternalLink from "../components/buttons/InternalLink.svelte";
  import ActionRow from "../components/buttons/ActionRow.svelte";
  import { getToken } from "../utils/auth";

  export let params: { token?: string } = {};

  let details: GameDetails | null = null;
  const knownPlayers: Map<string, PublicPlayer> = new Map();

  const fetchPlayerById = async (id: string): Promise<PublicPlayer | null> => {
    if (knownPlayers.has(id)) {
      return knownPlayers.get(id);
    }
    const res = await fetch(`/api/players/${id}`);
    if (!res.ok) {
      return null;
    }

    const p = (await res.json()) as PublicPlayer;
    knownPlayers.set(p.id, p);
    return p;
  };

  const fetchGameByToken = async () => {
    const res = await fetch(`/api/games/${params.token}`, {
      headers: {
        Authorization: `Bearer ${getToken()}`,
      },
    });
    if (!res.ok) {
      return null;
    }

    const game = (await res.json()) as Game;
    const players = await Promise.all(game.player_ids.map(fetchPlayerById));
    const admin = await fetchPlayerById(game.admin_id);

    details = {
      game,
      participants: {
        admin,
        players,
      },
    };
  };

  const fetchGamePeriodically = async () => {
    await fetchGameByToken();
    setInterval(() => {
      fetchGameByToken();
    }, 1000);
  };
</script>

<Dialog>
  <DialogHeader>Lobby</DialogHeader>
  <div class="mb-8 flex items-center">
    <div>Token</div>
    <div class="font-extrabold px-4 py-2">{params.token.toUpperCase()}</div>
  </div>

  <div class="mb-4">
    {#await fetchGamePeriodically()}
      <p>Loading game</p>
    {:then _}
      {#if details !== null}
        <div class="grid md:grid-cols-3 grid-cols-1 gap-8">
          <div class="md:col-span-2">
            <p class="mb-4">Hier ist viel zu tun:</p>
            <ul class="ml-6 list-disc mb-6">
              <li>Spieleinstellungen</li>
              <li>Fancy UI</li>
            </ul>
          </div>
          <div class="grid gap-4">
            <div>
              <div class="font-bold">Admin</div>
              <div>{details.participants.admin.name}</div>
            </div>
            <div>
              <div class="font-bold">Players</div>
              <ul>
                {#each details.participants.players as p}
                  <li>{p.name}</li>
                {/each}
              </ul>
            </div>
          </div>
        </div>
        <ActionRow>
          <PrimaryButton>Start</PrimaryButton>
          <InternalLink href="/games">Leave Game</InternalLink>
        </ActionRow>
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
