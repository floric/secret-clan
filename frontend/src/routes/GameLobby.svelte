<script lang="typescript">
  import { link } from "svelte-spa-router";
  import Dialog from "../components/layout/Dialog.svelte";
  import DialogHeader from "../components/headers/DialogHeader.svelte";
  import PrimaryButton from "../components/buttons/Primary.svelte";
  import type { Game } from "../types/Game";

  export let params: { token?: string } = {};

  async function fetchGameWithToken(): Promise<Game | null> {
    const res = await fetch(`/api/games/${params.token}`);
    if (res.ok && res.status === 200) {
      return (await res.json()) as Game;
    } else {
      return null;
    }
  }

  const fetchPromise = fetchGameWithToken();
</script>

<Dialog>
  <DialogHeader>Lobby</DialogHeader>
  <p class="mb-8">
    Token:
    <span class="font-extrabold">{params.token.toUpperCase()}</span>
  </p>

  <div class="mb-4">
    {#await fetchPromise}
      <p>Loading game</p>
    {:then game}
      {#if game !== null}
        <p class="mb-4">Hier ist viel zu tun:</p>
        <ul class="ml-6 list-disc mb-6">
          <li>Prüfung auf gültiges Spiel und lande des zugehörigen Spieles</li>
          <li>Anzeige beigetretener Spieler</li>
          <li>Spieleinstellungen</li>
          <li>Fancy UI</li>
        </ul>
        <PrimaryButton>Start</PrimaryButton>
      {:else}
        <p>Game doesn't exist.</p>
      {/if}
    {:catch error}
      <p style="color: red">Loading game has failed</p>
    {/await}
  </div>

  <div class="flex items-center">
    <div class="flex ml-auto">
      <a
        href="/games"
        use:link
        class="inline-flex text-xs sm:text-sm text-blue-500 hover:text-blue-700">Back
        Games</a>
    </div>
  </div>
</Dialog>
