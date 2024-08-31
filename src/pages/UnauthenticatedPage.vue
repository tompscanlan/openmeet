<script setup lang="ts">
import UsersList from '../components/UsersList.vue';

const emit = defineEmits(['navigateToLogin']);

const handleNavigateToLogin = () => {
  emit('navigateToLogin');
};
</script>

<template>
  <div class="unauthenticated-page">
    <h2>Welcome to OpenMeet</h2>
    <p>Please log in to access the application.</p>
    <a href="#/login">Go to Login</a>
    
    <component :is="UsersList" />
  </div>
</template>

<style scoped>
.unauthenticated-page {
  display: flex;
  flex-direction: column;
  align-items: center;
}
</style>