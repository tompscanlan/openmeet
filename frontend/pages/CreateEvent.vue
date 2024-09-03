<template>
  <div class="container">
    <h1>Create Event</h1>
    <form @submit.prevent="handleCreateEvent">
      <input v-model="title" type="text" placeholder="Event Title" required />
      <textarea v-model="description" placeholder="Event Description" required></textarea>
      <input v-model="startTime" type="datetime-local" required />
      <input v-model="endTime" type="datetime-local" required />
      <input v-model="lat" type="number" step="0.000001" placeholder="Latitude" required />
      <input v-model="lon" type="number" step="0.000001" placeholder="Longitude" required />
      <input v-model="address" type="text" placeholder="Address" required />
      <button type="submit">Create Event</button>
    </form>
    <p v-if="error" class="error">{{ error }}</p>
  </div>
</template>

<script setup>
import { ref } from 'vue';
import { useRouter } from 'vue-router';

const router = useRouter();

const title = ref('');
const description = ref('');
const startTime = ref('');
const endTime = ref('');
const lat = ref('');
const lon = ref('');
const address = ref('');
const error = ref('');
const my_user_id = ref('');

const getUser = async () => {
  const user = localStorage.getItem('user');
  if (user) {
    const parsedUser = JSON.parse(user);
    my_user_id.value = parsedUser.user_id;
    return parsedUser;
  }
  throw new Error('User not found');
};

const handleCreateEvent = async () => {
  try {
    const token = localStorage.getItem('token');
    if (!token) {
      router.push('/login');
      return;
    }

    const user = await getUser();

    const event = JSON.stringify({
        title: title.value,
        description: description.value,
        start_time: new Date(startTime.value).toISOString(),
        end_time: new Date(endTime.value).toISOString(),
        lat: parseFloat(lat.value),
        lon: parseFloat(lon.value),
        address: address.value,
        creator_id: user.user_id
      })

    console.log("posting event",event)
    const response = await fetch('http://localhost:8000/events', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${token}`,
      },
      body: event
    });

    if (!response.ok) {
      throw new Error('Failed to create event');
    }

    router.push('/'); // Redirect to home page or events list
  } catch (err) {
    console.error('Event creation failed', err);
    error.value = err.message;
  }
};
</script>