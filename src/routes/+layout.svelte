<script>
	import '../app.css';
</script>

<section class="flex-col">
	<nav class="flex items-center justify-end px-4 w-full h-10 border">
		<ul class="flex gap-x-4">
			<li><a href="/">Home</a></li>
			<li><a href="/" class="bg-red-500 rounded-md p-2 text-white text-center mb-3">Back</a></li>
			<li>|</li>
			<li><a href="/settings">Settings</a></li>
		</ul>
	</nav>
	<slot />
</section>
