<template>
  <div>
    <h1>Manage Users</h1>
    <ul>
      <li v-for="user in users" :key="user.user_id">
        {{ user.username }} ({{ user.email }})
        <button class="delete-button" @click="deleteUser(user.user_id)">
          Delete
        </button>
        <button class="reset-button" @click="resetPassword(user.email)">
          Reset Password
        </button>
      </li>
    </ul>
  </div>
</template>

<script>
import { ref, onMounted } from "vue";
import { useRouter } from "vue-router";

export default {
  setup() {
    const users = ref([]);
    const router = useRouter();

    const checkAuth = () => {
      const token = localStorage.getItem("token");
      if (!token) {
        router.push("/login");
      }
    };

    const fetchUsers = async () => {
      const response = await fetch("http://localhost:8000/users", {
        headers: {
          Authorization: `Bearer ${localStorage.getItem("token")}`, // Include token in request
        },
      });
      users.value = await response.json();
    };

    const deleteUser = async (userId) => {
      await fetch(`http://localhost:8000/users/${userId}`, {
        method: "DELETE",
        headers: {
          Authorization: `Bearer ${localStorage.getItem("token")}`, // Include token in request
        },
      });
      await fetchUsers();
    };

    const resetPassword = async (email) => {
      const newPassword = prompt("Enter new password:");
      if (newPassword) {
        await fetch("http://localhost:8000/reset_password", {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify({ email, new_password: newPassword }),
        });
        await fetchUsers();
      }
    };

    onMounted(() => {
      checkAuth();
      fetchUsers();
    });

    return { users, deleteUser, resetPassword };
  },
};
</script>
