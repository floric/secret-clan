<script lang="typescript">
  import Dialog from "../components/layout/Dialog.svelte";
  import DialogHeader from "../components/headers/DialogHeader.svelte";
  import PrimaryButton from "../components/buttons/Primary.svelte";
  import type { Game } from "../types/Game";
  import InternalLink from "../components/buttons/InternalLink.svelte";
  import ActionRow from "../components/buttons/ActionRow.svelte";
  import { getToken } from "../utils/auth";

  export let params: { token?: string } = {};

  async function fetchGameByToken(): Promise<Game | null> {
    const res = await fetch(`/api/games/${params.token}`, {
      headers: {
        Authorization: `Bearer ${getToken()}`,
      },
    });
    if (res.ok) {
      return (await res.json()) as Game;
    } else {
      return null;
    }
  }

  async function deleteGameByToken(): Promise<boolean> {
    const res = await fetch(`/api/games/${params.token}`, { method: "DELETE" });
    return res.ok;
  }

  const fetchGamePromise = fetchGameByToken();
</script>

<Dialog>
  <DialogHeader>Lobby</DialogHeader>
  <div class="mb-8 flex items-center">
    <div>Token</div>
    <div class="font-extrabold px-4 py-2">{params.token.toUpperCase()}</div>
  </div>

  <div class="mb-4">
    {#await fetchGamePromise}
      <p>Loading game</p>
    {:then game}
      {#if game !== null}
        <div class="grid md:grid-cols-3 grid-cols-1 gap-8">
          <div class="md:col-span-2">
            <p class="mb-4">Hier ist viel zu tun:</p>
            <ul class="ml-6 list-disc mb-6">
              <li>
                Prüfung auf gültiges Spiel und laden des zugehörigen Spieles
              </li>
              <li>Anzeige beigetretener Spieler</li>
              <li>Spieleinstellungen</li>
              <li>Fancy UI</li>
            </ul>
          </div>
          <div>Users</div>
          <ul>
            <li>Anzeige der Liste der Spieler</li>
          </ul>
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
