<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { api } from './api/client'

const status = ref('Checking…')
onMounted(async () => {
  try {
    const { data, error } = await api.GET('/api/health')
    status.value = error || !data ? 'Unavailable' : data.status
  } catch {
    status.value = 'Unavailable'
  }
})
</script>

<template>
  <main>
    <h1>Yumstash</h1>
    <p>Your Vue app and Rust API are ready to build on.</p>
    <p>API status: <strong>{{ status }}</strong></p>
  </main>
</template>

<style>
body { margin: 0; font-family: system-ui, sans-serif; color: #26332b; background: #f7f9f5; }
main { max-width: 48rem; margin: 5rem auto; padding: 0 1.5rem; }
h1 { font-size: 2.5rem; }
</style>
