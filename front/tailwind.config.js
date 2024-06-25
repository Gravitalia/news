/** @type {import("tailwindcss").Config} */
module.exports = {
  content: [
    "./components/**/*.{js,vue,ts}",
    "./layouts/**/*.vue",
    "./pages/**/*.vue",
    "./plugins/**/*.{js,ts}",
    "./app.vue",
    "./error.vue",
    "./nuxt.config.{js,ts}",
  ],
  theme: {
    extend: {
      keyframes: {
        slideDown: {
          "0%": { opacity: "0", height: "0", padding: "0" },
          "100%": { opacity: "1", height: "auto" },
        },
        showContent: {
          "0%": { opacity: "0", height: "0" },
          "100%": { opacity: "1", height: "auto" },
        },
      },
      animation: {
        "slide-down": "slideDown 0.3s ease-in-out",
        "show-content": "showContent 0.6s 0.2s forwards",
      },
    },
  },
  plugins: [],
};
