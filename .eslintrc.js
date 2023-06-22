module.exports = {
	root: true,
	env: {
		browser: true,
		node: true,
	},
	parserOptions: {
		ecmaVersion: 2020,
		sourceType: 'module',
		parser: '@typescript-eslint/parser',
	},
	extends: [
		'plugin:@typescript-eslint/recommended',
		'plugin:nuxt/recommended',
		'plugin:vue/vue3-recommended',
		'plugin:prettier/recommended',
	],
	plugins: ['@typescript-eslint'],
	rules: {
		'vue/script-setup-no-uses-vars': 'off',
		'vue/html-self-closing': [
			'off',
			{
				html: {
					void: 'always',
					normal: 'never',
					component: 'never',
				},
				svg: 'always',
				math: 'always',
			},
		],
		'vue/valid-template-root': 'off',
		'vue/html-indent': 'off',
	},
};
