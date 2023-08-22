// https://nuxt.com/docs/api/configuration/nuxt-config
import { themes } from './styles/colors';

export default defineNuxtConfig({
	target: 'static',
	alias: {
		assets: "/<rootDir>/assets",
		components: "/<rootDir>/components",
	},
	modules: ['@nuxtjs/tailwindcss'],
	app: {
		head: {
			title: 'Commercial Loans & Finance Solutions | Oplyst USA | (860) 317-6972',
			meta: [
				{ charset: 'utf-8' },
				{ name: 'viewport', content: 'width=device-width, initial-scale=1' },
				{
					hid: 'description',
					name: 'description',
					content:
						"Empower your business growth with Oplyst's diverse commercial financing and loan products. Serving nationwide from Stamford, CT. Call (860) 317-6972 to get started!",
				},
				{ hid: 'og:locale', property: 'og:locale', content: 'en_US' },
				{ hid: 'og:type', property: 'og:type', content: 'website' },
				{
					hid: 'og:title',
					property: 'og:title',
					content:
						'Commercial Loans & Finance Solutions in Stamford, CT | Serving Nationwide | Commercial Loans | (860) 317-6972',
				},
				{
					hid: 'og:description',
					property: 'og:description',
					content:
						"Empower your business growth with Oplyst's diverse commercial financing and loan products. Serving nationwide from Stamford, CT. Call (860) 317-6972 to get started!",
				},
				{ hid: 'og:url', property: 'og:url', content: 'https://oplystusa.com/' },
				{ hid: 'og:site_name', property: 'og:site_name', content: 'Oplyst International, LLC' },
				{ hid: 'twitter:card', name: 'twitter:card', content: 'summary_large_image' },
				{ hid: 'twitter:label1', name: 'twitter:label1', content: 'Est. reading time' },
				{ hid: 'twitter:data1', name: 'twitter:data1', content: '3 minutes' },
			],
			link: [{ rel: 'canonical', href: 'https://oplystusa.com/' }],
			metaInfo: {
				script: [{ src: 'https://cdn.jsdelivr.net/npm/vue/dist/vue.js', async: true, defer: true }],
			},
		},
		modules: ['@nuxt/image-edge', '@pinia/nuxt', 'nuxt-icons', '@nuxt/image'],
		pinia: {
			autoImports: [
				['defineStore', 'definePiniaStore'], // import { defineStore as definePiniaStore } from 'pinia'
			],
		},
		publicRuntimeConfig: {
			CMS_API_TOKEN: process.env.CMS_API_TOKEN,
			HELLOWORLD: process.env.HELLOWORLD,
		},
		
	},
	image: {
		inject: true,
		quality: 80,
		format: ['webp','png','jpg','jpeg'],
		screens: {
		xs: 320,
		sm: 640,
		md: 768,
		lg: 1024,
		xl: 1280,
		xxl: 1536,
		'2xl': 1536
		},
		domains: ['nuxtjs.org', 'images.unsplash.com'],
		presets: {
		avatar: {
			modifiers: {
			format: 'jpg',
			width: 50,
			height: 50
			}
		}
		},
		densities: [1, 2, 3],
		dir: 'assets/images',
		alias: {
		unsplash: 'https://images.unsplash.com'
		}
	},
	css: [
		'~/assets/goldmanSans.css'
	  ]
	
	});
