<script lang="typescript">
  import { push } from "svelte-spa-router";
  import Dialog from "../components/layout/Dialog.svelte";
  import DialogHeader from "../components/headers/DialogHeader.svelte";
  import PrimaryButton from "../components/buttons/Primary.svelte";
  import SecondaryButton from "../components/buttons/Secondary.svelte";
  import type { Game, GameDetails } from "../types/Game";
  import type { PublicPlayer } from "../types/Player";
  import InternalLink from "../components/buttons/InternalLink.svelte";
  import ActionRow from "../components/buttons/ActionRow.svelte";
  import { getClaims, getToken } from "../utils/auth";
  import TextInput from "../components/inputs/TextInput.svelte";
  import Label from "../components/inputs/Label.svelte";

  export let params: { token?: string } = {};

  let details: GameDetails | null = null;
  let refreshId: number | null = null;
  const claims = getClaims();

  const fetchPlayerById = async (id: string): Promise<PublicPlayer | null> => {
    const res = await fetch(`/api/players/${id}`);
    if (!res.ok) {
      return null;
    }

    return (await res.json()) as PublicPlayer;
  };

  const fetchGameByToken = async () => {
    const res = await fetch(`/api/games/${params.token}`, {
      headers: {
        Authorization: `Bearer ${getToken()}`,
      },
    });
    if (!res.ok) {
      details = null;
      return;
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
    await fetchGameByToken();
    refreshId = setInterval(() => {
      fetchGameByToken();
    }, 3000);
  };

  const onChangeName = async (ev: any) => {
    const name = ev?.target?.value;
    if (!name) {
      return;
    }

    await fetch(`/api/players/${claims.sub}`, {
      headers: {
        Authorization: `Bearer ${getToken()}`,
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ name }),
      method: "POST",
    });
  };
</script>

<Dialog>
  <DialogHeader>Lobby</DialogHeader>
  <div class="mb-8 flex items-center">
    <div>Token</div>
    <div
      class="font-extrabold mx-4 px-3 py-2 rounded-md bg-gray-200 border-black">
      {params.token.toUpperCase()}
    </div>
  </div>

  <div>
    {#await fetchGamePeriodically()}
      <p>Loading game</p>
    {:then _}
      {#if details !== null}
        <div class="grid md:grid-cols-3 grid-cols-1 gap-8 mb-8">
          <div class="md:col-span-2">
            <h4 class="font-bold mb-4">Settings</h4>
            <div class="max-w-xs">
              <Label target="name">Your Name</Label>
              <TextInput
                id="name"
                placeholder="Name"
                value={claims.name}
                on:change={onChangeName} />
            </div>
          </div>
          <div>
            <h4 class="font-bold mb-4">Players</h4>
            <ul>
              <li>
                {details.participants.admin.name}
                <span class="font-bold">(Admin)</span>
              </li>
              {#each details.participants.players as p}
                <li>{p.name}</li>
              {/each}
            </ul>
          </div>
        </div>
        <ActionRow>
          <PrimaryButton>Start</PrimaryButton>
          <SecondaryButton on:click={leaveGame}>Leave Game</SecondaryButton>
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
