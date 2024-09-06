<template>
  <div class="container">
    <p>Thanks for checking in! See ya next time!</p>
    <LoginComponent />
  </div>
</template>

<script setup>
import LoginComponent from "../components/LoginComponent.vue";
import { ref, onMounted } from "vue";
import { useRouter } from "vue-router";

const router = useRouter();
onMounted(() => {
  localStorage.removeItem("token");
  localStorage.removeItem("user");
  router.push("/");
});
</script>
