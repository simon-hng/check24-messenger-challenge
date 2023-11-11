<script lang="ts">
	import { userStore } from '$lib/stores';
	import Navbar from '$lib/components/navbar.svelte';
	import Fa from 'svelte-fa';
	import { faRightFromBracket } from '@fortawesome/free-solid-svg-icons';
	import LoginForm from './loginForm.svelte';
	import RegisterForm from './registerForm.svelte';
</script>

<Navbar />

{#if $userStore}
	<div class="p-4 space-y-4">
		<p>You're logged in as {$userStore.id}</p>
		<button class="btn variant-filled gap-2" on:click={() => userStore.logout()}>
			<Fa icon={faRightFromBracket} />
			Logout</button
		>
	</div>
{:else}
	<LoginForm />
	<RegisterForm />
{/if}
