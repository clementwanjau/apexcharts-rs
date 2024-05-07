module.exports = {
    content: [
        "./src/**/*.rs",
        "./index.html",
        "./src/**/*.html",
        "./assets/**/*.css",
        "./assets/**/*.js",
        "./node_modules/flowbite/**/*.js",
    ],
    theme: {
        fontFamily: {
            sans: ['Roboto', 'Inter', 'sans-serif']
        },
        extend: {
            height:{
                '18': '4.5rem',
            }
        }
    },
    variants: {},
    plugins: [
    ],
};
