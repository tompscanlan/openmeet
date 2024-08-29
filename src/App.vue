<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/tauri';

const title = ref('');
const description = ref('');
const date = ref('');
const location = ref('');
const events = ref([]);
const errorMessage = ref('');

const createKeyspace = async () => {
  try {
    await invoke('create_keyspace', { keyspaceName: 'events' });
    errorMessage.value = '';
  } catch (error: unknown) {
    errorMessage.value = (error as Error).toString();
  }
};

const createEvent = async () => {
  try {
    const id: string = Date.now().toString();
    await invoke('create_event', { id: id, title: title.value, description: description.value, date: date.value, location: location.value });
    loadEvents();
    errorMessage.value = '';
  } catch (error) {
    errorMessage.value = error.toString();  }
};

const loadEvents = async () => {
  try {
    const result = await invoke('list_events');
    console.log('Raw result:', result);
    if (Array.isArray(result)) {
      events.value = result;
      console.log('Loaded events:', events.value);
    } else {
      throw new Error('Expected an array of events');
    }
  } catch (error) {
    console.error('Error loading events:', error);
    errorMessage.value = `Error loading events: ${error}`;
    events.value = [];
  }
};

onMounted(async () => {
  await loadEvents();
});
</script>

<template>
  <div class="container">
    <h1>OpenMeet</h1>

    <form @submit.prevent="createEvent">
      <input v-model="title" type="text" placeholder="Title" required />
      <input v-model="description" type="text" placeholder="Description" required />
      <input v-model="date" type="date" required />
      <input v-model="location" type="text" placeholder="Location" required />
      <button type="submit">Create Event</button>
    </form>

    <button @click="createKeyspace">Create Keyspace</button>

    <div v-if="errorMessage" class="error-message">
         {{ errorMessage }}
       </div>

       <div v-if="events.length === 0">No events found.</div>
<ul v-else>
  <li v-for="event in events" :key="event.id">
    <strong>{{ event.title }}</strong> - {{ event.date }} at {{ event.location }}
    <p>{{ event.description }}</p>
  </li>
</ul>
  </div>
</template>
   <style scoped>
   .container {
     margin: 0;
     padding-top: 10vh;
     display: flex;
     flex-direction: column;
     justify-content: center;
     text-align: center;
   }

   form {
     display: flex;
     flex-direction: column;
     gap: 1em;
     margin-bottom: 2em;
   }

   input,
   button {
     border-radius: 8px;
     border: 1px solid transparent;
     padding: 0.6em 1.2em;
     font-size: 1em;
     font-weight: 500;
     font-family: inherit;
     color: #0f0f0f;
     background-color: #ffffff;
     transition: border-color 0.25s;
     box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
   }

   button {
     cursor: pointer;
   }

   button:hover {
     border-color: #396cd8;
   }

   button:active {
     border-color: #396cd8;
     background-color: #e8e8e8;
   }

   input,
   button {
     outline: none;
   }

   .error-message {
     color: red;
     margin-bottom: 1em;
   }
   </style>