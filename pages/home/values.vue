<script setup>
import { ref, onMounted } from 'vue';

const post = {
	summary: 'This is a summary',
	title: 'This is a title',
	featured_image: 'https://example.com/image.png',
};
</script>
<template>
	<div class="is-responsive is-mobile">
		<NuxtLayout>
			<section class="hero is-small is-primary is-fluid">
				<div class="hero-body is-fluid">
					<div class="my-3 has-text-centered">
						<p class="title">Values of Oplyst International, LLC</p>
						<p class="subtitle">
							Oplyst International isn't just a company – we're a mindset, a disruptive force. Our
							strength lies in our unique blend of technology, logistics, and financial advisory. We hold
							dear the values of quality, simplicity, and client satisfaction, ensuring that these
							principles guide us in providing holistic solutions to complex challenges. With Oplyst,
							you're not just getting a service – you're joining a forward-thinking community of
							innovators and pioneers.
						</p>
					</div>
				</div>
			</section>
		</NuxtLayout>
	</div>
</template>

<style></style>
