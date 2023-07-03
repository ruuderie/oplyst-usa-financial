<script setup>
import { ref, computed } from 'vue';
const options = [
	{
		label: 'Home',
		name: 'Home',
		children: [
			{ label: 'Overview', name: 'Overview' },
			{ label: 'Values', name: 'Values' },
			{ label: 'Mission', name: 'Mission' },
		],
	},
	{
		label: 'Services',
		name: 'Services',
		children: [
			{ label: 'Real Estate Loans', name: 'Real Estate Loans' },
			{ label: 'Equipment Loans', name: 'Equipment Loans' },
			{ label: 'Lines of Credit', name: 'Lines of Credit' },
			{ label: 'Accounts Receivable Finance', name: 'Accounts Receivable Finance' },
			{ label: 'Startups', name: 'Startups' },
			{ label: 'Merchant Cash Advance', name: 'Merchant Cash Advance' },
		],
	},
	{
		label: 'Industries',
		name: 'Industries',
		children: [
			{ label: 'Real Estate', name: 'Real Estate' },
			{ label: 'Construction', name: 'Construction' },
			{ label: 'Healthcare', name: 'Healthcare' },
			{ label: 'Manufacturing', name: 'Manufacturing' },
			{ label: 'Retail', name: 'Retail' },
			{ label: 'Financial Services', name: 'Financial Services' },
			{ label: 'Technology', name: 'Technology' },
		],
	},
	{
		label: 'Resources',
		name: 'Resources',
		children: [
			{ label: 'Blog', name: 'Blog' },
			{ label: 'Industry News', name: 'Industry News' },
			{ label: 'eBooks/Whitepapers', name: 'eBooks/Whitepapers' },
			{ label: 'Case Studies', name: 'Case Studies' },
		],
	},
];
const transformToSEOName = name => {
	return name
		.toLowerCase()
		.replace(/\s+/g, '-')
		.replace(/[^a-z0-9\-]/g, '');
};

options.forEach(option => {
	option.name = transformToSEOName(option.name);
	option.children.forEach(child => {
		child.name = transformToSEOName(child.name);
	});
});

const activeSubmenu = ref('');
const isActive = ref(false);

const toggleSubmenu = menuName => {
	console.log('menuName', menuName);
	console.log('activeSubmenu.value', activeSubmenu.value);
	activeSubmenu.value = activeSubmenu.value === menuName ? '' : menuName;
};

const toggleMenu = () => {
	console.log('toggleMenu');
	isActive.value = !isActive.value;
	console.log('isActive.value : ', isActive.value);
};
</script>

<template>
	<div>
		<nav class="navbar" role="navigation" aria-label="main navigation">
			<div class="navbar-brand">
				<NuxtLink class="navbar-item" to="/"> </NuxtLink>
				<a role="button" class="navbar-burger" aria-label="menu" aria-expanded="false" @click="toggleMenu">
					<span aria-hidden="true"></span>
					<span aria-hidden="true"></span>
					<span aria-hidden="true"></span>
				</a>
			</div>

			<div id="navbarStandard" class="navbar-menu" :class="{ 'is-active': isActive }">
				<div class="navbar-start">
					<div v-for="menu in options" :key="menu.label" class="navbar-item has-dropdown is-hoverable">
						<NuxtLink class="navbar-link" to="{{menu.name}}"> {{ menu.label }} </NuxtLink>
						<div class="navbar-dropdown">
							<NuxtLink
								v-for="child in menu.children"
								:key="child.label"
								:to="'/' + menu.name.toLowerCase() + '/' + child.name.toLowerCase()"
								class="navbar-item"
								>{{ child.label }}</NuxtLink
							>
						</div>
					</div>
				</div>
			</div>
		</nav>
	</div>
</template>

<style scoped>
.navbar {
	background-color: #c2dbff; /* Goldman Sachs Dark Blue */
}

.navbar-item h2 {
	color: #ffffff; /* White color for the text */
}

.navbar-burger {
	color: #ffffff; /* White color for the burger menu */
}

.navbar-item.has-dropdown:hover .navbar-link {
	color: #d4a017; /* Goldman Sachs Gold for hover state */
}

.navbar-item.has-dropdown:hover .navbar-dropdown,
.navbar-item.has-dropdown.is-active .navbar-dropdown {
	display: block;
}

.v-enter-active,
.v-leave-active {
	transition: opacity 0.5s ease;
}

.v-enter-from,
.v-leave-to {
	opacity: 0;
}
</style>
