<script lang="typescript">
  import PrimaryButton from "../components/buttons/Primary.svelte";
  import SecondaryButton from "../components/buttons/Secondary.svelte";
  import Dialog from "../components/layout/Dialog.svelte";
  import Divider from "../components/layout/Divider.svelte";
  import DialogHeader from "../components/headers/DialogHeader.svelte";
  import { link } from "svelte-spa-router";
</script>

<Dialog>
  <DialogHeader>Start Game</DialogHeader>
  <SecondaryButton>Create new game</SecondaryButton>

  <Divider><span slot="text">or</span></Divider>

  <div class="flex flex-col mb-6">
    <label
      for="token"
      class="mb-1 text-xs sm:text-sm tracking-wide text-gray-600">Game token:</label>
    <input
      id="token"
      name="token"
      class="text-sm sm:text-base placeholder-gray-500 rounded-lg border border-gray-400 w-full py-2 focus:outline-none focus:border-blue-400" />
  </div>
  <PrimaryButton>Attend</PrimaryButton>
  <div class="flex items-center mt-4">
    <div class="flex ml-auto">
      <a
        href="/"
        use:link
        class="inline-flex text-xs sm:text-sm text-blue-500 hover:text-blue-700">Back
        to Start</a>
    </div>
  </div>
</Dialog>
