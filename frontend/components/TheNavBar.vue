<script setup>
import { ref, computed } from 'vue';
const options = [
	{
		label: 'Our Company',
		name: 'Our Company',
		children: [
			{ label: 'Overview', name: 'Overview' },
			{ label: 'Values', name: 'Values' },
			{ label: 'Mission', name: 'Mission' },
		],
	},
	{
		label: 'Partner with Us',
		name: 'Partner with Us',
		children: [			
			{ label: 'Brokers', name: 'Brokers' },
			{ label: 'Lenders', name: 'Lenders' },
		]
	},
	{
		label: 'Services',
		name: 'Services',
		children: [
			{ label: 'Commercial Mortgages', name: 'Commercial Mortgages' },
			{ label: 'Equipment Loans', name: 'Equipment Loans' },
			{ label: 'Lines of Credit', name: 'Lines of Credit' },
			{ label: 'Invoice Financing', name: 'Invoice Financing' },
			{ label: 'Startups', name: 'Startups' },
			{ label: 'SBA Loans', name: 'SBA Loans' },
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
			{ label: 'Logistics & Trucking', name: 'Logistics' },
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
	  <nav class="navbar is-flex-direction-row	">
		<div class="container">
		  <div class="navbar-brand">
			<NuxtLink class="navbar-item" to="/">
			  <img src="@/assets/Oplyst_International_White_logo_800x600.png" alt="Logo" class="navbar-logo"/>
			</NuxtLink>
			<a role="button" class="navbar-burger" aria-label="menu" aria-expanded="false" @click="toggleMenu">
			  <span aria-hidden="true"></span>
			  <span aria-hidden="true"></span>
			  <span aria-hidden="true"></span>
			</a>
		  </div>
  
		  <div id="navbarStandard" class="navbar-menu" :class="{ 'is-active': isActive }">
			<div class="navbar-end">
				<div 
					v-for="menu in options" 
					:key="menu.label" 
					class="navbar-item" 
					:class="{ 'has-dropdown is-hoverable': menu.children && menu.children.length > 0 }"
				>
					<NuxtLink class="navbar-link" :to="'/' + menu.name.toLowerCase()"> {{ menu.label }} </NuxtLink>
					<div v-if="menu.children && menu.children.length > 0" class="navbar-dropdown">
						<NuxtLink
							v-for="child in menu.children"
							:key="child.label"
							:to="'/' + menu.name.toLowerCase() + '/' + child.name.toLowerCase()"
							class="navbar-item"
						>
							{{ child.label }}
						</NuxtLink>
					</div>
				</div>
			</div>
		  </div>
		</div>
	  </nav>
  </template>
  

<style scoped>
	.navbar {
		border: 2px solid #FF5733;
		background-color: #003366; /* Dark blue, common in finance industries for trustworthiness */
		border-bottom: 1px solid #e0e0e0; /* A subtle border for separation */
	}
	.navbar-logo {
		max-height: 10.5rem;
	}
	.navbar-item h2, .navbar-link, .navbar-item {
		color: #ffffff; /* White color for the text for contrast against the dark navbar */
		font-size: 1.125rem;
	}
	
	.navbar-burger {
		color: #ffffff; /* White color for the burger menu */
	}
	
	.navbar-link:hover, .navbar-item:hover {
		background-color: #005699; /* Slightly lighter blue for hover effect */
	}
	
	.navbar-item.has-dropdown:hover .navbar-link,
	.navbar-item.has-dropdown.is-active .navbar-link {
		color: #d4a017; /* A touch of gold for active/hovered dropdowns */
		background-color: #005699; /* Slightly lighter blue for hover effect */
	}
	
	.navbar-dropdown {
		background-color: #004882; /* Even darker shade for the dropdown to distinguish it from the main bar */
		border: none; /* Remove any default borders */
	}
	
	.navbar-dropdown .navbar-item:hover {
		background-color: #0066aa; /* A bit lighter blue for hover effect in dropdowns */
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
