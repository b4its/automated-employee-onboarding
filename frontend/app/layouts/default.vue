<template>
  <div class="min-h-screen bg-gray-50 dark:bg-gray-950 text-gray-900 dark:text-gray-100 transition-colors duration-300 font-sans flex">
    
    <div class="md:hidden fixed top-0 w-full z-40 bg-white/80 dark:bg-gray-900/80 backdrop-blur border-b border-gray-200 dark:border-gray-800 flex items-center justify-between p-4">
      <div class="flex items-center gap-3">
        <div class="flex h-8 w-8 items-center justify-center rounded-lg bg-blue-600 text-white font-bold shadow-md text-xs">HR</div>
        <h1 class="text-lg font-bold tracking-tight">Oryphem</h1>
      </div>
      <button @click="isSidebarOpen = true" class="p-2 text-gray-600 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-800 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500">
        <svg class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
        </svg>
      </button>
    </div>

    <div 
      v-if="isSidebarOpen" 
      @click="isSidebarOpen = false" 
      class="fixed inset-0 z-40 bg-gray-900/60 backdrop-blur-sm md:hidden transition-opacity"
    ></div>

    <aside
      :class="[
        isSidebarOpen ? 'translate-x-0' : '-translate-x-full md:translate-x-0',
        'fixed inset-y-0 left-0 z-50 w-64 bg-white dark:bg-gray-900 border-r border-gray-200 dark:border-gray-800 flex flex-col transition-transform duration-300 ease-in-out'
      ]"
    >
      <div class="flex h-16 shrink-0 items-center justify-between px-6 border-b border-gray-200 dark:border-gray-800">
        <div class="flex items-center gap-3">
          <div class="flex h-9 w-9 items-center justify-center rounded-xl bg-blue-600 text-white font-bold shadow-lg">HR</div>
          <h1 class="text-xl font-bold tracking-tight truncate">Oryphem HRIS</h1>
        </div>
        <button @click="isSidebarOpen = false" class="md:hidden p-2 text-gray-400 hover:text-gray-600 dark:hover:text-white focus:outline-none">
          <svg class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
            <path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd" />
          </svg>
        </button>
      </div>

      <nav class="flex-1 px-4 py-6 space-y-2 overflow-y-auto">
        <p class="px-2 text-xs font-semibold text-gray-400 dark:text-gray-500 uppercase tracking-wider mb-2">Menu Utama</p>
        
        <NuxtLink 
          to="/" 
          @click="isSidebarOpen = false"
          active-class="bg-blue-50 dark:bg-blue-900/30 text-blue-700 dark:text-blue-400 font-semibold" 
          class="flex items-center gap-3 rounded-xl px-4 py-3 text-sm font-medium text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-800/80 transition-all group"
        >
          <svg class="h-5 w-5 text-gray-400 group-hover:text-blue-600 dark:group-hover:text-blue-400" :class="{ 'text-blue-600 dark:text-blue-400': $route.path === '/' }" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6" />
          </svg>
          HR Admin
        </NuxtLink>

        <NuxtLink 
          to="/candidate/ob_123" 
          @click="isSidebarOpen = false"
          active-class="bg-blue-50 dark:bg-blue-900/30 text-blue-700 dark:text-blue-400 font-semibold" 
          class="flex items-center gap-3 rounded-xl px-4 py-3 text-sm font-medium text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-800/80 transition-all group"
        >
          <svg class="h-5 w-5 text-gray-400 group-hover:text-blue-600 dark:group-hover:text-blue-400" :class="{ 'text-blue-600 dark:text-blue-400': $route.path.includes('/candidate') }" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
          </svg>
          Portal Kandidat
        </NuxtLink>
      </nav>

      <div class="p-4 border-t border-gray-200 dark:border-gray-800">
        <div class="flex items-center justify-between rounded-xl bg-gray-50 dark:bg-gray-800/50 p-3 border border-transparent dark:border-gray-700/50">
          <span class="text-sm font-medium text-gray-700 dark:text-gray-300">Tema Aplikasi</span>
          <button 
            @click="toggleTheme" 
            class="flex items-center justify-center rounded-lg p-2.5 bg-white dark:bg-gray-700 shadow-sm border border-gray-200 dark:border-gray-600 hover:bg-gray-100 dark:hover:bg-gray-600 transition-all focus:outline-none focus:ring-2 focus:ring-blue-500"
            :title="isDarkMode ? 'Beralih ke Light Mode' : 'Beralih ke Dark Mode'"
          >
            <svg v-if="!isDarkMode" xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 text-gray-600" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z" />
            </svg>
            <svg v-else xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 text-yellow-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z" />
            </svg>
          </button>
        </div>
      </div>
    </aside>

    <main class="flex-1 w-full min-h-screen pt-16 md:pt-0 md:ml-64 transition-all duration-300">
      <div class="mx-auto max-w-7xl p-4 lg:p-8">
        <slot />
      </div>
    </main>

  </div>
</template>

<script setup>
import { ref, onMounted, watch } from 'vue'

const isDarkMode = ref(false)
const isSidebarOpen = ref(false)

const toggleTheme = () => {
  isDarkMode.value = !isDarkMode.value
}

watch(isDarkMode, (newVal) => {
  if (import.meta.client) {
    if (newVal) {
      document.documentElement.classList.add('dark')
    } else {
      document.documentElement.classList.remove('dark')
    }
    localStorage.setItem('theme', newVal ? 'dark' : 'light')
  }
}, { immediate: true })

onMounted(() => {
  if (import.meta.client) {
    const savedTheme = localStorage.getItem('theme')
    if (savedTheme === 'dark' || (!savedTheme && window.matchMedia('(prefers-color-scheme: dark)').matches)) {
      isDarkMode.value = true
    } else {
      isDarkMode.value = false
    }
  }
})
</script>