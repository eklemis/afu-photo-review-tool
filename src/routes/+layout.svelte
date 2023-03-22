<script>
	import '../app.css';
</script>

<section class="flex-col font-sans">
	<nav class="flex items-center justify-between px-4 w-full h-12 border-[1px] border-r-gray-500">
		<ul class="flex gap-x-4">
			<li><a href="/">Home</a></li>
			<li class=" text-gray-300">|</li>
			<li><a href="/settings">Settings</a></li>
		</ul>
		<a
			class="flex self-start items-center content-center justify-center mt-1 w-28 h-10 bg-red-500 text-white rounded"
			href="/progress">See progress</a
		>
	</nav>
	<slot />
</section>
