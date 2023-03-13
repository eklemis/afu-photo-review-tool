/** @type {import('tailwindcss').Config} */
module.exports = {
	content: ['./src/**/*.{html,js,svelte,ts}'],
	theme: {
		fontFamily: {
			sans: ['Arial', 'Helvetica', 'sans-serif']
		},
		extend: {
			colors: {
				'base-blue': '#405CF5',
				'white-blue': '#DADEF3',
				'font-blue': '#0087ED',
				'text-blue-darker': '#405CF5'
			}
		}
	},
	plugins: []
};
