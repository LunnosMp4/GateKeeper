<template>
  <div>
    <h1>Admin Panel - User Management</h1>
    <router-link to="/">Home</router-link><br><br>
    <button @click="fetchUsers">Refresh Users</button>
    <p v-if="loading">Loading...</p>
    <p v-if="error">{{ error }}</p>

    <input
      type="text"
      v-model="searchQuery"
      placeholder="Search by name, email, or API key"
    />
    <br /><br />

    <table v-if="filteredUsers.length">
      <thead>
        <tr>
          <th>ID</th>
          <th>Name</th>
          <th>Email</th>
          <th>API Key</th>
          <th>Permission</th>
          <th>Actions</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="targetUser in filteredUsers" :key="targetUser.id">
          <td>{{ targetUser.id }}</td>
          <td>{{ targetUser.name }}</td>
          <td>{{ targetUser.email }}</td>
          <td v-if="targetUser.api_key">{{ targetUser.api_key }}</td>
          <td v-else style="color: red;">API Key Revoked</td>
          <td>
            <select
              v-model="targetUser.permission"
              :disabled="targetUser.email === user.email"
              @change="changePermission(targetUser.id, targetUser.permission)"
            >
              <option value="0">User</option>
              <option value="1">Admin</option>
            </select>
          </td>
          <td>
            <button
              v-if="targetUser.api_key"
              :disabled="targetUser.email === user.email"
              @click="revokeApiKey(targetUser.id)"
            >
              Revoke API Key
            </button>
            <button
              v-else
              :disabled="targetUser.email === user.email"
              @click="createApiKey(targetUser.id)"
            >
              Create API Key
            </button>
            <button
              :disabled="targetUser.email === user.email"
              @click="deleteUser(targetUser.id)"
            >
              Delete User
            </button>
          </td>
        </tr>
      </tbody>
    </table>
    <p v-if="!filteredUsers.length && !loading">No users found.</p>
  </div>
</template>


<script>
export default {
  data() {
    return {
      users: [],
      searchQuery: "",
      loading: false,
      error: null,
    };
  },
  computed: {
    user() {
      return this.$route.meta.user;
    },
    filteredUsers() {
      const query = this.searchQuery.toLowerCase();
      return this.users.filter(
        (user) =>
          user.name.toLowerCase().includes(query) ||
          user.email.toLowerCase().includes(query) ||
          user.api_key.toLowerCase().includes(query)
      );
    },
  },
  methods: {
    fetchUsers() {
      this.loading = true;
      this.error = null;
      const authToken = localStorage.getItem("authToken");
      fetch("http://localhost:8080/dashboard/admin/users", {
        method: "GET",
        headers: {
          Authorization: `${authToken}`,
        },
      })
        .then((response) => {
          if (!response.ok) {
            throw new Error("Failed to fetch users");
          }
          return response.json();
        })
        .then((data) => {
          this.users = data;
        })
        .catch((error) => {
          this.error = error.message;
        })
        .finally(() => {
          this.loading = false;
        });
    },
    changePermission(userId, permission) {
      const authToken = localStorage.getItem("authToken");
      fetch(`http://localhost:8080/dashboard/admin/users/${userId}/${permission}`, {
        method: "POST",
        headers: {
          Authorization: `${authToken}`,
        },
      })
        .then((response) => {
          if (!response.ok) {
            throw new Error("Failed to change user permission");
          }
          alert("Permission updated successfully");
        })
        .catch((error) => {
          alert(error.message);
        });
    },
    revokeApiKey(userId) {
      const authToken = localStorage.getItem("authToken");
      fetch(`http://localhost:8080/dashboard/admin/users/${userId}/revoke`, {
        method: "POST",
        headers: {
          Authorization: `${authToken}`,
        },
      })
        .then((response) => {
          if (!response.ok) {
            throw new Error("Failed to revoke API key");
          }
          alert("API key revoked successfully");
          this.fetchUsers();
        })
        .catch((error) => {
          alert(error.message);
        });
    },
    createApiKey(userId) {
      const authToken = localStorage.getItem("authToken");
      fetch(`http://localhost:8080/dashboard/admin/users/${userId}/create_api_key`, {
        method: "POST",
        headers: {
          Authorization: `${authToken}`,
        },
      })
        .then((response) => {
          if (!response.ok) {
            throw new Error("Failed to create API key");
          }
          alert("API key created successfully");
          this.fetchUsers();
        })
        .catch((error) => {
          alert(error.message);
        });
    },
    deleteUser(userId) {
      const authToken = localStorage.getItem("authToken");
      fetch(`http://localhost:8080/dashboard/admin/users/${userId}`, {
        method: "DELETE",
        headers: {
          Authorization: `${authToken}`,
        },
      })
        .then((response) => {
          if (!response.ok) {
            throw new Error("Failed to delete user");
          }
          alert("User deleted successfully");
          this.fetchUsers();
        })
        .catch((error) => {
          alert(error.message);
        });
    },
  },
  created() {
    this.fetchUsers();
  },
};
</script>


<style scoped>
input {
  padding: 8px;
  width: 300px;
  margin-bottom: 10px;
}
table {
  border-collapse: collapse;
  width: 100%;
  margin-top: 10px;
}
table th,
table td {
  border: 1px solid #ddd;
  padding: 8px;
  text-align: left;
}
table th {
  background-color: #f4f4f4;
}
button {
  margin: 0 5px;
  padding: 5px 10px;
  cursor: pointer;
}

button:disabled {
  background-color: #ccc;
  cursor: not-allowed;
}

select {
  padding: 5px;
}
</style>
