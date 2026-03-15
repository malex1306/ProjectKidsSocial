/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: [
      "./src/**/*.rs",
      "./*.html"
    ],
    // Transform für Leptos-spezifische Syntax (class:name=bool)
    transform: {
      rs: (content) => content.replace(/(?:class|id):/g, ""),
    },
  },
  theme: {
    extend: {
      fontFamily: {
        heading: ['Outfit', 'sans-serif'],
        body: ['Inter', 'sans-serif'],
      },
    },
  },
  plugins: [],
}
