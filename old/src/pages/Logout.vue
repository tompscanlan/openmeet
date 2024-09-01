<script setup lang="ts">
import { onMounted } from 'vue';

const emit = defineEmits(['logout', 'navigateToLogin']);

onMounted(() => {
  // Perform any necessary logout actions
  emit('logout');
  
  // Automatically redirect to login page after a short delay
  setTimeout(() => {
    emit('navigateToLogin');
  }, 2000);
});
</script>

<template>
  <div class="logout-page">
    <h2>Logging Out</h2>
    <p>You have been successfully logged out.</p>
    <p>Redirecting to login page...</p>
  </div>
</template>

<style scoped>
.logout-page {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100vh;
  text-align: center;
}
</style>