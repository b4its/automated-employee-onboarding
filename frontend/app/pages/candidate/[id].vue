<template>
  <div class="mx-auto max-w-2xl animate-in zoom-in-95 duration-500">
    <div class="overflow-hidden rounded-3xl bg-white shadow-xl ring-1 ring-gray-200 dark:bg-gray-800 dark:ring-gray-700">
      <div class="p-8 sm:p-12">
        <div class="text-center">
          <div class="mx-auto flex h-16 w-16 items-center justify-center rounded-full bg-blue-100 dark:bg-blue-900/30 mb-6">
            <svg class="h-8 w-8 text-blue-600 dark:text-blue-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
            </svg>
          </div>
          <h2 class="text-2xl font-bold text-gray-900 dark:text-white">Selamat Datang di Oryphem!</h2>
          <p class="mt-2 text-gray-500 dark:text-gray-400">Silakan unggah dokumen identitas Anda untuk melanjutkan proses onboarding ke tahap review.</p>
        </div>

        <form @submit.prevent="uploadDocument" class="mt-10 space-y-6">
          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300">Tipe Dokumen</label>
            <select v-model="documentType" class="mt-2 block w-full rounded-xl border border-gray-300 bg-white px-4 py-3 text-gray-900 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-2 focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white transition-shadow">
              <option value="IDENTIFICATION">KTP / Paspor</option>
              <option value="CONTRACT">Kontrak Kerja</option>
              <option value="MEDICAL">Surat Kesehatan</option>
            </select>
          </div>

          <div 
            class="group relative mt-2 flex justify-center rounded-2xl border-2 border-dashed border-gray-300 px-6 py-10 transition-colors hover:border-blue-500 hover:bg-blue-50 dark:border-gray-600 dark:hover:border-blue-400 dark:hover:bg-blue-900/10"
            :class="{ 'border-blue-500 bg-blue-50 dark:border-blue-400 dark:bg-blue-900/10': isDragging }"
            @dragover.prevent="isDragging = true"
            @dragleave.prevent="isDragging = false"
            @drop.prevent="handleDrop"
          >
            <div class="text-center">
              <svg class="mx-auto h-12 w-12 text-gray-300 group-hover:text-blue-500 dark:text-gray-500 dark:group-hover:text-blue-400 transition-colors" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
                <path fill-rule="evenodd" d="M1.5 6a2.25 2.25 0 012.25-2.25h16.5A2.25 2.25 0 0122.5 6v12a2.25 2.25 0 01-2.25 2.25H3.75A2.25 2.25 0 011.5 18V6zM3 16.06V18c0 .414.336.75.75.75h16.5A.75.75 0 0021 18v-1.94l-2.69-2.689a1.5 1.5 0 00-2.12 0l-.88.879.97.97a.75.75 0 11-1.06 1.06l-5.16-5.159a1.5 1.5 0 00-2.12 0L3 16.061zm10.125-7.81a1.125 1.125 0 112.25 0 1.125 1.125 0 01-2.25 0z" clip-rule="evenodd" />
              </svg>
              <div class="mt-4 flex text-sm leading-6 text-gray-600 dark:text-gray-400 justify-center">
                <label for="file-upload" class="relative cursor-pointer rounded-md bg-transparent font-semibold text-blue-600 focus-within:outline-none focus-within:ring-2 focus-within:ring-blue-600 focus-within:ring-offset-2 hover:text-blue-500 dark:text-blue-400 dark:hover:text-blue-300">
                  <span>Pilih file</span>
                  <input id="file-upload" name="file-upload" type="file" class="sr-only" @change="handleFileSelect" accept="application/pdf,image/jpeg,image/png" />
                </label>
                <p class="pl-1">atau seret dan lepas ke sini</p>
              </div>
              <p class="text-xs leading-5 text-gray-500 dark:text-gray-500">PDF, PNG, JPG hingga 10MB</p>
              
              <div v-if="selectedFile" class="mt-4 inline-flex items-center gap-2 rounded-full bg-green-100 px-4 py-1.5 text-sm font-medium text-green-800 dark:bg-green-900/30 dark:text-green-400">
                <svg class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/></svg>
                {{ selectedFile.name }}
              </div>
            </div>
          </div>

          <div class="pt-4">
            <button type="submit" :disabled="!selectedFile" class="flex w-full justify-center rounded-xl bg-blue-600 px-3 py-4 text-sm font-semibold text-white shadow-sm hover:bg-blue-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-600 disabled:opacity-50 disabled:cursor-not-allowed transition-all">
              Unggah Dokumen Aman
            </button>
          </div>
        </form>
      </div>
      <div class="bg-gray-50 px-8 py-6 border-t border-gray-200 dark:bg-gray-900/50 dark:border-gray-700">
        <p class="text-center text-xs text-gray-500 dark:text-gray-400">
          Dilindungi oleh enkripsi AES-256. Sesuai dengan regulasi GDPR & CCPA.
        </p>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'
import { useRoute } from 'nuxt/app'

const route = useRoute()
const onboardingId = route.params.id || 'ob_123' // Fallback for demo

const documentType = ref('IDENTIFICATION')
const selectedFile = ref(null)
const isDragging = ref(false)

const handleFileSelect = (event) => {
  const file = event.target.files[0]
  if (file && file.size <= 10 * 1024 * 1024) { // Validasi 10MB
    selectedFile.value = file
  } else {
    alert("Ukuran file maksimal adalah 10MB.")
  }
}

const handleDrop = (event) => {
  isDragging.value = false
  const file = event.dataTransfer.files[0]
  if (file && file.size <= 10 * 1024 * 1024) {
    selectedFile.value = file
  }
}

const uploadDocument = async () => {
  if (!selectedFile.value) return
  
  // Simulasi struktur FormData untuk POST /documents
  const formData = new FormData()
  formData.append('file', selectedFile.value)
  formData.append('documentType', documentType.value)
  
  console.log(`Mengunggah ke /api/v1/onboarding/${onboardingId}/documents`, formData)
  alert('Simulasi: Dokumen berhasil diunggah dengan status PENDING_REVIEW')
  selectedFile.value = null
}
</script>