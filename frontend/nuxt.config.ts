// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: '2025-07-15',
  ssr: false,
  devtools: { enabled: true },
  css: ['~/assets/css/main.css'],
  modules: [
    '@nuxtjs/tailwindcss',
    '@nuxtjs/color-mode'
  ],
  colorMode: {
    classSuffix: '' // Agar sesuai dengan class="dark" di Tailwind
  },
  runtimeConfig: {
    public: {
      // Masukkan IP backend Rust Anda di sini
      apiBase: 'http://192.168.101.8:8080'
    }
  },
})
