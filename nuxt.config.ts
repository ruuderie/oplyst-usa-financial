// https://nuxt.com/docs/api/configuration/nuxt-config

export default defineNuxtConfig({
	head: {
		titleTemplate: '%s - Nuxt',
		meta: [
			{ charset: 'utf-8' },
			{ name: 'viewport', content: 'width=device-width, initial-scale=1' },
			// hid is used as unique identifier. Do not use `vmid` for it as it will not work
			{ hid: 'description', name: 'description', content: 'Meta description' },
		],
		metaInfo: {
			script: [{ src: 'https://cdn.jsdelivr.net/npm/vue/dist/vue.js', async: true, defer: true }],
		},
	},
	modules: ['@nuxt/image-edge', '@pinia/nuxt', 'nuxt-icon'],
	pinia: {
		autoImports: [
			['defineStore', 'definePiniaStore'], // import { defineStore as definePiniaStore } from 'pinia'
		],
	},
	publicRuntimeConfig: {
		CMS_API_TOKEN: process.env.CMS_API_TOKEN,
		HELLOWORLD: process.env.HELLOWORLD,
	},
	image: {
		// The screen sizes predefined by `@nuxt/image`:
		screens: {
			xs: 320,
			sm: 640,
			md: 768,
			lg: 1024,
			xl: 1280,
			xxl: 1536,
			'2xl': 1536,
		},
	},
});
