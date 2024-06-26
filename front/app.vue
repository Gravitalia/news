<script setup lang="ts">
const error = ref(false);
function callback() {
  error.value = true;
}
</script>

<template>
  <NuxtLayout>
    <!-- Server error. -->
    <Banner
      v-if="error"
      :content="$t('error.internal_server_error')"
      :can-close="false"
    />

    <NavBar />
    <NuxtPage @show-error="callback" />
    <Footer />
  </NuxtLayout>
</template>
