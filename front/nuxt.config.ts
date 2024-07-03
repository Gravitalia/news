import { isDevelopment } from "std-env";

export default defineNuxtConfig({
  app: {
    keepalive: true,
    head: {
      charset: "utf-8",
      viewport: "width=device-width,initial-scale=1",
      title: "Gravitalia News",
      meta: [
        { property: "og:type", content: "website" },
        { property: "og:site_name", content: "Gravitalia News" },
        { property: "og:title", content: "Gravitalia News" },
        { property: "og:image", content: "/favicon.webp" },
        {
          name: "og:description",
          content:
            "Explore the latest world news with Gravitalia! Take advantage of summaries and interactive quizzes to stay informed and test your knowledge.",
        },
        { name: "theme-color", content: "#000000" },
        { name: "robots", content: "index, follow" },
        { name: "twitter:card", content: "summary" },
        { name: "twitter:site", content: "@gravitaliaoss" },
        {
          name: "description",
          content:
            "Explore the latest world news with Gravitalia! Take advantage of summaries and interactive quizzes to stay informed and test your knowledge.",
        },
      ],
      link: [{ rel: "manifest", href: "/manifest.json" }],
      script: [
        {
          innerHTML: !isDevelopment
            ? '"serviceWorker"in navigator&&navigator.serviceWorker.register("/sw.js",{scope:"/"});'
            : "",
        },
      ],
      bodyAttrs: {
        class: "dark:bg-zinc-900 dark:text-white",
      },
    },
  },

  ssr: false,
  components: true,
  spaLoadingTemplate: "pages/loading.html",
  sourcemap: isDevelopment,

  modules: [
    "@nuxt/image",
    [
      "@nuxtjs/apollo",
      {
        clients: {
          default: {
            httpEndpoint: "https://testapi.gravitalia.com/graphql",
            httpLinkOptions: {
              credentials: "same-origin",
              httpOnly: false,
            },
          },
        },
      },
    ],
    [
      "@nuxtjs/color-mode",
      {
        preference: "system",
        fallback: "light",
        hid: "color-script",
        globalName: "__NUXT_COLOR_MODE__",
        componentName: "ColorScheme",
        classPrefix: "",
        classSuffix: "",
        storageKey: "mode",
      },
    ],
    [
      "@nuxtjs/i18n",
      {
        defaultLocale: "us",
        strategy: "prefix",
        customRoutes: "config",
        lazy: true,
        langDir: "locales",
        detectBrowserLanguage: {
          useCookie: true,
          cookieKey: "locale",
          redirectOn: "root",
          fallbackLocale: "en",
          alwaysRedirect: true,
        },
        locales: [
          {
            code: "us",
            iso: "en-US",
            file: "en-US.json",
            name: "English",
          },
          {
            code: "fr",
            iso: "fr-FR",
            file: "fr-FR.json",
            name: "Français",
          },
        ],
        pages: {
          // Implement this later due to [`localePath`] limitations.
          /*mth: {
            en: "/mth",
            fr: "/qcm",
          }*/
        },
        baseUrl: "https://news.gravitalia.com",
      },
    ],
    "@nuxtjs/tailwindcss",
    "@nuxt/eslint",
  ],

  devtools: { enabled: true },

  nitro: {
    preset: "cloudflare_pages",
  },

  experimental: {
    headNext: true,
    payloadExtraction: false,
    renderJsonPayloads: true,
  },

  vue: {
    propsDestructure: true,
  },

  /*typescript: {
    typeCheck: true,
  },*/
});
