<script lang="ts">
	import { createQuery, useQueryClient } from '@tanstack/svelte-query';
	import axios from 'axios';
	import { createForm } from 'svelte-forms-lib';

	const { form, handleChange, handleSubmit } = createForm({
		initialValues: {
			username: ''
		},

		onSubmit: async (values) => {
			let account_id = await axios.post(`auth/login/${encodeURIComponent(values.username)}`);

			queryClient.invalidateQueries(['whoami']);

			let socket: WebSocket;
			socket = new WebSocket('ws://localhost:8080/message/receive');
			socket.onopen = () => {
				socket.send(JSON.stringify(account_id));
			};
		}
	});

	const queryClient = useQueryClient();

	const whoamiQuery = createQuery<string, Error>({
		queryKey: ['whoami'],
		queryFn: async () => await axios.get('auth/whoami').then((res) => res.data)
	});
</script>

<svelte:head>
	<title>Home</title>
	<meta name="description" content="Svelte demo app" />
</svelte:head>

<section>
	<h1 class="text-center mt-32 text-4xl font-serif">
		Aller Anfang ist schwer. Anfangen ist einfach, Beharrlichkeit eine Kunst
	</h1>

	{#if $whoamiQuery.isSuccess}
		<h2 class="text-center">
			{$whoamiQuery.data}
		</h2>
	{/if}

	<form class="space-y-2 m-8" on:submit={handleSubmit}>
		<label>
			Username
			<input
				class="input py-2 px-4 mt-2"
				placeholder="simonhng"
				on:change={handleChange}
				bind:value={$form.username}
			/>
		</label>

		<button type="submit" class="btn variant-filled w-full">login</button>
	</form>
</section>
