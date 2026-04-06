<template>
  <div class="space-y-8 animate-in fade-in slide-in-from-bottom-4 duration-500">
    <div class="flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
      <div>
        <h2 class="text-3xl font-bold tracking-tight">Dashboard Onboarding</h2>
        <p class="text-gray-500 dark:text-gray-400 mt-1">Kelola proses masuk kandidat baru dan tinjau dokumen.</p>
      </div>
      <button @click="showInitiateModal = true" class="inline-flex items-center justify-center rounded-xl bg-blue-600 px-5 py-3 text-sm font-semibold text-white shadow-sm hover:bg-blue-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-600 transition-all">
        + Inisiasi Karyawan Baru
      </button>
    </div>

    <div class="overflow-hidden rounded-2xl border border-gray-200 bg-white shadow-sm dark:border-gray-800 dark:bg-gray-800/50 backdrop-blur-sm">
      <div class="overflow-x-auto">
        <table class="min-w-full divide-y divide-gray-200 dark:divide-gray-700">
          <thead class="bg-gray-50/50 dark:bg-gray-900/50">
            <tr>
              <th scope="col" class="px-6 py-4 text-left text-xs font-semibold text-gray-500 uppercase tracking-wider dark:text-gray-400">ID</th>
              <th scope="col" class="px-6 py-4 text-left text-xs font-semibold text-gray-500 uppercase tracking-wider dark:text-gray-400">Kandidat</th>
              <th scope="col" class="px-6 py-4 text-left text-xs font-semibold text-gray-500 uppercase tracking-wider dark:text-gray-400">Departemen</th>
              <th scope="col" class="px-6 py-4 text-left text-xs font-semibold text-gray-500 uppercase tracking-wider dark:text-gray-400">Tanggal Mulai</th>
              <th scope="col" class="px-6 py-4 text-left text-xs font-semibold text-gray-500 uppercase tracking-wider dark:text-gray-400">Status</th>
              <th scope="col" class="px-6 py-4 text-right text-xs font-semibold text-gray-500 uppercase tracking-wider dark:text-gray-400">Aksi</th>
            </tr>
          </thead>
          <tbody class="divide-y divide-gray-200 dark:divide-gray-700">
            <tr v-for="candidate in activeCandidates" :key="candidate.onboardingId" class="hover:bg-gray-50 dark:hover:bg-gray-800/80 transition-colors">
              <td class="whitespace-nowrap px-6 py-4 text-sm font-medium text-gray-900 dark:text-gray-100">{{ candidate.onboardingId }}</td>
              <td class="whitespace-nowrap px-6 py-4 text-sm text-gray-700 dark:text-gray-300">{{ candidate.candidateName }}</td>
              <td class="whitespace-nowrap px-6 py-4 text-sm text-gray-700 dark:text-gray-300">{{ candidate.department }}</td>
              <td class="whitespace-nowrap px-6 py-4 text-sm text-gray-700 dark:text-gray-300">{{ candidate.startDate }}</td>
              <td class="whitespace-nowrap px-6 py-4 text-sm">
                <span :class="getStatusClass(candidate.status)" class="inline-flex items-center rounded-full px-2.5 py-0.5 text-xs font-medium">
                  {{ candidate.status }}
                </span>
              </td>
              <td class="whitespace-nowrap px-6 py-4 text-right text-sm font-medium">
                <button v-if="candidate.status === 'PENDING_REVIEW'" class="text-blue-600 hover:text-blue-900 dark:text-blue-400 dark:hover:text-blue-300">Review Dokumen</button>
                <span v-else class="text-gray-400 dark:text-gray-600">-</span>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>

    <div v-if="showInitiateModal" class="relative z-50" aria-labelledby="modal-title" role="dialog" aria-modal="true">
      <div class="fixed inset-0 bg-gray-900/75 backdrop-blur-sm transition-opacity"></div>
      <div class="fixed inset-0 z-10 w-screen overflow-y-auto">
        <div class="flex min-h-full items-end justify-center p-4 text-center sm:items-center sm:p-0">
          <div class="relative transform overflow-hidden rounded-2xl bg-white text-left shadow-xl transition-all sm:my-8 sm:w-full sm:max-w-lg dark:bg-gray-800 border border-gray-700">
            <div class="px-4 pb-4 pt-5 sm:p-6 sm:pb-4">
              <h3 class="text-xl font-bold leading-6 text-gray-900 dark:text-white" id="modal-title">Inisiasi Karyawan Baru</h3>
              <form @submit.prevent="submitInitiate" class="mt-6 space-y-4">
                <div class="grid grid-cols-2 gap-4">
                  <div>
                    <label class="block text-sm font-medium text-gray-700 dark:text-gray-300">Nama Depan</label>
                    <input v-model="form.firstName" type="text" required class="mt-1 block w-full rounded-lg border border-gray-300 bg-white px-3 py-2 text-gray-900 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white sm:text-sm" />
                  </div>
                  <div>
                    <label class="block text-sm font-medium text-gray-700 dark:text-gray-300">Nama Belakang</label>
                    <input v-model="form.lastName" type="text" required class="mt-1 block w-full rounded-lg border border-gray-300 bg-white px-3 py-2 text-gray-900 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white sm:text-sm" />
                  </div>
                </div>
                <div>
                  <label class="block text-sm font-medium text-gray-700 dark:text-gray-300">Email Personal</label>
                  <input v-model="form.personalEmail" type="email" required class="mt-1 block w-full rounded-lg border border-gray-300 bg-white px-3 py-2 text-gray-900 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white sm:text-sm" />
                </div>
                <div class="grid grid-cols-2 gap-4">
                  <div>
                    <label class="block text-sm font-medium text-gray-700 dark:text-gray-300">Departemen</label>
                    <select v-model="form.department" required class="mt-1 block w-full rounded-lg border border-gray-300 bg-white px-3 py-2 text-gray-900 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white sm:text-sm">
                      <option>Engineering</option>
                      <option>Marketing</option>
                      <option>Finance</option>
                    </select>
                  </div>
                  <div>
                    <label class="block text-sm font-medium text-gray-700 dark:text-gray-300">Tanggal Mulai</label>
                    <input v-model="form.startDate" type="date" required class="mt-1 block w-full rounded-lg border border-gray-300 bg-white px-3 py-2 text-gray-900 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white sm:text-sm" />
                  </div>
                </div>
              </form>
            </div>
            <div class="bg-gray-50 px-4 py-3 sm:flex sm:flex-row-reverse sm:px-6 dark:bg-gray-900/50 border-t dark:border-gray-700">
              <button @click="submitInitiate" type="button" class="inline-flex w-full justify-center rounded-xl bg-blue-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-blue-500 sm:ml-3 sm:w-auto transition-colors">Simpan & Kirim Email</button>
              <button @click="showInitiateModal = false" type="button" class="mt-3 inline-flex w-full justify-center rounded-xl bg-white px-3 py-2 text-sm font-semibold text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 hover:bg-gray-50 sm:mt-0 sm:w-auto dark:bg-gray-800 dark:text-gray-300 dark:ring-gray-600 dark:hover:bg-gray-700 transition-colors">Batal</button>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, reactive } from 'vue'

const showInitiateModal = ref(false)

// Dummy data sementara untuk respons GET /api/v1/onboarding/active
const activeCandidates = ref([
  { onboardingId: 'ob_123', candidateName: 'John Doe', department: 'Engineering', startDate: '2026-05-01', status: 'INITIATED' },
  { onboardingId: 'ob_124', candidateName: 'Jane Smith', department: 'Marketing', startDate: '2026-05-10', status: 'PENDING_REVIEW' },
  { onboardingId: 'ob_125', candidateName: 'Alan Turing', department: 'Engineering', startDate: '2026-06-01', status: 'IT_PROVISIONING' }
])

// Struktur payload POST /initiate
const form = reactive({
  firstName: '',
  lastName: '',
  personalEmail: '',
  department: 'Engineering',
  startDate: ''
})

const getStatusClass = (status) => {
  const map = {
    'INITIATED': 'bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-300',
    'PENDING_REVIEW': 'bg-yellow-100 text-yellow-800 dark:bg-yellow-900/30 dark:text-yellow-400',
    'IT_PROVISIONING': 'bg-blue-100 text-blue-800 dark:bg-blue-900/30 dark:text-blue-400',
    'COMPLETED': 'bg-green-100 text-green-800 dark:bg-green-900/30 dark:text-green-400'
  }
  return map[status] || 'bg-gray-100 text-gray-800'
}

const submitInitiate = () => {
  console.log('Submitting payload:', form)
  // Mockup update UI
  activeCandidates.value.unshift({
    onboardingId: `ob_${Math.floor(Math.random() * 1000)}`,
    candidateName: `${form.firstName} ${form.lastName}`,
    department: form.department,
    startDate: form.startDate,
    status: 'INITIATED'
  })
  showInitiateModal.value = false
  // Reset form
  form.firstName = ''
  form.lastName = ''
  form.personalEmail = ''
  form.startDate = ''
}
</script>