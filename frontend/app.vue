<template>
  <div class="container">
    <nav>
      <NuxtLink to="/">Home</NuxtLink>
      <NuxtLink to="/manageUsers">Manage Users</NuxtLink>
      <NuxtLink to="/createEvent">Create Event</NuxtLink>
      <NuxtLink to="/login">Login</NuxtLink>
      <NuxtLink to="/logout">Logout</NuxtLink>
    </nav>
    <NuxtPage />
  </div>
</template>

<script setup>
import { useHead } from "#app";
useHead({
  title: "OpenMeet",
  meta: [
    {
      name: "description",
      content: "Organize local meetups easily while keeping your data private.",
    },
  ],
  link: [
    {
      rel: "stylesheet",
      href: "https://fonts.googleapis.com/css2?family=Roboto:wght@400;700&display=swap",
    },
  ],
});
</script>
