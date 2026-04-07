<template>
  <div>
    <div class="mb-8 flex justify-between items-center">
      <div>
        <h1 class="text-2xl font-bold text-gray-900 dark:text-white">HR Dashboard</h1>
        <p class="text-gray-500 dark:text-gray-400 text-sm mt-1">Kelola data onboarding kandidat Oryphem.</p>
      </div>
      <button @click="showInitiateModal = true" class="bg-blue-600 hover:bg-blue-500 text-white px-4 py-2 rounded-lg text-sm font-semibold transition-colors">
        + Initiate Onboarding
      </button>
    </div>

    <div class="bg-white dark:bg-gray-800 rounded-2xl shadow-sm border border-gray-200 dark:border-gray-700 overflow-hidden">
      <div v-if="pending" class="p-8 text-center text-gray-500">
        Memuat data...
      </div>
      <div v-else-if="error" class="p-8 text-center text-red-500">
        Gagal memuat data: {{ error.message }}
      </div>
      <table v-else class="w-full text-left text-sm">
        <thead class="bg-gray-50 dark:bg-gray-900/50 text-gray-600 dark:text-gray-400">
          <tr>
            <th class="px-6 py-4 font-medium">ID Onboarding</th>
            <th class="px-6 py-4 font-medium">Nama Kandidat</th>
            <th class="px-6 py-4 font-medium">Departemen</th>
            <th class="px-6 py-4 font-medium">Start Date</th>
            <th class="px-6 py-4 font-medium">Status</th>
            <th class="px-6 py-4 font-medium">Aksi</th>
          </tr>
        </thead>
        <tbody class="divide-y divide-gray-200 dark:divide-gray-700">
          <tr v-for="item in activeOnboardings?.data" :key="item.onboardingId" class="hover:bg-gray-50 dark:hover:bg-gray-700/50 transition-colors">
            <td class="px-6 py-4 font-mono text-xs text-gray-500">{{ item.onboardingId }}</td>
            <td class="px-6 py-4 font-medium text-gray-900 dark:text-white">{{ item.candidateName }}</td>
            <td class="px-6 py-4 text-gray-600 dark:text-gray-300">{{ item.department }}</td>
            <td class="px-6 py-4 text-gray-600 dark:text-gray-300">{{ item.startDate }}</td>
            <td class="px-6 py-4">
              <span class="inline-flex items-center rounded-full px-2.5 py-0.5 text-xs font-medium"
                :class="{
                  'bg-yellow-100 text-yellow-800 dark:bg-yellow-900/30 dark:text-yellow-400': item.status === 'INITIATED',
                  'bg-blue-100 text-blue-800 dark:bg-blue-900/30 dark:text-blue-400': item.status === 'PENDING_REVIEW',
                  'bg-green-100 text-green-800 dark:bg-green-900/30 dark:text-green-400': item.status === 'COMPLETED',
                }">
                {{ item.status.replace('_', ' ') }}
              </span>
            </td>
            <td class="px-6 py-4">
              <NuxtLink :to="`/candidate/${item.onboardingId}`" class="text-blue-600 hover:text-blue-800 dark:text-blue-400 dark:hover:text-blue-300 font-medium text-xs">
                Buka Portal
              </NuxtLink>
            </td>
          </tr>
          <tr v-if="!activeOnboardings?.data?.length">
            <td colspan="6" class="px-6 py-8 text-center text-gray-500 dark:text-gray-400">
              Tidak ada data onboarding aktif.
            </td>
          </tr>
        </tbody>
      </table>
    </div>

    <div v-if="showInitiateModal" class="fixed inset-0 z-50 bg-gray-900/50 backdrop-blur-sm flex items-center justify-center p-4">
      <div class="bg-white dark:bg-gray-800 rounded-2xl w-full max-w-md p-6 shadow-xl border border-gray-200 dark:border-gray-700">
        <h3 class="text-lg font-bold mb-4">Initiate Onboarding Baru</h3>
        <form @submit.prevent="submitInitiate" class="space-y-4">
          <div class="grid grid-cols-2 gap-4">
            <div>
              <label class="block text-xs font-medium text-gray-700 dark:text-gray-300 mb-1">First Name</label>
              <input v-model="form.firstName" type="text" required class="w-full rounded-lg border border-gray-300 dark:border-gray-600 bg-transparent px-3 py-2 text-sm focus:ring-2 focus:ring-blue-500 outline-none" />
            </div>
            <div>
              <label class="block text-xs font-medium text-gray-700 dark:text-gray-300 mb-1">Last Name</label>
              <input v-model="form.lastName" type="text" required class="w-full rounded-lg border border-gray-300 dark:border-gray-600 bg-transparent px-3 py-2 text-sm focus:ring-2 focus:ring-blue-500 outline-none" />
            </div>
          </div>
          <div>
            <label class="block text-xs font-medium text-gray-700 dark:text-gray-300 mb-1">Personal Email</label>
            <input v-model="form.personalEmail" type="email" required class="w-full rounded-lg border border-gray-300 dark:border-gray-600 bg-transparent px-3 py-2 text-sm focus:ring-2 focus:ring-blue-500 outline-none" />
          </div>
          <div>
            <label class="block text-xs font-medium text-gray-700 dark:text-gray-300 mb-1">Department</label>
            <input v-model="form.department" type="text" required class="w-full rounded-lg border border-gray-300 dark:border-gray-600 bg-transparent px-3 py-2 text-sm focus:ring-2 focus:ring-blue-500 outline-none" />
          </div>
          <div>
            <label class="block text-xs font-medium text-gray-700 dark:text-gray-300 mb-1">Start Date</label>
            <input v-model="form.startDate" type="date" required class="w-full rounded-lg border border-gray-300 dark:border-gray-600 bg-transparent px-3 py-2 text-sm focus:ring-2 focus:ring-blue-500 outline-none" />
          </div>
          
          <div class="flex justify-end gap-3 pt-4">
            <button type="button" @click="showInitiateModal = false" class="px-4 py-2 text-sm font-medium text-gray-600 hover:text-gray-900 dark:text-gray-400 dark:hover:text-white">Batal</button>
            <button type="submit" :disabled="isSubmitting" class="bg-blue-600 hover:bg-blue-500 text-white px-4 py-2 rounded-lg text-sm font-semibold disabled:opacity-50">
              {{ isSubmitting ? 'Memproses...' : 'Simpan' }}
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'

const config = useRuntimeConfig()
const showInitiateModal = ref(false)
const isSubmitting = ref(false)

// Fetch Data Active Onboarding
const { data: activeOnboardings, pending, error, refresh } = useFetch(() => '/api/v1/onboarding/active', {
  baseURL: config.public.apiBase, 
  server: false,
  immediate: true,
  watch: false 
})

const form = ref({
  firstName: '',
  lastName: '',
  personalEmail: '',
  department: '',
  startDate: ''
})

const submitInitiate = async () => {
  isSubmitting.value = true
  try {
    await $fetch('/api/v1/onboarding/initiate', {
      baseURL: config.public.apiBase,
      method: 'POST',
      body: form.value
    })
    
    // Reset & Close Modal
    form.value = { firstName: '', lastName: '', personalEmail: '', department: '', startDate: '' }
    showInitiateModal.value = false
    
    // Refresh Data Table
    await refresh()
    alert('Onboarding berhasil diinisiasi!')
  } catch (err) {
    alert('Gagal inisiasi: ' + (err.data?.error || err.message))
  } finally {
    isSubmitting.value = false
  }
}
</script>