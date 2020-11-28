<script lang="typescript">
  import { push } from "svelte-spa-router";
  import type { Game } from "../types/Game";
  import type { Player } from "../types/Player";
  import ActionRow from "../components/buttons/ActionRow.svelte";
  import PrimaryButton from "../components/buttons/Primary.svelte";
  import SecondaryButton from "../components/buttons/Secondary.svelte";
  import Dialog from "../components/layout/Dialog.svelte";
  import Divider from "../components/layout/Divider.svelte";
  import DialogHeader from "../components/headers/DialogHeader.svelte";
  import InternalLink from "../components/buttons/InternalLink.svelte";

  let inputToken = "";
  let inputName = "";

  async function createGame() {
    try {
      const res = await fetch("/api/games", {
        method: "PUT",
      });

      const newGame = (await res.json()) as Game;
      push(`/games/${newGame.token}`);
    } catch (err) {
      // TODO Handle all API errors in a generic way
    }
  }

  async function attendGame() {
    try {
      const res = await fetch(`/api/games/${inputToken}/attend`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          name: inputName,
        }),
      });

      if (!res.ok) {
        return;
      }

      const player = (await res.json()) as Player;
      window.localStorage.setItem("ACCESS_TOKEN", player.user_token);

      push(`/games/${inputToken?.trim()}`);
    } catch (err) {
      // TODO Handle all API errors in a generic way
    }
  }
</script>

<Dialog>
  <DialogHeader>Start Game</DialogHeader>
  <SecondaryButton on:click={createGame}>Create new game</SecondaryButton>

  <Divider><span slot="text">or</span></Divider>

  <form on:submit|preventDefault={attendGame}>
    <div class="grid grid-cols-1 md:grid-cols-2 mb-6 gap-4">
      <input
        id="token"
        name="token"
        placeholder="Token"
        bind:value={inputToken}
        class="text-sm sm:text-base placeholder-gray-500 rounded-lg border border-gray-400 w-full py-2 px-3 focus:outline-none focus:border-blue-400" />
      <input
        id="name"
        name="name"
        placeholder="Name"
        bind:value={inputName}
        class="text-sm sm:text-base placeholder-gray-500 rounded-lg border border-gray-400 w-full py-2 px-3 focus:outline-none focus:border-blue-400" />
    </div>
    <ActionRow>
      <PrimaryButton>Attend</PrimaryButton>
      <InternalLink href="/">Back to Start</InternalLink>
    </ActionRow>
  </form>
</Dialog>
