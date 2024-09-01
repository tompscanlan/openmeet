<script setup lang="ts">
const emit = defineEmits(['logout']);

const handleLogout = () => {
  emit('logout');
};
</script>

<template>
  <div class="authenticated-page">
    <h2>Welcome, User!</h2>
    <p>This is the authenticated page. You can add your main app functionality here.</p>
    <button @click="handleLogout">Logout</button>
  </div>
</template>

<style scoped>
.authenticated-page {
  display: flex;
  flex-direction: column;
  align-items: center;
}
</style>