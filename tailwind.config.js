/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    relative: true,
    files: ["*.html", "./src/**/*.rs"],
    transform: {
      rs: (content) => content.replace(/class:([-a-zA-Z0-9_/:\[\]]+)=/g, ' $1 '),
    },
  },
  theme: {
    extend: {},
  },
  plugins: [],
}
