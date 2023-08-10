<script setup lang="ts">
const props = defineProps({
	pageTitle: {
		type: String,
		required: true,
	},
	subTitle: {
		type: String,
		required: true,
	},
	industryName: {
		type: String,
		required: false,
	},
	industryDescription: {
		type: String,
		required: true,
	},
	whyItems: {
		type: Array,
		required: true,
		default: () => [],
	},
	servicesItems: {
		type: Array,
		required: true,
		default: () => [],
	},
	caseStudies: {
		type: Object,
		required: true,
		default: () => [],
	},
	faq: {
		type: String,
		required: true,
		default: () => '',
	},
	callToAction: {
		type: String,
		required: false,
	},
});

const name = ref('');
const email = ref('');
const phone = ref('');

const submitForm = () => {
	console.log('name', name.value);
	console.log('email', email.value);
	console.log('phone', phone.value);
};
</script>

<template>
	<div class="is-responsive is-mobile">
		<NuxtLayout>
			<section class="hero is-small is-primary is-fluid">
				<div class="hero-body is-fluid">
					<div class="my-3 has-text-centered">
						<h1 class="title">{{ pageTitle }}</h1>
						<p class="subtitle">{{ subTitle }}</p>
					</div>
				</div>
			</section>
			<section class="container is-fluid">
				<div class="columns">
					<div class="column is-three-quarters">
						<div class="content is-medium">
							<p>{{ industryDescription }}</p>
							<div v-if="whyItems.length > 0">
								<h2>Why Choose Our Financial Solutions for {{ industryName }}</h2>
								<ul>
									<li v-for="item in whyItems">
										<p>{{ item.description }}</p>
									</li>
								</ul>
							</div>
							<ul>
								<li v-for="item in servicesItems">
									<h3>{{ item.title }}</h3>
									<p>{{ item.description }}</p>
								</li>
							</ul>
							<div v-if="caseStudies.length > 0">
								<h2>Case Studies/Testimonials</h2>
								<p>{{ caseStudies }}</p>
							</div>
							<h2>FAQ</h2>
							<p>{{ faq }}</p>
						</div>
					</div>
					<div class="column">
						<section class="is-fluid">
							<p>{{ callToAction }}</p>
							<div class="field">
								<label class="label">Name</label>
								<div class="control">
									<input v-model="name" class="input" type="text" />
								</div>
							</div>
							<div class="field">
								<label class="label">Email</label>
								<div class="control">
									<input v-model="email" class="input" type="email" />
								</div>
							</div>
							<div class="field">
								<label class="label">Phone</label>
								<div class="control">
									<input v-model="phone" class="input" type="tel" />
								</div>
							</div>
							<div class="control">
								<button class="button is-primary" @click="submitForm">Submit</button>
							</div>
						</section>
					</div>
				</div>
			</section>
			<NuxtPage />
		</NuxtLayout>
	</div>
</template>
<style scoped>

/* Base colors */
:root {
    --primary-color: #003366;  /* As taken from the navbar */
    --secondary-color: #f4f4f4; 
    --accent-color: #d4a017;  /* Gold color for accents */
    --text-color: #ffffff;
    --border-color: #e0e0e0;  /* As taken from the navbar */
}

body {
    font-family: 'Arial', sans-serif;
    color: var(--text-color);
    background-color: var(--secondary-color);
}

/* Titles and subtitles */
.title, .subtitle {
    color: var(--primary-color);
}

/* Buttons */
.button {
    color: var(--text-color);
    background-color: var(--primary-color);
    transition: background-color 0.3s ease;
}

.button.is-primary:hover {
    background-color: var(--accent-color);
}

/* Input fields */
.input {
    border-color: var(--border-color);
    background-color: var(--secondary-color);
    color: var(--primary-color);
}

/* Content styles */
.content {
    line-height: 1.6;
}

.content h2, .content h3 {
    margin-top: 1em;
    color: var(--primary-color);
}

.content p {
    margin-top: 0.5em;
    color: var(--primary-color);
}

/* Layout and structure */
.is-fluid {
    padding: 1em;
}

.field label {
    font-weight: bold;
    margin-bottom: 0.5em;
    display: block;
    color: var(--primary-color);
}

/* Hero section */
.hero.is-small.is-primary {
    background-color: var(--primary-color);
    border-bottom: 1px solid var(--border-color);
    color: var(--text-color);
}

.hero-body {
    padding: 2em 1em;
}

</style>

